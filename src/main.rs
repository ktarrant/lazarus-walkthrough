mod encounters;
mod items;
mod pokeapi;
mod type_chart;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Content helpers for the Pokémon walkthrough")]
struct Cli {
    /// Location of the downloaded PokeAPI dataset
    #[arg(
        long,
        env = "POKEAPI_DATA_DIR",
        default_value = "data/api-data-master/data/api/v2"
    )]
    data_dir: PathBuf,
    /// Path to the parsed encounter manifest JSON
    #[arg(
        long,
        env = "POKEMON_LAZARUS_ENCOUNTERS_JSON",
        default_value = "data/encounters/encounters.json"
    )]
    encounters_json: PathBuf,
    /// Path to the parsed important items manifest JSON
    #[arg(
        long,
        env = "POKEMON_LAZARUS_ITEMS_JSON",
        default_value = "data/items/important-items.json"
    )]
    items_json: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render the Gen 6+ type matchup table
    TypeChart,
    /// Generate a Pokémon card for the requested species or National Dex number
    PokemonCard { identifier: String },
    /// Generate Pokémon cards for every species found in the encounters manifest
    PokemonCardsAll {
        /// Output directory for generated cards
        #[arg(long, default_value = "book/src/pokemon")]
        out_dir: PathBuf,
    },
    /// Render encounter tables for an area defined under data/encounters
    Encounters { area_id: String },
    /// Render encounter tables for all areas in the manifest
    EncountersAll {
        /// Output directory for generated Markdown files
        #[arg(long, default_value = "book/src/encounters")]
        out_dir: PathBuf,
    },
    /// Render a reference list of egg groups -> encountered Pokémon
    EggGroups {
        /// Output file for the generated Markdown
        #[arg(long, default_value = "book/src/egg-groups.md")]
        out: PathBuf,
    },
    /// Generate item reference pages from the items manifest
    Items {
        #[arg(value_enum)]
        page: items::ItemsPage,
        /// Output path for the generated Markdown file
        #[arg(long)]
        out: PathBuf,
    },
    /// Generate all curated item reference pages
    ItemsAll {
        /// Output directory for generated Markdown files
        #[arg(long, default_value = "book/src")]
        out_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::TypeChart => render_type_chart(),
        Command::PokemonCard { identifier } => {
            render_pokemon_card(cli.data_dir, cli.encounters_json.clone(), identifier)?
        }
        Command::PokemonCardsAll { out_dir } => {
            render_all_pokemon_cards(cli.data_dir, cli.encounters_json, out_dir)?
        }
        Command::Encounters { area_id } => render_encounters(cli.encounters_json, area_id)?,
        Command::EncountersAll { out_dir } => render_all_encounters(cli.encounters_json, out_dir)?,
        Command::EggGroups { out } => render_egg_groups(cli.data_dir, cli.encounters_json, out)?,
        Command::Items { page, out } => items::render_page(cli.items_json, page, out)?,
        Command::ItemsAll { out_dir } => items::render_all_pages(cli.items_json, out_dir)?,
    }
    Ok(())
}

fn render_type_chart() {
    println!("# Type Matchups (Gen 6+)");
    println!();
    println!("This table is generated from the structured data in `src/type_chart.rs`.");
    println!(
        "Run `cargo run -- type-chart > book/src/type-matchups.md` whenever the source data changes."
    );
    println!();
    print!("{}", type_chart::colored_table());
}

fn render_pokemon_card(data_dir: PathBuf, manifest: PathBuf, identifier: String) -> Result<()> {
    let repo = pokeapi::Repository::new(data_dir);
    let encounter_map = encounters::species_encounters_map(&manifest)?;
    for candidate in candidate_identifiers(&identifier) {
        if let Ok(deck) = repo.build_card_deck(&candidate) {
            print!("{}", deck.render_markdown_with_encounters(&encounter_map));
            return Ok(());
        }
    }
    anyhow::bail!("No Pokémon species named {identifier} found in cache")
}

fn render_all_pokemon_cards(data_dir: PathBuf, manifest: PathBuf, out_dir: PathBuf) -> Result<()> {
    let repo = pokeapi::Repository::new(data_dir);
    let species = encounters::list_species(&manifest)?;
    let encounter_map = encounters::species_encounters_map(&manifest)?;
    std::fs::create_dir_all(&out_dir)?;
    let mut index_entries = Vec::new();
    let total = species.len();
    for (idx, name) in species.into_iter().enumerate() {
        println!("Generating card {}/{}: {}", idx + 1, total, name);
        let mut deck = None;
        for candidate in candidate_identifiers(&name) {
            if let Ok(d) = repo.build_card_deck(&candidate) {
                deck = Some(d);
                break;
            }
        }
        if let Some(deck) = deck {
            let slug = encounters::slugify(&name);
            let path = out_dir.join(format!("{slug}.md"));
            std::fs::write(path, deck.render_markdown_with_encounters(&encounter_map))?;
            index_entries.push((slug, name));
        } else {
            eprintln!("Failed to generate card for {}; writing placeholder", name);
            let slug = encounters::slugify(&name);
            let path = out_dir.join(format!("{slug}.md"));
            std::fs::write(
                path,
                format!(
                    "## {}\n\n_Data for this form is not available in the local cache._\n",
                    name
                ),
            )?;
            index_entries.push((slug, name));
        }
    }
    println!("Finished generating {} Pokémon cards", total);
    write_index(&out_dir, "Pokémon Cards", &index_entries)?;
    Ok(())
}

