#!/usr/bin/env python3
"""Convert the Lazarus custom Pokédex CSV into structured JSON."""

import argparse
import csv
import json
from pathlib import Path
from typing import Any, Dict, List, Optional


def slugify(name: str) -> str:
    slug = []
    for ch in name.strip().lower():
        if ch.isalnum():
            slug.append(ch)
        elif ch in {" ", "_", "-"}:
            if not slug or slug[-1] != "-":
                slug.append("-")
        elif ch in {"♀", "⚥"}:
            slug.append("-f")
        elif ch == "♂":
            slug.append("-m")
    return "".join(slug).strip("-")


def parse_level_move(value: str) -> Dict[str, str]:
    if not value:
        return {}
    if "," in value:
        level, move = value.split(",", 1)
        return {"level": level.strip(), "move": move.strip()}
    return {"level": value.strip(), "move": ""}


def parse_move_list(values: List[str], is_level: bool = False) -> List[Any]:
    moves: List[Any] = []
    for raw in values:
        raw = raw.strip()
        if not raw:
            continue
        if is_level:
            entry = parse_level_move(raw)
            if entry:
                moves.append(entry)
        else:
            moves.append(raw)
    return moves


def infer_evolves_from(location: str) -> Optional[str]:
    if "(evolve" not in location.lower():
        return None
    prefix = location.split("(evolve", 1)[0].strip().strip(",")
    if not prefix:
        return None
    return slugify(prefix)


def parse_row(row: List[str]) -> Dict[str, Any]:
    name = row[0].strip()
    if not name:
        return {}
    types = [t.strip() for t in row[2:4] if t.strip()]
    stats_keys = ["hp", "attack", "defense", "sp_atk", "sp_def", "speed"]
    stats = {}
    for idx, key in enumerate(stats_keys, start=4):
        value = row[idx].strip()
        stats[key] = int(value) if value.isdigit() else 0
    level_range = row[33:59]
    egg_moves_range = row[60:78]
    tm_range = row[78:119]
    tutor_range = row[119:144]

    egg_groups = []
    for g in row[31:33]:
        g = g.strip()
        if g and g not in egg_groups:
            egg_groups.append(g)

    return {
        "name": name,
        "slug": slugify(name),
        "dex": int(row[1]) if row[1].strip().isdigit() else None,
        "types": types,
        "stats": stats,
        "abilities": {
            "primary": row[25].strip(),
            "secondary": row[26].strip(),
            "hidden": row[27].strip(),
        },
        "evolution": row[28].strip(),
        "held_item": row[29].strip(),
        "location": row[30].strip(),
        "egg_groups": egg_groups,
        "level_up_moves": parse_move_list(level_range, is_level=True),
        "egg_moves": parse_move_list(egg_moves_range),
        "tm_moves": parse_move_list(tm_range),
        "tutor_moves": parse_move_list(tutor_range),
        "evolves_from": None,
        "evolves_to": [],
    }


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Convert Lazarus CSV data into JSON for card generation"
    )
    parser.add_argument("csv", help="Path to Lazarus Data - Raw Data CSV")
    parser.add_argument(
        "--json",
        default="../data/pokedex/lazarus_pokedex.json",
        help="Output path for structured JSON",
    )
    args = parser.parse_args()

    csv_path = Path(args.csv)
    out_path = Path(args.json)
    out_path.parent.mkdir(parents=True, exist_ok=True)

    with csv_path.open(newline="", encoding="utf-8-sig") as f:
        reader = csv.reader(f)
        rows = list(reader)

    entries = []
    for row in rows[2:]:
        entry = parse_row(row)
        if entry:
            entries.append(entry)

    # Infer evolution links based on location hints like "Dartrix (evolve)".
    slug_map = {e["slug"]: e for e in entries}
    for entry in entries:
        loc = entry.get("location", "")
        evolve_from = infer_evolves_from(loc)
        if evolve_from:
            entry["evolves_from"] = evolve_from
            if evolve_from in slug_map:
                slug_map[evolve_from].setdefault("evolves_to", []).append(entry["slug"])

    out_path.write_text(json.dumps(entries, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
