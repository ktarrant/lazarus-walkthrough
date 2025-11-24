mod encounters;
mod items;
mod pokedex;
mod pokemon_card;
mod type_chart;

use crate::pokemon_card::non_empty;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Content helpers for the Pokémon walkthrough")]
struct Cli {
    /// Path to the structured Lazarus Pokédex JSON
    #[arg(
        long,
        env = "POKEMON_LAZARUS_POKEDEX_JSON",
        default_value = "data/pokedex/lazarus_pokedex.json"
    )]
    pokedex_json: PathBuf,
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
    /// Generate a move catalog (move -> Pokémon -> method)
    MoveCatalog {
        /// Output file for the generated Markdown
        #[arg(long, default_value = "book/src/move-catalog.md")]
        out: PathBuf,
    },
    /// Generate an ability catalog (ability -> Pokémon -> slot info)
    AbilityCatalog {
        /// Output file for the generated Markdown
        #[arg(long, default_value = "book/src/ability-catalog.md")]
        out: PathBuf,
    },
    /// Generate a Pokédex page that includes every card in dex order
    PokedexPage {
        /// Output file for the generated Markdown
        #[arg(long, default_value = "book/src/pokedex.md")]
        out: PathBuf,
    },
    /// Generate a lookup page for Pokémon cards
    PokemonLookup {
        /// Output file for the generated Markdown
        #[arg(long, default_value = "book/src/pokemon-lookup.md")]
        out: PathBuf,
    },
    /// Run all generators (cards, encounters, catalogs, lookup)
    All {
        /// Output directory for generated cards
        #[arg(long, default_value = "book/src/pokemon")]
        cards_out: PathBuf,
        /// Output directory for encounter tables
        #[arg(long, default_value = "book/src/encounters")]
        encounters_out: PathBuf,
        /// Output path for move catalog
        #[arg(long, default_value = "book/src/move-catalog.md")]
        move_out: PathBuf,
        /// Output path for ability catalog
        #[arg(long, default_value = "book/src/ability-catalog.md")]
        ability_out: PathBuf,
        /// Output path for egg groups index
        #[arg(long, default_value = "book/src/egg-groups.md")]
        eggs_out: PathBuf,
        /// Output path for Pokédex page
        #[arg(long, default_value = "book/src/pokedex.md")]
        pokedex_out: PathBuf,
        /// Output path for lookup page
        #[arg(long, default_value = "book/src/pokemon-lookup.md")]
        lookup_out: PathBuf,
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
        Command::PokemonCard { identifier } => render_pokemon_card(
            cli.pokedex_json.clone(),
            cli.encounters_json.clone(),
            identifier,
        )?,
        Command::PokemonCardsAll { out_dir } => {
            render_all_pokemon_cards(cli.pokedex_json.clone(), cli.encounters_json, out_dir)?
        }
        Command::Encounters { area_id } => render_encounters(cli.encounters_json, area_id)?,
        Command::EncountersAll { out_dir } => render_all_encounters(cli.encounters_json, out_dir)?,
        Command::EggGroups { out } => render_egg_groups(cli.pokedex_json.clone(), out)?,
        Command::MoveCatalog { out } => render_move_catalog(cli.pokedex_json.clone(), out)?,
        Command::AbilityCatalog { out } => render_ability_catalog(cli.pokedex_json.clone(), out)?,
        Command::PokedexPage { out } => render_pokedex_page(cli.pokedex_json.clone(), out)?,
        Command::PokemonLookup { out } => render_pokemon_lookup(cli.pokedex_json.clone(), out)?,
        Command::Items { page, out } => items::render_page(cli.items_json, page, out)?,
        Command::ItemsAll { out_dir } => items::render_all_pages(cli.items_json, out_dir)?,
        Command::All {
            cards_out,
            encounters_out,
            move_out,
            ability_out,
            eggs_out,
            pokedex_out,
            lookup_out,
        } => {
            render_all_pokemon_cards(
                cli.pokedex_json.clone(),
                cli.encounters_json.clone(),
                cards_out,
            )?;
            render_all_encounters(cli.encounters_json.clone(), encounters_out)?;
            render_move_catalog(cli.pokedex_json.clone(), move_out)?;
            render_ability_catalog(cli.pokedex_json.clone(), ability_out)?;
            render_egg_groups(cli.pokedex_json.clone(), eggs_out)?;
            render_pokedex_page(cli.pokedex_json.clone(), pokedex_out)?;
            render_pokemon_lookup(cli.pokedex_json.clone(), lookup_out)?;
        }
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

fn render_pokemon_card(pokedex_path: PathBuf, manifest: PathBuf, identifier: String) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let encounter_map = encounters::species_encounters_map(&manifest)?;
    if let Some((entry, _slug_used)) = find_entry(&dex, &identifier) {
        let chain = dex.evolution_chain(&entry.slug);
        print!(
            "{}",
            pokemon_card::render_deck(&chain, &entry.slug, &encounter_map)
        );
        return Ok(());
    }
    anyhow::bail!("No Pokémon species named {identifier} found in cache")
}

