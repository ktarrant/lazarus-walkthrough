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

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render the Gen 6+ type matchup table
    TypeChart,
    /// Generate a Pokémon card for the requested species or National Dex number
    PokemonCard { identifier: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::TypeChart => render_type_chart(),
        Command::PokemonCard { identifier } => render_pokemon_card(cli.data_dir, identifier)?,
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
    let card = repo.build_card(&identifier)?;
    print!("{}", card.render_markdown());
    Ok(())
}
