# Encounter Tables

Wild encounters for Pokémon Lazarus are pulled directly from
`Pokemon Lazarus Documentation - Encounters.pdf`. The `parsers/` uv project
contains a helper that extracts every table into a normalized JSON file that the
Rust tooling can consume.

## 1. Refresh the master dataset

```
cd parsers
uv run python convert_lazarus_encounters.py \
  "../Pokemon Lazarus Documentation - Encounters.pdf" \
  --json ../data/encounters/encounters.json \
  --csv ../data/encounters/encounters.csv
```

Commit the resulting JSON so the mdBook build always has access to the latest
encounter data.

## 2. Render Markdown for the book

```
cargo run -- encounters bronze-fields-north > book/src/encounters/bronze-fields-north.md
```

Use either the location slug (e.g. `bronze-fields-north`) or the full display
name (e.g. `"Bronze Fields (North)"`). Include the generated file in any chapter
with the standard mdBook directive:

```
{{#include ./encounters/bronze-fields-north.md}}
```

This keeps the walkthrough synchronized with the canonical PDF while allowing
you to drop encounter tables wherever they are needed.