fn candidate_identifiers(name: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let trimmed = name.trim();
    let hyphenated = trimmed
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string();

    if let Some((base, variety)) = canonical_form_slug(&hyphenated) {
        ids.push(variety);
        ids.push(base);
    }
    ids.push(hyphenated.clone());
    ids.push(trimmed.to_string());

    let words: Vec<&str> = trimmed.split_whitespace().collect();
    if let Some(last) = words.last() {
        ids.push(last.to_string());
    }
    if let Some(first) = words.first() {
        ids.push(first.to_string());
    }

    ids.sort();
    ids.dedup();
    ids
}

fn write_index(out_dir: &PathBuf, title: &str, entries: &[(String, String)]) -> Result<()> {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut buf = String::new();
    buf.push_str(&format!("# {title}\n\n"));
    for (slug, name) in sorted {
        buf.push_str(&format!("- [{name}](./{slug}.md)\n"));
    }
    std::fs::write(out_dir.join("index.md"), buf)?;
    Ok(())
}

fn canonical_form_slug(slug: &str) -> Option<(String, String)> {
    const FORM_PREFIXES: [(&str, &str); 8] = [
        ("alolan-", "alola"),
        ("hisuian-", "hisui"),
        ("galarian-", "galar"),
        ("paldean-", "paldea"),
        ("white-striped-", "white-striped"),
        ("blue-striped-", "blue-striped"),
        ("red-striped-", "red-striped"),
        ("black-striped-", "black-striped"),
    ];

    for (prefix, suffix) in FORM_PREFIXES.iter() {
        if slug.starts_with(prefix) {
            let base = slug
                .trim_start_matches(prefix)
                .trim_matches('-')
                .to_string();
            return Some((base.clone(), format!("{base}-{suffix}")));
        }
    }

    const FORM_SUFFIXES: [(&str, &str); 5] = [
        ("-blue-flower", "blue"),
        ("-orange-flower", "orange"),
        ("-red-flower", "red"),
        ("-white-flower", "white"),
        ("-yellow-flower", "yellow"),
    ];
    for (pattern, suffix) in FORM_SUFFIXES.iter() {
        if slug.ends_with(pattern) {
            let base = slug.trim_end_matches(pattern).trim_matches('-').to_string();
            return Some((base.clone(), format!("{base}-{suffix}")));
        }
    }

    match slug {
        "sligoo" => Some(("sliggoo".to_string(), "sliggoo".to_string())),
        "hisuian-sligoo" => Some(("sliggoo".to_string(), "sliggoo-hisui".to_string())),
        _ => None,
    }
}

fn render_encounters(manifest: PathBuf, area_id: String) -> Result<()> {
    let area = encounters::EncounterArea::from_manifest(manifest, &area_id)?;
    print!("{}", area.render_markdown());
    Ok(())
}

fn render_all_encounters(manifest: PathBuf, out_dir: PathBuf) -> Result<()> {
    let names = encounters::list_locations(&manifest)?;
    std::fs::create_dir_all(&out_dir)?;
    let mut index_entries = Vec::new();
    for name in names {
        let area = encounters::EncounterArea::from_manifest(manifest.clone(), &name)?;
        let slug = encounters::slugify(&area.name);
        let path = out_dir.join(format!("{slug}.md"));
        std::fs::write(&path, area.render_markdown())?;
        index_entries.push((slug, area.name));
    }
    write_index(&out_dir, "Encounter Tables", &index_entries)?;
    Ok(())
}

fn render_egg_groups(data_dir: PathBuf, manifest: PathBuf, out: PathBuf) -> Result<()> {
    let repo = pokeapi::Repository::new(data_dir);
    let species_list = encounters::list_species(&manifest)?;
    let mut groups: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for name in species_list {
        let mut species_record = None;
        for candidate in candidate_identifiers(&name) {
            if let Ok(species) = repo.load_species(&candidate) {
                species_record = Some(species);
                break;
            }
        }
        let Some(species) = species_record else {
            eprintln!("Skipped egg group lookup for {name}; species not found");
            continue;
        };

        let slug = encounters::slugify(&name);
        let display = name.clone();
        for group in species.egg_groups {
            let label = format_egg_group(&group.name);
            groups
                .entry(label)
                .or_default()
                .push((display.clone(), slug.clone()));
        }
    }

    for entries in groups.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let mut buf = String::new();
    buf.push_str("# Egg Groups\n\n");
    buf.push_str("Pokémon appearing in the Lazarus encounters grouped by egg group.\n\n");
    buf.push_str("<div class=\"egg-group-grid\">\n");
    for (group, entries) in groups {
        buf.push_str("<div class=\"egg-group-section\">\n");
        buf.push_str(&format!("<h3>{}</h3>\n<ul>\n", html_escape(&group)));
        for (display, slug) in entries {
            buf.push_str(&format!(
                "<li><a href=\"./pokemon/{slug}.md\">{name}</a></li>\n",
                slug = slug,
                name = html_escape(&display)
            ));
        }
        buf.push_str("</ul>\n</div>\n");
    }
    buf.push_str("</div>\n");
    std::fs::write(out, buf)?;
    Ok(())
}

fn format_egg_group(name: &str) -> String {
    name.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn html_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}
