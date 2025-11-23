use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ItemsPage {
    #[clap(alias = "evo")]
    EvolutionKey,
    #[clap(alias = "tms")]
    TmsHms,
    #[clap(alias = "decor")]
    DecorationsOutfits,
}

impl ItemsPage {
    fn title(&self) -> &'static str {
        match self {
            ItemsPage::EvolutionKey => "Items - Evolution & Key Items",
            ItemsPage::TmsHms => "Items - TMs / HMs",
            ItemsPage::DecorationsOutfits => "Items - Decorations & Outfits",
        }
    }

    fn filename(&self) -> &'static str {
        match self {
            ItemsPage::EvolutionKey => "items-evolution-key.md",
            ItemsPage::TmsHms => "items-tms-hms.md",
            ItemsPage::DecorationsOutfits => "items-decorations-outfits.md",
        }
    }

    fn sections(&self) -> Vec<SectionSpec> {
        match self {
            ItemsPage::EvolutionKey => vec![
                SectionSpec::new(
                    "Evolutionary / Form Change Items",
                    "Evolutionary & Form Change Items",
                ),
                SectionSpec::new("Key Items", "Key Items"),
            ],
            ItemsPage::TmsHms => vec![SectionSpec::new("TMs / HMs", "TMs / HMs")],
            ItemsPage::DecorationsOutfits => vec![
                SectionSpec::new("Decorations", "Decorations"),
                SectionSpec::new("Normal Outfits", "Normal Outfits"),
                SectionSpec::new("Special Outfits", "Special Outfits"),
            ],
        }
    }
}

struct SectionSpec {
    manifest_key: &'static str,
    heading: &'static str,
}

impl SectionSpec {
    const fn new(manifest_key: &'static str, heading: &'static str) -> Self {
        Self {
            manifest_key,
            heading,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawManifest(pub BTreeMap<String, Vec<BTreeMap<String, String>>>);

pub fn render_page(manifest_path: PathBuf, page: ItemsPage, out: PathBuf) -> Result<()> {
    let manifest = load_manifest(&manifest_path)?;
    let markdown = render_markdown(&manifest, &page);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, markdown)?;
    Ok(())
}

pub fn render_all_pages(manifest_path: PathBuf, out_dir: PathBuf) -> Result<()> {
    let manifest = load_manifest(&manifest_path)?;
    fs::create_dir_all(&out_dir)?;
    for page in [
        ItemsPage::EvolutionKey,
        ItemsPage::TmsHms,
        ItemsPage::DecorationsOutfits,
    ] {
        let markdown = render_markdown(&manifest, &page);
        let path = out_dir.join(page.filename());
        fs::write(path, markdown)?;
    }
    Ok(())
}

fn render_markdown(manifest: &BTreeMap<String, Vec<ItemRecord>>, page: &ItemsPage) -> String {
    let mut buf = String::new();
    buf.push_str(&format!("# {}\n\n", page.title()));
    buf.push_str("Reference data sourced from `data/items/important-items.json`.\n\n");
    for section in page.sections() {
        let entries = manifest.get(section.manifest_key);
        buf.push_str(&format!("## {}\n\n", section.heading));
        if entries.map(|e| e.is_empty()).unwrap_or(true) {
            buf.push_str("_No data available._\n\n");
            continue;
        }
        if let Some(entries) = entries {
            buf.push_str(&render_table(entries));
        }
        buf.push('\n');
    }
    buf
}

type ItemRecord = BTreeMap<String, String>;

fn render_table(entries: &[ItemRecord]) -> String {
    if entries.is_empty() {
        return "_No entries._\n".to_string();
    }
    let mut columns: BTreeSet<String> = BTreeSet::new();
    for entry in entries {
        for key in entry.keys() {
            columns.insert(key.clone());
        }
    }
    let mut ordered: Vec<String> = columns.into_iter().collect();
    ordered.sort_by_key(|key| column_rank(key));
    ordered.retain(|col| {
        entries.iter().any(|entry| {
            entry
                .get(col)
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false)
        })
    });

    let mut table = String::new();
    table.push_str("| ");
    for col in &ordered {
        table.push_str(col);
        table.push_str(" | ");
    }
    table.push('\n');
    table.push_str("| ");
    for _ in &ordered {
        table.push_str("--- | ");
    }
    table.push('\n');
    for entry in entries {
        table.push_str("| ");
        for col in &ordered {
            let value = entry.get(col).map(|s| s.as_str()).unwrap_or("");
            table.push_str(value);
            table.push_str(" | ");
        }
        table.push('\n');
    }
    table.push('\n');
    table
}

fn column_rank(name: &str) -> u8 {
    match name.to_lowercase().as_str() {
        "item name" => 0,
        "name" => 0,
        "move" => 1,
        "category" => 1,
        "use" => 1,
        "locations" => 2,
        "location" => 2,
        "unlock" => 2,
        _ => 3,
    }
}

fn load_manifest(path: &Path) -> Result<BTreeMap<String, Vec<ItemRecord>>> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("reading items manifest {}", path.display()))?;
    let manifest: RawManifest = serde_json::from_str(&data)
        .with_context(|| format!("parsing items manifest {}", path.display()))?;
    Ok(manifest.0)
}
