#!/usr/bin/env python3
"""Convert the Lazarus Pokémon Data CSV into structured JSON for card generation."""

import argparse
import csv
import json
import unicodedata
from pathlib import Path
from typing import Any, Dict, List, Optional


def slugify(name: str) -> str:
    normalized = unicodedata.normalize("NFKD", name)
    slug = []
    for ch in normalized.lower():
        if unicodedata.category(ch) == "Mn":
            continue
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
        # Strip known numeric noise like "0.1" or "0.2" that appear in the sheet
        if raw.replace(".", "", 1).isdigit():
            continue
        if is_level:
            entry = parse_level_move(raw)
            if entry:
                moves.append(entry)
        else:
            moves.append(raw)
    return moves


def find_indices(header: List[str], name: str) -> List[int]:
    return [i for i, val in enumerate(header) if val.strip() == name]


def parse_row(
    row: List[str],
    move_sections: List[int],
    tutor_end_idx: int,
    type_indices: List[int],
    stat_indices: List[int],
    ability_indices: List[int],
    location_idx: int,
    egg_group_indices: List[int],
) -> Dict[str, Any]:
    name = row[0].strip()
    if not name:
        return {}

    types = [row[i].strip() for i in type_indices if i < len(row) and row[i].strip()]

    stats_keys = ["hp", "attack", "defense", "sp_atk", "sp_def", "speed"]
    stats: Dict[str, int] = {}
    for key, idx in zip(stats_keys, stat_indices):
        value = row[idx].strip() if idx < len(row) else ""
        stats[key] = int(value) if value.isdigit() else 0

    level_start, egg_start, tm_start, tutor_start = move_sections
    level_range = row[level_start:egg_start]
    egg_moves_range = row[egg_start:tm_start]
    tm_range = row[tm_start:tutor_start]
    tutor_range = row[tutor_start:tutor_end_idx]

    egg_groups = []
    for idx in egg_group_indices:
        if idx < len(row):
            val = row[idx].strip()
            if val and val not in egg_groups:
                egg_groups.append(val)

    evolution_line = [cell.strip() for cell in row[tutor_end_idx:] if cell.strip()]

    abilities = {
        "primary": row[ability_indices[0]].strip() if ability_indices else "",
        "secondary": row[ability_indices[1]].strip() if len(ability_indices) > 1 else "",
        "hidden": row[ability_indices[2]].strip() if len(ability_indices) > 2 else "",
    }

    held = row[30].strip() if len(row) > 30 else ""
    if len(row) > 31 and row[31].strip():
        held = held or row[31].strip()

    return {
        "name": name,
        "slug": slugify(name),
        "dex": int(row[2]) if len(row) > 2 and row[2].strip().isdigit() else None,
        "types": types,
        "stats": stats,
        "abilities": abilities,
        "evolution": row[29].strip() if len(row) > 29 else "",
        "held_item": held,
        "location": row[location_idx].strip() if len(row) > location_idx else "",
        "egg_groups": egg_groups,
        "level_up_moves": parse_move_list(level_range, is_level=True),
        "egg_moves": parse_move_list(egg_moves_range),
        "tm_moves": parse_move_list(tm_range),
        "tutor_moves": parse_move_list(tutor_range),
        "evolves_from": None,
        "evolves_to": [],
        "evolution_line": evolution_line,
    }


def infer_evolution_links(entries: List[Dict[str, Any]]) -> None:
    slug_map = {e["slug"]: e for e in entries}
    for entry in entries:
        line = entry.get("evolution_line", [])
        names = [cell for cell in line if not cell.startswith("⇒") and "Lv." not in cell and cell.lower() not in {"day", "night"}]
        slugs = [slugify(n) for n in names]
        if not slugs or entry["slug"] not in slugs:
            continue
        idx = slugs.index(entry["slug"])
        if idx > 0:
            entry["evolves_from"] = slugs[idx - 1]
        if idx + 1 < len(slugs):
            entry.setdefault("evolves_to", [])
            if slugs[idx + 1] not in entry["evolves_to"]:
                entry["evolves_to"].append(slugs[idx + 1])
        # also add reverse linkage for parent if present
        if idx > 0 and slugs[idx - 1] in slug_map:
            parent = slug_map[slugs[idx - 1]]
            parent.setdefault("evolves_to", [])
            if entry["slug"] not in parent["evolves_to"]:
                parent["evolves_to"].append(entry["slug"])
    # cleanup helper field and dedupe evolves_to
    for entry in entries:
        entry.pop("evolution_line", None)
        if "evolves_to" in entry:
            entry["evolves_to"] = sorted(set(entry["evolves_to"]))


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Convert Lazarus CSV data into JSON for card generation"
    )
    parser.add_argument("csv", help="Path to Lazarus Data - Pokemon Data.csv")
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

    header = rows[1]
    type_indices = [3, 4]
    stat_indices = [5, 6, 7, 8, 9, 10]
    ability_indices = [26, 27, 28]
    location_idx = header.index("Location") if "Location" in header else 32
    egg_group_indices = [33, 34]

    move_starts = [idx for idx, val in enumerate(header) if val.strip() == "Move 1"]
    if len(move_starts) < 4:
        raise SystemExit("Expected at least 4 move sections (level, egg, TM/HM, tutor)")
    level_start, egg_start, tm_start, tutor_start = move_starts[:4]

    evo_line_idx = header.index("Evolution Line") if "Evolution Line" in header else len(header)
    tutor_end_idx = header.index("Sprite #") if "Sprite #" in header else evo_line_idx

    entries = []
    for row in rows[2:]:
        entry = parse_row(
            row,
            [level_start, egg_start, tm_start, tutor_start],
            tutor_end_idx,
            type_indices,
            stat_indices,
            ability_indices,
            location_idx,
            egg_group_indices,
        )
        if entry:
            entries.append(entry)

    infer_evolution_links(entries)

    out_path.write_text(json.dumps(entries, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
