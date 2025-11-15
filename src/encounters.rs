use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EncounterArea {
    pub id: String,
    pub name: String,
    pub source: Option<String>,
    pub notes: Option<String>,
    pub sections: Vec<EncounterSection>,
}

#[derive(Debug)]
pub struct EncounterSection {
    pub name: String,
    pub method: String,
    pub table: Vec<EncounterEntry>,
}

#[derive(Debug, Clone)]
pub struct EncounterEntry {
    pub pokemon: String,
    pub levels: Option<String>,
    pub rate: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EncounterManifest {
    locations: Vec<RawLocation>,
    meta: Option<ManifestMeta>,
}

#[derive(Debug, Deserialize)]
struct ManifestMeta {
    source: Option<String>,
    generated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawLocation {
    name: String,
    #[serde(default)]
    notes: Option<String>,
    methods: BTreeMap<String, Vec<RawEntry>>,
}

#[derive(Debug, Deserialize)]
struct RawEntry {
    #[serde(alias = "species")]
    pokemon: String,
    #[serde(default)]
    levels: Option<String>,
    #[serde(default)]
    rate: Option<f32>,
    #[serde(default)]
    notes: Option<String>,
}

impl EncounterArea {
    pub fn from_manifest(path: impl Into<PathBuf>, id_or_name: &str) -> Result<Self> {
        let path = path.into();
        let file = fs::File::open(&path)
            .with_context(|| format!("opening encounter manifest {}", path.display()))?;
        let manifest: EncounterManifest = serde_json::from_reader(file)
            .with_context(|| format!("decoding encounter manifest {}", path.display()))?;

        let query_slug = slugify(id_or_name);
        let (loc, meta_source) = manifest
            .locations
            .into_iter()
            .find(|loc| {
                slugify(&loc.name) == query_slug || loc.name.eq_ignore_ascii_case(id_or_name)
            })
            .map(|loc| {
                let source = manifest
                    .meta
                    .as_ref()
                    .and_then(|m| m.source.clone())
                    .unwrap_or_else(|| {
                        "Pokemon Lazarus Documentation - Encounters.pdf".to_string()
                    });
                let generated = manifest.meta.as_ref().and_then(|m| m.generated_at.clone());
                let label = generated
                    .map(|stamp| format!("{source} (generated {stamp})", source = source))
                    .unwrap_or(source);
                (loc, Some(label))
            })
            .ok_or_else(|| anyhow!("no encounter data found for '{id_or_name}'"))?;

        Ok(build_area(loc, meta_source))
    }

    pub fn render_markdown(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("<!-- area-id: {} -->\n", self.id));
        buf.push_str(&format!("### {}\n\n", self.name));
        if let Some(source) = &self.source {
            buf.push_str(&format!("_Source: {}_\n\n", source));
        }
        if let Some(notes) = &self.notes {
            if !notes.trim().is_empty() {
                buf.push_str(notes);
                buf.push_str("\n\n");
            }
        }

        let columns = self.collect_columns();
        let species_map = self.collect_species_map();

        if columns.is_empty() || species_map.is_empty() {
            buf.push_str("_No encounters recorded._\n");
            return buf;
        }

        buf.push_str("| Pokémon |");
        for col in &columns {
            buf.push(' ');
            buf.push_str(&col.label);
            buf.push_str(" |");
        }
        buf.push('\n');
        buf.push_str("| --- |");
        for _ in &columns {
            buf.push_str(" --- |");
        }
        buf.push('\n');

        for (species, methods) in species_map {
            buf.push_str(&format!("| {} |", species));
            for col in &columns {
                let cell = methods
                    .get(&col.slug)
                    .map(|entry| {
                        entry
                            .rate
                            .clone()
                            .or_else(|| {
                                entry.levels.as_ref().map(|levels| format!("Lv {}", levels))
                            })
                            .or_else(|| entry.notes.clone())
                            .unwrap_or_else(|| "✓".to_string())
                    })
                    .unwrap_or_else(|| "—".to_string());
                buf.push_str(&format!(" {} |", cell));
            }
            buf.push('\n');
        }
        buf.push('\n');
        buf
    }

