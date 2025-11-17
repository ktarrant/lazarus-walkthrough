# Pokémon Lazarus Walkthrough

This repository contains an mdBook walkthrough focused on the Pokémon Lazarus
ROM hack. The book highlights each region’s story beats, recommended teams,
trainer battles, and route encounters. Supporting scripts and data help keep the
reference sections in sync with upstream sources.

## Project structure

- `book/` – mdBook source for the walkthrough.
- `src/` – Rust helpers that generate Markdown for reusable artifacts
  (type chart, Pokémon cards, encounter tables).
- `parsers/` – uv-based Python project for parsing the encounter PDF.
- `data/` – cached datasets (PokeAPI mirror, parsed encounters JSON).

## Prerequisites

- Rust toolchain (1.76+ recommended) for building the helper CLI.
- [uv](https://github.com/astral-sh/uv) for running the Python parser without
  polluting the global environment.
- `mdbook` and the `mdbook-autosummary` preprocessor for local previews
  (`cargo install mdbook mdbook-autosummary`).

## Commands & workflows

### Type matchup table

The Rust CLI renders the Gen 6+ matchup chart:

```sh
cargo run -- type-chart > book/src/type-matchups.md
```

Regenerate the file whenever `src/type_chart.rs` changes.

### Pokémon cards

1. Download the PokeAPI dataset:
   ```sh
   mkdir -p data
   curl -L https://github.com/PokeAPI/api-data/archive/refs/heads/master.zip \
     -o data/pokeapi-api-data.zip
   unzip -q data/pokeapi-api-data.zip -d data
   ```
2. Generate a card (replace `sprigatito` with any species name or dex number):
   ```sh
   cargo run -- pokemon-card sprigatito > book/src/pokemon/sprigatito.md
   ```
   Use `POKEAPI_DATA_DIR=/custom/path` if the dataset lives elsewhere.
3. Embed cards with `{{#include ./pokemon/<name>.md}}` inside the book.

### Encounter tables

1. Convert the PDF via the uv parser:
   ```sh
   cd parsers
   uv run python convert_lazarus_encounters.py \
     "../Pokemon Lazarus Documentation - Encounters.pdf" \
     --out ../data/encounters/encounters.json
   ```
2. Generate Markdown for a single location (slug or full name):
   ```sh
   cargo run -- encounters bronze-fields-north > book/src/encounters/bronze-fields-north.md
   ```
   Or generate Markdown for every location in the manifest:
   ```sh
   cargo run -- encounters-all --out-dir book/src/encounters
   ```
3. Generate Pokémon cards for every species found in the encounters data:
   ```sh
   cargo run -- pokemon-cards-all --out-dir book/src/pokemon
   ```
4. Include the snippet in chapters via `{{#include ./encounters/bronze-fields-north.md}}`. Encounter tables now link each species name to its generated card.

### mdBook preview
Due to a limitation in mdbook-autosummary, an empty SUMMARY.md should be created
before running build or serve.

```sh
mdbook serve --open
```

The `mdbook-autosummary` preprocessor keeps `SUMMARY.md` in sync with `book/src`
automatically during builds. Index files under `book/src/encounters/` and
`book/src/pokemon/` are produced by the helper CLI to support this workflow.

## Conventions

- Never commit `data/api-data-master` or other large caches; only the JSON
  manifests in `data/encounters/` should be tracked.
- Keep documentation focused on narrative content; process notes belong here or
  in inline comments.
- Use the helper CLI wherever possible instead of hand-editing generated files.