fn render_all_pokemon_cards(
    pokedex_path: PathBuf,
    manifest: PathBuf,
    out_dir: PathBuf,
) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let encounter_map = encounters::species_encounters_map(&manifest)?;
    std::fs::create_dir_all(&out_dir)?;
    let entries = dex.all_entries();
    let total = entries.len();
    for entry in entries {
        let slug = entry.slug.clone();
        let path = out_dir.join(format!("{slug}.md"));
        let chain = dex.evolution_chain(&entry.slug);
        std::fs::write(
            path,
            pokemon_card::render_deck(&chain, &entry.slug, &encounter_map),
        )?;
    }
    println!("Finished generating {} Pokémon cards", total);
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

fn find_entry<'a>(
    dex: &'a pokedex::LazarusPokedex,
    name: &str,
) -> Option<(&'a pokedex::PokemonEntry, String)> {
    let slug = encounters::slugify(name);
    if let Some(entry) = dex.get_by_slug(&slug) {
        return Some((entry, slug));
    }
    if let Some(alias) = alias_slug(&slug) {
        if let Some(entry) = dex.get_by_slug(&alias) {
            return Some((entry, alias));
        }
    }
    for candidate in candidate_identifiers(name) {
        if let Some(entry) = dex.find(&candidate) {
            return Some((entry, entry.slug.clone()));
        }
    }
    None
}

fn alias_slug(slug: &str) -> Option<String> {
    match slug {
        "flabebe-blue-flower"
        | "flabebe-orange-flower"
        | "flabebe-red-flower"
        | "flabebe-white-flower"
        | "flabebe-yellow-flower" => Some("flabebe".to_string()),
        "hisuian-sliggo" => Some("sliggoo-hisui".to_string()),
        "hisuian-sliggoo" => Some("hisuian-sliggoo".to_string()),
        "hisuian-sligoo" => Some("hisuian-sliggoo".to_string()),
        "white-striped-basculin" => Some("basculin-white".to_string()),
        "unovan-basculin" => Some("basculin-red".to_string()),
        _ => None,
    }
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
    for name in names {
        let area = encounters::EncounterArea::from_manifest(manifest.clone(), &name)?;
        let slug = encounters::slugify(&area.name);
        let path = out_dir.join(format!("{slug}.md"));
        std::fs::write(&path, area.render_markdown())?;
    }
    Ok(())
}

fn render_move_catalog(pokedex_path: PathBuf, out: PathBuf) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let mut catalog: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for entry in dex.all_entries() {
        for mv in &entry.level_up_moves {
            let method = if mv.level.trim().is_empty() {
                "Level".to_string()
            } else {
                format!("Level {}", mv.level.trim())
            };
            catalog
                .entry(mv.move_name.trim().to_string())
                .or_default()
                .push((entry.name.clone(), method));
        }
        for mv in &entry.egg_moves {
            let move_name = mv.trim();
            if move_name.is_empty() {
                continue;
            }
            catalog
                .entry(move_name.to_string())
                .or_default()
                .push((entry.name.clone(), "Egg".to_string()));
        }
        for mv in &entry.tm_moves {
            let move_name = mv.trim();
            if move_name.is_empty() {
                continue;
            }
            catalog
                .entry(move_name.to_string())
                .or_default()
                .push((entry.name.clone(), "TM/HM".to_string()));
        }
        for mv in &entry.tutor_moves {
            let move_name = mv.trim();
            if move_name.is_empty() {
                continue;
            }
            catalog
                .entry(move_name.to_string())
                .or_default()
                .push((entry.name.clone(), "Tutor".to_string()));
        }
    }

    let mut buf = String::new();
    buf.push_str("# Move Catalog\n\n");
    buf.push_str("Moves available in Lazarus with the Pokémon that learn them and the acquisition method.\n\n");
    buf.push_str("| Move | Pokémon | How |\n| --- | --- | --- |\n");
    for (mv, entries) in &catalog {
        let mut sorted = entries.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, how) in sorted {
            buf.push_str(&format!("| {} | {} | {} |\n", mv, name, how));
        }
    }
    std::fs::write(out, buf)?;
    println!("Generated move catalog with {} moves", catalog.len());
    Ok(())
}