    fn collect_species_map(&self) -> BTreeMap<String, BTreeMap<String, EncounterEntry>> {
        let mut map: BTreeMap<String, BTreeMap<String, EncounterEntry>> = BTreeMap::new();
        for section in &self.sections {
            for entry in &section.table {
                map.entry(entry.pokemon.clone())
                    .or_default()
                    .insert(section.method.clone(), entry.clone());
            }
        }
        map
    }

    fn collect_columns(&self) -> Vec<TableColumn> {
        let mut columns = Vec::new();
        let mut seen = BTreeSet::new();
        for (slug, label) in METHOD_ORDER.iter() {
            if self.sections.iter().any(|sec| sec.method == *slug) {
                columns.push(TableColumn {
                    slug: slug.to_string(),
                    label: (*label).to_string(),
                });
                seen.insert(slug.to_string());
            }
        }
        for section in &self.sections {
            if seen.contains(&section.method) {
                continue;
            }
            seen.insert(section.method.clone());
            columns.push(TableColumn {
                slug: section.method.clone(),
                label: section.name.clone(),
            });
        }
        columns
    }
}

fn build_area(raw: RawLocation, source: Option<String>) -> EncounterArea {
    let id = slugify(&raw.name);
    let mut sections = Vec::new();
    let mut seen = BTreeSet::new();

    for (method, label) in METHOD_ORDER.iter() {
        if let Some(entries) = raw.methods.get(*method) {
            let table = build_entries(entries);
            if table.is_empty() {
                continue;
            }
            sections.push(EncounterSection {
                name: (*label).to_string(),
                method: method.to_string(),
                table,
            });
            seen.insert(method.to_string());
        }
    }

    for (method, entries) in raw.methods.iter() {
        if seen.contains(method) {
            continue;
        }
        let table = build_entries(entries);
        if table.is_empty() {
            continue;
        }
        seen.insert(method.clone());
        sections.push(EncounterSection {
            name: method_label(method),
            method: method.clone(),
            table,
        });
    }

    EncounterArea {
        id,
        name: raw.name,
        source,
        notes: raw.notes,
        sections,
    }
}

fn build_entries(entries: &[RawEntry]) -> Vec<EncounterEntry> {
    entries
        .iter()
        .map(|entry| EncounterEntry {
            pokemon: entry.pokemon.clone(),
            levels: entry.levels.clone(),
            rate: entry.rate.map(format_rate),
            notes: entry.notes.clone(),
        })
        .collect()
}

#[derive(Debug)]
struct TableColumn {
    slug: String,
    label: String,
}

const METHOD_ORDER: [(&str, &str); 9] = [
    ("grass_day", "Grass (Day)"),
    ("grass_night", "Grass (Night)"),
    ("fishing", "Fishing"),
    ("old_rod", "Old Rod"),
    ("good_rod", "Good Rod"),
    ("super_rod", "Super Rod"),
    ("surf", "Surfing"),
    ("underwater", "Underwater"),
    ("special", "Special Encounters"),
];

fn method_label(method: &str) -> String {
    METHOD_ORDER
        .iter()
        .find(|(slug, _)| *slug == method)
        .map(|(_, label)| label.to_string())
        .unwrap_or_else(|| method.replace('_', " ").to_title_case())
}

fn format_rate(rate: f32) -> String {
    if (rate - rate.round()).abs() < f32::EPSILON {
        format!("{:.0}%", rate)
    } else {
        format!("{:.1}%", rate)
    }
}

fn slugify(input: &str) -> String {
    let mut slug = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if ch.is_whitespace() || "-_/()".contains(ch) {
            if !slug.ends_with('-') {
                slug.push('-');
            }
        }
    }
    slug.trim_matches('-').to_string()
}

trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for String {
    fn to_title_case(&self) -> String {
        self.as_str().to_title_case()
    }
}

impl ToTitleCase for &str {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize = true;
        for ch in self.chars() {
            if ch.is_whitespace() || ch == '_' || ch == '-' {
                result.push(' ');
                capitalize = true;
            } else if capitalize {
                result.push(ch.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(ch.to_ascii_lowercase());
            }
        }
        result
    }
}
