mod encounters;
mod pokeapi;
mod type_chart;

use anyhow::Result;
use clap::{Parser, Subcommand};
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

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render the Gen 6+ type matchup table
    TypeChart,
    /// Generate a Pokémon card for the requested species or National Dex number
    PokemonCard { identifier: String },
    /// Render encounter tables for an area defined under data/encounters
    Encounters { area_id: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::TypeChart => render_type_chart(),
        Command::PokemonCard { identifier } => render_pokemon_card(cli.data_dir, identifier)?,
        Command::Encounters { area_id } => render_encounters(cli.encounters_json, area_id)?,
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

fn render_pokemon_card(data_dir: PathBuf, identifier: String) -> Result<()> {
    let repo = pokeapi::Repository::new(data_dir);
    let deck = repo.build_card_deck(&identifier)?;
    print!("{}", deck.render_markdown());
    Ok(())
}

fn render_encounters(manifest: PathBuf, area_id: String) -> Result<()> {
    let area = encounters::EncounterArea::from_manifest(manifest, &area_id)?;
    print!("{}", area.render_markdown());
    Ok(())
}