fn render_ability_catalog(pokedex_path: PathBuf, out: PathBuf) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let mut catalog: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for entry in dex.all_entries() {
        if let Some(primary) = non_empty(&entry.abilities.primary) {
            catalog
                .entry(primary.to_string())
                .or_default()
                .push((entry.name.clone(), "Primary".to_string()));
        }
        if let Some(secondary) = non_empty(&entry.abilities.secondary) {
            catalog
                .entry(secondary.to_string())
                .or_default()
                .push((entry.name.clone(), "Secondary".to_string()));
        }
        if let Some(hidden) = non_empty(&entry.abilities.hidden) {
            catalog
                .entry(hidden.to_string())
                .or_default()
                .push((entry.name.clone(), "Hidden".to_string()));
        }
    }

    let mut buf = String::new();
    buf.push_str("# Ability Catalog\n\n");
    buf.push_str("Abilities available in Lazarus with the Pokémon that can have them.\n\n");
    buf.push_str("| Ability | Pokémon | Slot |\n| --- | --- | --- |\n");
    for (ability, entries) in &catalog {
        let mut sorted = entries.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, slot) in sorted {
            buf.push_str(&format!("| {} | {} | {} |\n", ability, name, slot));
        }
    }
    std::fs::write(out, buf)?;
    println!("Generated ability catalog with {} abilities", catalog.len());
    Ok(())
}

fn render_pokemon_lookup(pokedex_path: PathBuf, out_path: PathBuf) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let mut entries = Vec::new();
    for entry in dex.all_entries() {
        entries.push((entry.name.clone(), entry.slug.clone()));
    }
    entries.sort_by(|a, b| a.1.cmp(&b.1));
    let total_entries = entries.len();

    let mut buf = String::new();
    buf.push_str("# Pokémon Lookup\n\n");
    buf.push_str("Type to filter and reveal a matching Pokémon card.\n\n");
    buf.push_str(r#"<input id="lookup-input" type="text" placeholder="Start typing a name..." />"#);
    buf.push_str("\n\n<div id=\"lookup-status\"></div>\n\n<div id=\"lookup-cards\">\n");
    for (name, slug) in entries {
        let search = format!("{} {}", name.to_lowercase(), slug);
        buf.push_str(&format!(
            "<div class=\"lookup-card\" data-name=\"{}\">\n{{{{#include ./pokemon/{}.md}}}}\n</div>\n",
            search, slug
        ));
    }
    buf.push_str("</div>\n\n<script>\nconst input = document.getElementById('lookup-input');\nconst cards = Array.from(document.querySelectorAll('.lookup-card'));\nconst status = document.getElementById('lookup-status');\nfunction presetFromUrl() {\n  const params = new URLSearchParams(window.location.search);\n  const hash = window.location.hash.replace('#','');\n  const q = params.get('q') || hash;\n  if (q) {\n    input.value = q;\n  }\n}\nfunction expandAllDetails() {\n  for (const card of cards) {\n    const detail = card.querySelector('details.pokemon-card-container');\n    if (detail) {\n      detail.setAttribute('open', '');\n    }\n  }\n}\nfunction applyFilter() {\n  const q = input.value.trim().toLowerCase();\n  let shown = 0;\n  for (const card of cards) {\n    if (!q) {\n      card.style.display = 'none';\n      continue;\n    }\n    if (card.dataset.name.includes(q) && shown === 0) {\n      card.style.display = 'block';\n      shown += 1;\n    } else {\n      card.style.display = 'none';\n    }\n  }\n  status.textContent = q && shown === 0 ? 'No match found' : '';\n}\nexpandAllDetails();\npresetFromUrl();\napplyFilter();\ninput.addEventListener('input', applyFilter);\n</script>\n");
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out_path, buf)?;
    println!("Generated Pokémon lookup with {} entries", total_entries);
    Ok(())
}

fn render_pokedex_page(pokedex_path: PathBuf, out_path: PathBuf) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let mut entries = dex.all_entries();
    entries.sort_by(|a, b| match (a.dex, b.dex) {
        (Some(ad), Some(bd)) => ad.cmp(&bd),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.name.cmp(&b.name),
    });

    let mut buf = String::new();
    buf.push_str("# Pokédex\n\n");
    buf.push_str("All Pokémon cards in dex order.\n\n");
    for entry in &entries {
        buf.push_str(&format!("{{{{#include ./pokemon/{}.md}}}}\n\n", entry.slug));
    }
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out_path, buf)?;
    println!("Generated Pokédex page with {} entries", entries.len());
    Ok(())
}

fn render_egg_groups(pokedex_path: PathBuf, out: PathBuf) -> Result<()> {
    let dex = pokedex::LazarusPokedex::load(pokedex_path)?;
    let mut groups: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for entry in dex.all_entries() {
        let slug = entry.slug.clone();
        let display = entry.name.clone();
        for group in &entry.egg_groups {
            let label = format_egg_group(group);
            groups
                .entry(label)
                .or_default()
                .push((display.clone(), slug.clone()));
        }
    }

    for entries in groups.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(&b.0));
    }
    let group_count = groups.len();

    let mut buf = String::new();
    buf.push_str("# Egg Groups\n\n");
    buf.push_str("Pokémon available in Lazarus grouped by egg group.\n\n");
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
    println!("Generated egg group index with {} groups", group_count);
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
