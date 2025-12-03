use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
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
        let value: Value = serde_json::from_reader(file)
            .with_context(|| format!("decoding encounter manifest {}", path.display()))?;

        if value.get("locations").is_some() {
            // legacy manifest
            let manifest: EncounterManifest = serde_json::from_value(value)?;
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

            return Ok(build_area_from_methods(
                loc.name,
                build_methods_from_raw(&loc.methods),
                loc.notes,
                meta_source,
            ));
        }

        // new manifest keyed by location name
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("encounter manifest root must be an object"))?;
        let query_slug = slugify(id_or_name);
        let (loc_name, loc_value) = obj
            .iter()
            .find(|(name, _)| slugify(name) == query_slug || name.eq_ignore_ascii_case(id_or_name))
            .ok_or_else(|| anyhow!("no encounter data found for '{id_or_name}'"))?;

        let methods = parse_new_format_methods(loc_value)?;
        Ok(build_area_from_methods(
            loc_name.clone(),
            methods,
            None,
            Some("Pokemon Lazarus Encounters PDF".to_string()),
        ))
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
        buf.push_str(" ☑ |\n");
        buf.push_str("| --- |");
        for _ in &columns {
            buf.push_str(" --- |");
        }
        buf.push_str(" --- |\n");

        let mut rows: Vec<_> = species_map.iter().collect();
        rows.sort_by(|(a_name, a_methods), (b_name, b_methods)| {
            let a_score = a_methods
                .keys()
                .map(|m| method_priority(m))
                .min()
                .unwrap_or(usize::MAX);
            let b_score = b_methods
                .keys()
                .map(|m| method_priority(m))
                .min()
                .unwrap_or(usize::MAX);
            a_score.cmp(&b_score).then_with(|| a_name.cmp(b_name))
        });

        for (species, methods) in rows {
            let link = format!(
                "<a href=\"../pokemon-lookup.html?q={}\">{}</a>",
                slugify(&species),
                species
            );
            buf.push_str(&format!("| {} |", link));
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
            let species_slug = slugify(species);
            buf.push_str(&format!(
                " <input type=\"checkbox\" class=\"caught-check\" data-species=\"{}\" /> |\n",
                species_slug
            ));
        }
        buf.push_str(
            "\n<script>\n(function() {\n  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;\n  const STORAGE_KEY = 'lazarusCaught';\n  function loadState() {\n    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }\n  }\n  function saveState(state) {\n    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}\n  }\n  function applyState() {\n    const state = loadState();\n    const boxes = Array.from(document.querySelectorAll('.caught-check'));\n    const bySpecies = boxes.reduce((m, cb) => {\n      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});\n    boxes.forEach(cb => {\n      const key = cb.dataset.species;\n      cb.checked = !!state[key];\n      cb.onchange = () => {\n        const checked = cb.checked;\n        if (checked) state[key] = true; else delete state[key];\n        saveState(state);\n        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });\n      };\n    });\n  }\n  if (document.readyState === 'loading') {\n    document.addEventListener('DOMContentLoaded', applyState);\n  } else {\n    applyState();\n  }\n})();\n</script>\n\n",
        );
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

fn build_area_from_methods(
    name: String,
    methods: BTreeMap<String, Vec<EncounterEntry>>,
    notes: Option<String>,
    source: Option<String>,
) -> EncounterArea {
    let id = slugify(&name);
    let mut sections = Vec::new();
    let mut seen = BTreeSet::new();

    for (method, label) in METHOD_ORDER.iter() {
        if let Some(entries) = methods.get(*method) {
            if entries.is_empty() {
                continue;
            }
            sections.push(EncounterSection {
                name: (*label).to_string(),
                method: method.to_string(),
                table: entries.clone(),
            });
            seen.insert(method.to_string());
        }
    }

    for (method, entries) in methods.iter() {
        if seen.contains(method) || entries.is_empty() {
            continue;
        }
        seen.insert(method.clone());
        sections.push(EncounterSection {
            name: method_label(method),
            method: method.clone(),
            table: entries.clone(),
        });
    }

    EncounterArea {
        id,
        name,
        source,
        notes,
        sections,
    }
}

fn build_methods_from_raw(
    methods: &BTreeMap<String, Vec<RawEntry>>,
) -> BTreeMap<String, Vec<EncounterEntry>> {
    let mut out: BTreeMap<String, Vec<EncounterEntry>> = BTreeMap::new();
    for (method, entries) in methods {
        out.insert(
            method.clone(),
            entries
                .iter()
                .map(|entry| EncounterEntry {
                    pokemon: entry.pokemon.clone(),
                    levels: entry.levels.clone(),
                    rate: entry.rate.map(format_rate),
                    notes: entry.notes.clone(),
                })
                .collect(),
        );
    }
    out
}

#[derive(Debug)]
struct TableColumn {
    slug: String,
    label: String,
}

const METHOD_ORDER: [(&str, &str); 9] = [
    ("grass_day", "Grass (Day)"),
    ("grass_night", "Grass (Night)"),
    ("surf", "Surfing"),
    ("old_rod", "Old Rod"),
    ("good_rod", "Good Rod"),
    ("super_rod", "Super Rod"),
    ("fishing", "Fishing"),
    ("underwater", "Dive"),
    ("special", "Special Encounters"),
];

fn method_label(method: &str) -> String {
    METHOD_ORDER
        .iter()
        .find(|(slug, _)| *slug == method)
        .map(|(_, label)| label.to_string())
        .unwrap_or_else(|| method.replace('_', " ").to_title_case())
}

