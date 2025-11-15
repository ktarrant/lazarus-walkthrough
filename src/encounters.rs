use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::collections::BTreeMap;
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

#[derive(Debug)]
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
            if !notes.is_empty() {
                buf.push_str(&format!("{notes}\n\n"));
            }
        }
        for section in &self.sections {
            buf.push_str(&format!("#### {} (`{}`)\n\n", section.name, section.method));
            let show_levels = section.table.iter().any(|e| e.levels.is_some());
            let show_rate = section.table.iter().any(|e| e.rate.is_some());
            let show_notes = section.table.iter().any(|e| {
                e.notes
                    .as_ref()
                    .map(|n| !n.trim().is_empty())
                    .unwrap_or(false)
            });

            let mut headers = vec!["Pokémon".to_string()];
            if show_levels {
                headers.push("Levels".to_string());
            }
            if show_rate {
                headers.push("Rate".to_string());
            }
            if show_notes {
                headers.push("Notes".to_string());
            }

            buf.push_str("| ");
            buf.push_str(&headers.join(" | "));
            buf.push_str(" |\n| ");
            buf.push_str(
                &headers
                    .iter()
                    .map(|_| "---")
                    .collect::<Vec<_>>()
                    .join(" | "),
            );
            buf.push_str(" |\n");

            for entry in &section.table {
                let mut row = vec![entry.pokemon.clone()];
                if show_levels {
                    row.push(entry.levels.clone().unwrap_or_else(|| "—".to_string()));
                }
                if show_rate {
                    row.push(entry.rate.clone().unwrap_or_else(|| "—".to_string()));
                }
                if show_notes {
                    row.push(entry.notes.clone().unwrap_or_else(|| "—".to_string()));
                }
                buf.push_str("| ");
                buf.push_str(&row.join(" | "));
                buf.push_str(" |\n");
            }
            buf.push('\n');
        }
        buf
    }
}

fn build_area(raw: RawLocation, source: Option<String>) -> EncounterArea {
    let id = slugify(&raw.name);
    let mut sections = Vec::new();
    for (method, _) in METHOD_ORDER.iter() {
        if let Some(raw_entries) = raw.methods.get(*method) {
            if raw_entries.is_empty() {
                continue;
            }
            let table = raw_entries
                .iter()
                .map(|entry| EncounterEntry {
                    pokemon: entry.pokemon.clone(),
                    levels: entry.levels.clone(),
                    rate: entry.rate.map(|r| format_rate(r)),
                    notes: entry.notes.clone(),
                })
                .collect::<Vec<_>>();
            if table.is_empty() {
                continue;
            }
            sections.push(EncounterSection {
                name: method_label(method),
                method: method.to_string(),
                table,
            });
        }
    }

    // Include any unexpected methods to avoid losing data.
    for (method, entries) in raw.methods.iter() {
        if METHOD_ORDER
            .iter()
            .any(|(known, _)| *known == method.as_str())
        {
            continue;
        }
        if entries.is_empty() {
            continue;
        }
        let table = entries
            .iter()
            .map(|entry| EncounterEntry {
                pokemon: entry.pokemon.clone(),
                levels: entry.levels.clone(),
                rate: entry.rate.map(|r| format_rate(r)),
                notes: entry.notes.clone(),
            })
            .collect::<Vec<_>>();
        if table.is_empty() {
            continue;
        }
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
        .find(|(key, _)| *key == method)
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

trait TitleCase {
    fn to_title_case(&self) -> String;
}

impl TitleCase for String {
    fn to_title_case(&self) -> String {
        self.as_str().to_title_case()
    }
}

impl TitleCase for &str {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize = true;
        for ch in self.chars() {
            if ch.is_whitespace() || ch == '_' {
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
