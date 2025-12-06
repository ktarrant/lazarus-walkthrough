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
- `data/` – cached datasets (custom Pokédex JSON, parsed encounters/items).

## Prerequisites

- Rust toolchain (1.76+ recommended) for building the helper CLI.
- [uv](https://github.com/astral-sh/uv) for running the Python parser without
  polluting the global environment.
- `mdbook` for local previews (`cargo install mdbook`).

### Restoring the local PokeAPI cache
Several generators expect a local copy of the PokeAPI data (moves, Pokémon, species).
If the cache is missing, restore it before generating cards or move data:

1. Download the official PokeAPI data dump (api-data):
   ```sh
   git clone https://github.com/PokeAPI/api-data.git data/api-data-master
   ```
   (Or download the zip from GitHub and extract it into `data/api-data-master`.)
2. Verify paths look like `data/api-data-master/data/api/v2/move/1/index.json`,
   `.../pokemon/1/index.json`, etc.
3. Regenerate content after the cache is present (e.g., `cargo run -- all`).

Keep the cache out of source control if it’s large; only the derived JSON/Markdown
artifacts should be committed.

## Commands & workflows

### Type matchup table

The Rust CLI renders the Gen 6+ matchup chart:

```sh
cargo run -- type-chart > book/src/type-matchups.md
```

Regenerate the file whenever `src/type_chart.rs` changes.

### Pokémon cards

1. Convert the official CSV (`sources/Lazarus Data - Pokemon Data.csv`) into the structured
   JSON used by the helper CLI:
   ```sh
   cd parsers
   uv run python convert_lazarus_pokedex.py \
     "../sources/Lazarus Data - Pokemon Data.csv" \
     --json ../data/pokedex/lazarus_pokedex.json
   ```
2. Generate a card (replace `sprigatito` with any species name or dex number):
   ```sh
   cargo run -- pokemon-card sprigatito > book/src/pokemon/sprigatito.md
   ```
   Pass `--pokedex-json /custom/path.json` if the JSON lives elsewhere.
3. Embed cards with `{{#include ./pokemon/<name>.md}}` inside the book.

Generate a full Pokédex page (includes every card in dex order):

```sh
cargo run -- pokedex-page --out book/src/pokedex.md
```

Generate move cards and lookup:

```sh
cargo run -- move-cards-all --out-dir book/src/moves
cargo run -- move-lookup --out book/src/move-lookup.md
```

Generate a quests table (includes persistent checkboxes shared with chapter quests):

```sh
cargo run -- quests --csv "sources/Lazarus Data - Quests.csv" --out book/src/quests.md
```

### Encounter tables

1. Convert the PDF via the uv parser:
   ```sh
   cd parsers
uv run python convert_lazarus_encounters.py \
     "../sources/Pokemon Lazarus Documentation - Encounters.pdf" \
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

### Important item locations

Parse the “Important Item Locations” PDF into a structured JSON manifest:

```sh
cd parsers
uv run python convert_lazarus_items.py \
  "../sources/Pokemon Lazarus Documentation - Important Item Locations.pdf" \
  --json ../data/items/important-items.json
```

The output groups each category (e.g., Evolutionary Items, Key Items, TMs) and
lists the fields shown in the PDF. Reference the JSON when generating new
documentation pages.

Render the curated Markdown pages and commit them to `book/src`:

```sh
cargo run -- items-all --out-dir book/src
```

This produces:

- `items-evolution-key.md`
- `items-tms-hms.md`
- `items-decorations-outfits.md`

Include these pages anywhere in the walkthrough via
`{{#include ./items-evolution-key.md}}` (or the respective filename).

### Pokemon breeding reference

Generate a Markdown page covering breeding basics plus an egg-group index that
links to the Pokémon cards for each species:

```sh
cargo run -- egg-groups --out book/src/pokemon-breeding.md
```

The file is safe to commit and can be linked wherever needed in the book.

### Move & ability catalogs

Generate reference tables that invert the Pokédex data:

```sh
cargo run -- move-catalog --out book/src/move-catalog.md
cargo run -- ability-catalog --out book/src/ability-catalog.md
```

Each table lists the move or ability, which Pokémon can learn/possess it, and
the acquisition method or ability slot.

### mdBook preview

```sh
mdbook serve --open
```

## GitHub Pages deployment

The repository includes `.github/workflows/deploy-pages.yml`, which builds the
book with `mdbook build book` and publishes the rendered HTML via GitHub Pages.

To host the walkthrough:

1. Push the workflow to your default branch (`main` by default).
2. In the GitHub UI, open **Settings → Pages** and set **Source** to
   **GitHub Actions**.
3. Trigger the workflow (push to `main` or use **Run workflow**). The job
   installs `mdbook`, runs the build, uploads `book/book` as the artifact, and
   deploys it with `actions/deploy-pages`.
4. Once the workflow finishes, the public URL appears in the `github-pages`
   environment summary. Configure a custom domain there if needed.

## Conventions

- Never commit `data/api-data-master` or other large caches; only the JSON
  manifests in `data/encounters/` should be tracked.
- Keep documentation focused on narrative content; process notes belong here or
  in inline comments.
- Use the helper CLI wherever possible instead of hand-editing generated files.