fn method_priority(method: &str) -> usize {
    METHOD_ORDER
        .iter()
        .position(|(slug, _)| *slug == method)
        .unwrap_or(METHOD_ORDER.len())
}

fn format_rate(rate: f32) -> String {
    if (rate - rate.round()).abs() < f32::EPSILON {
        format!("{:.0}%", rate)
    } else {
        format!("{:.1}%", rate)
    }
}

pub fn slugify(input: &str) -> String {
    let input = deunicode::deunicode(input);
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

pub fn list_locations(path: &PathBuf) -> Result<Vec<String>> {
    let file = fs::File::open(path)
        .with_context(|| format!("opening encounter manifest {}", path.display()))?;
    let value: Value =
        serde_json::from_reader(file).with_context(|| format!("decoding {}", path.display()))?;

    if value.get("locations").is_some() {
        let manifest: EncounterManifest = serde_json::from_value(value)?;
        return Ok(manifest.locations.into_iter().map(|l| l.name).collect());
    }

    let obj = value
        .as_object()
        .ok_or_else(|| anyhow!("encounter manifest root must be an object"))?;
    let mut names: Vec<String> = obj.keys().cloned().collect();
    names.sort();
    Ok(names)
}

#[derive(Debug, Clone, Default)]
pub struct SpeciesEncounter {
    pub location: String,
    pub method: String,
    pub rate: Option<String>,
}

pub fn species_encounters_map(path: &PathBuf) -> Result<HashMap<String, Vec<SpeciesEncounter>>> {
    let mut map: HashMap<String, Vec<SpeciesEncounter>> = HashMap::new();
    let file = fs::File::open(path)
        .with_context(|| format!("opening encounter manifest {}", path.display()))?;
    let value: Value =
        serde_json::from_reader(file).with_context(|| format!("decoding {}", path.display()))?;

    if value.get("locations").is_some() {
        let manifest: EncounterManifest = serde_json::from_value(value)?;
        for loc in manifest.locations {
            for (method, entries) in loc.methods.iter() {
                let label = method_label(method);
                for entry in entries {
                    insert_species_encounter(
                        &mut map,
                        &entry.pokemon,
                        &loc.name,
                        &label,
                        entry.rate.map(format_rate),
                    );
                }
            }
        }
    } else if let Some(obj) = value.as_object() {
        for (location, methods) in obj.iter() {
            if let Some(methods_obj) = methods.as_object() {
                for (method_name, entries_val) in methods_obj.iter() {
                    let slug = method_slug(method_name);
                    if slug.is_empty() {
                        continue;
                    }
                    let label = method_label(&slug);
                    if let Some(entries) = entries_val.as_array() {
                        for entry in entries {
                            if let Some(pokemon) = entry.get("Pokemon").and_then(|v| v.as_str()) {
                                let rate = entry
                                    .get("Rate")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                                insert_species_encounter(&mut map, pokemon, location, &label, rate);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(map)
}

fn insert_species_encounter(
    map: &mut HashMap<String, Vec<SpeciesEncounter>>,
    pokemon: &str,
    location: &str,
    method_label: &str,
    rate: Option<String>,
) {
    let slug = slugify(pokemon);
    map.entry(slug).or_default().push(SpeciesEncounter {
        location: location.to_string(),
        method: method_label.to_string(),
        rate,
    });
}

fn parse_new_format_methods(value: &Value) -> Result<BTreeMap<String, Vec<EncounterEntry>>> {
    let mut out: BTreeMap<String, Vec<EncounterEntry>> = BTreeMap::new();
    let methods = value
        .as_object()
        .ok_or_else(|| anyhow!("location entry must be an object"))?;
    for (label, entries_val) in methods.iter() {
        let slug = method_slug(label);
        if slug.is_empty() {
            continue;
        }
        let arr = entries_val
            .as_array()
            .ok_or_else(|| anyhow!("method entries must be arrays"))?;
        for entry_val in arr {
            let mut bucket = slug.clone();
            let obj = entry_val
                .as_object()
                .ok_or_else(|| anyhow!("encounter entry must be an object"))?;
            let pokemon = obj
                .get("Pokemon")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("encounter entry missing Pokemon"))?
                .trim()
                .to_string();
            let rate = obj
                .get("Rate")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_string());
            let rod = obj
                .get("Rod")
                .and_then(|v| v.as_str())
                .map(|s| s.to_lowercase());
            if slug == "fishing" {
                if let Some(r) = rod {
                    if r.contains("old rod") {
                        bucket = "old_rod".to_string();
                    } else if r.contains("good rod") {
                        bucket = "good_rod".to_string();
                    } else if r.contains("super rod") {
                        bucket = "super_rod".to_string();
                    } else {
                        bucket = "fishing".to_string();
                    }
                }
            }
            out.entry(bucket.clone()).or_default().push(EncounterEntry {
                pokemon,
                levels: None,
                rate,
                notes: None,
            });
        }
    }
    Ok(out)
}

fn method_slug(label: &str) -> String {
    let lower = label.to_lowercase();
    if lower.contains("land encounters (day") {
        "grass_day".to_string()
    } else if lower.contains("land encounters (night") {
        "grass_night".to_string()
    } else if lower.starts_with("fishing") {
        "fishing".to_string()
    } else if lower.starts_with("surf") {
        "surf".to_string()
    } else if lower.starts_with("underwater") {
        "underwater".to_string()
    } else {
        String::new()
    }
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
