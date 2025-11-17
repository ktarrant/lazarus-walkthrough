# Encounter Data

Wild encounter information for Pokémon Lazarus is sourced from the PDF
`Pokemon Lazarus Documentation - Encounters.pdf` and extracted via the Python
utility in the `parsers/` uv project.

## Generating the master JSON

```
cd parsers
uv run python convert_lazarus_encounters.py \
  "../Pokemon Lazarus Documentation - Encounters.pdf" \
  --out ../data/encounters/encounters.json
```

This command writes a normalized JSON file (and optional CSV) that the Rust CLI
consumes. Commit the JSON in `data/encounters/encounters.json` so mdBook builds
stay deterministic.

## Rendering Markdown for the walkthrough

```
cargo run -- encounters bronze-fields-north > book/src/encounters/bronze-fields-north.md
# or generate every location at once
cargo run -- encounters-all --out-dir book/src/encounters
```

Use the location slug (lowercase with dashes) or the exact display name to
render other areas. The `encounters-all` helper iterates every location found in
`data/encounters/encounters.json` and writes one Markdown file per area.

Once a snippet is generated, include it in the relevant chapter with:

```
{{#include ./encounters/bronze-fields-north.md}}
```

This keeps the encounter data in sync with the authoritative PDF while making
it easy to reuse across multiple chapters.
