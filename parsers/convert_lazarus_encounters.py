#!/usr/bin/env python3
# convert_lazarus_encounters_simple.py
# Usage:
#   python convert_lazarus_encounters_simple.py "Pokemon Lazarus Documentation - Encounters.pdf" --json out/encounters.json --csv out/encounters.csv
#
# Output JSON schema (simplified):
# {
#   "regions": ["Lazarus"],
#   "locations": [
#     {
#       "name": "Acrisia City",
#       "notes": null,
#       "methods": {
#         "grass_day": [ {"species":"Pikipek","rate":20.0}, ... ],
#         "grass_night": [ ... ],
#         "old_rod": [ ... ],
#         "good_rod": [ ... ],
#         "super_rod": [ ... ],
#         "surf": [ ... ],
#         "underwater": [ ... ]
#       }
#     }
#   ],
#   "meta": { ... }
# }
#
# CSV columns:
# location,method,species,rate_percent,notes
#
# This file focuses on the simplified model you requested.
# Table extraction is doc-specific; the mapping hooks are provided so you can adapt
# quickly if the PDF formatting changes slightly.

import sys, os, re, json, csv, argparse, datetime
from collections import defaultdict

try:
    import pdfplumber  # type: ignore
    HAS_PDFPLUMBER = True
except Exception:
    HAS_PDFPLUMBER = False

METHOD_ALIASES = {
    "Land Encounters (Day)": "grass_day",
    "Land (Day)": "grass_day",
    "Land Day": "grass_day",
    "Land Encounters (Night)": "grass_night",
    "Land (Night)": "grass_night",
    "Land Night": "grass_night",
    "Fishing": "fishing",
    "Surfing": "surf",
    "Underwater": "underwater",
    "Old Rod": "old_rod",
    "Good Rod": "good_rod",
    "Super Rod": "super_rod",
    # some PDFs might label "Sea encounters" as Surfing equivalent
    "Sea Encounters": "surf",
}

SPECIES_RATE_RE = re.compile(r"^(?P<name>.+?)\s*\((?P<rate>[\d.]+)%\)\s*$")

def build_output_struct():
    return {
        "regions": ["Lazarus"],
        "locations": [],
        "meta": {
            "source": "Pokemon Lazarus Encounters PDF",
            "generated_at": datetime.datetime.utcnow().isoformat() + "Z",
            "normalization": {
                "rate_unit": "percent",
                "method_aliases": METHOD_ALIASES,
                "schema": "simplified:v1"
            }
        }
    }

def parse_species_rate(cell: str):
    cell = (cell or "").strip()
    if not cell or cell.upper() == "N/A":
        return None
    m = SPECIES_RATE_RE.match(cell)
    if not m:
        # Sometimes rod qualifier is glued (e.g., "Magikarp (70%) Old Rod")
        m2 = re.match(r"^(?P<name>.+?)\s*\((?P<rate>[\d.]+)%\)\s+.*$", cell)
        if not m2:
            return None
        name = m2.group("name").strip()
        rate = float(m2.group("rate"))
        return {"species": name, "rate": rate}
    return {"species": m.group("name").strip(), "rate": float(m.group("rate"))}

def extract_tables(pdf_path):
    """Return list of pages, each page is a list of tables, each table is list[list[str]]."""
    pages = []
    with pdfplumber.open(pdf_path) as pdf:
        for page in pdf.pages:
            try:
                tables = page.extract_tables() or []
            except Exception:
                tables = []
            cleaned = []
            for t in tables:
                rows = [[(c or "").strip() for c in row] for row in t if any((c or "").strip() for c in row)]
                if rows:
                    cleaned.append(rows)
            pages.append(cleaned)
    return pages

def guess_locations_from_header(row_cells):
    """
    Given a header row like: ["Location:", "Acrisia City", "Bronze Fields (North)", ...]
    return the list of column-aligned location names (index 1..N).
    """
    if not row_cells: return []
    # find the "Location" keyword cell
    # Some tables put "Location:" as the first col; sometimes it's split.
    # We'll just skip the first cell and treat the rest as locations.
    if row_cells[0].lower().startswith("location"):
        return [c for c in row_cells[1:] if c]
    # otherwise try to find "Location" anywhere and take after
    for i, c in enumerate(row_cells):
        if c.lower().startswith("location"):
            return [x for x in row_cells[i+1:] if x]
    return []

def normalize_method_label(lbl: str):
    lbl = (lbl or "").strip()
    # strip any trailing colon
    if lbl.endswith(":"):
        lbl = lbl[:-1]
    return METHOD_ALIASES.get(lbl, lbl)

def map_page_tables_to_locations(tables):
    """
    Heuristic mapper for a page:
    - Expect the first table to include a header row with "Location:" + N location columns.
    - Subsequent blocks label sections like "Land Encounters (Day):", etc.
    - Then rows contain N columns where each column cell is "Species (Rate%)" or "N/A".
    Returns: dict { location_name: { method_key: [ {species, rate}, ... ] } }
    """
    loc_map = {}
    if not tables:
        return loc_map

    # Find a header-like row with "Location"
    locations = []
    for table in tables:
        for row in table:
            if any(c.lower().startswith("location") for c in row if c):
                locations = guess_locations_from_header(row)
                break
        if locations:
            break
    if not locations:
        return loc_map

    # Initialize structure
    for loc in locations:
        loc_map[loc] = defaultdict(list)

    current_method = None

    # pass through all tables/rows and collect under current method
    for table in tables:
        for row in table:
            # Method label row? (first cell or entire row is a known method key)
            first = row[0] if row else ""
            norm = normalize_method_label(first)
            if norm in ("grass_day", "grass_night", "fishing", "surf", "underwater"):
                current_method = norm
                continue
            # Sometimes Fishing sub-rows specify rod in cells; we still store under fishing here.
            if current_method is None:
                continue

            # Collect column cells aligned with locations
            # Align by taking last len(locations) cells in the row to avoid leading labels.
            cells = row[-len(locations):] if len(row) >= len(locations) else row
            if len(cells) != len(locations):
                # skip malformed row
                continue
            for idx, cell in enumerate(cells):
                parsed = parse_species_rate(cell)
                if not parsed:
                    continue
                loc_map[locations[idx]][current_method].append(parsed)

    # Postprocess: split "fishing" into old/good/super if keywords present in the cell text
    # Our parser already strips "Old Rod"/etc; if you prefer keeping them, you can augment above.
    # For now we keep everything under fishing unless clearly disambiguated by text format.
    return loc_map

def assemble_output(loc_map):
    out = build_output_struct()
    for loc_name, methods in loc_map.items():
        # normalize keys and coerce to native dict
        norm_methods = {
            "grass_day": methods.get("grass_day", []),
            "grass_night": methods.get("grass_night", []),
            "old_rod": methods.get("old_rod", []),
            "good_rod": methods.get("good_rod", []),
            "super_rod": methods.get("super_rod", []),
            "surf": methods.get("surf", []),
            "underwater": methods.get("underwater", []),
        }
        # If "fishing" exists but no rod-specific buckets, put it under "old_rod/good_rod/super_rod" is unknown.
        # We'll keep a single "fishing" key only if present and others empty.
        if methods.get("fishing") and not (norm_methods["old_rod"] or norm_methods["good_rod"] or norm_methods["super_rod"]):
            norm_methods["fishing"] = methods["fishing"]

        out["locations"].append({
            "name": loc_name,
            "notes": None,
            "methods": norm_methods
        })
    return out

def write_csv(out_json, csv_path):
    rows = []
    for loc in out_json["locations"]:
        for method, entries in loc["methods"].items():
            if not entries:
                continue
            for e in entries:
                rows.append({
                    "location": loc["name"],
                    "method": method,
                    "species": e["species"],
                    "rate_percent": e["rate"],
                    "notes": loc.get("notes") or ""
                })
    with open(csv_path, "w", newline="", encoding="utf-8") as f:
        if rows:
            w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
            w.writeheader()
            w.writerows(rows)
        else:
            w = csv.DictWriter(f, fieldnames=["location","method","species","rate_percent","notes"])
            w.writeheader()

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("pdf", help="Path to 'Pokemon Lazarus Documentation - Encounters.pdf'")
    ap.add_argument("--json", default="encounters.json", help="Output JSON path")
    ap.add_argument("--csv", default="encounters.csv", help="Output CSV path")
    args = ap.parse_args()

    if not HAS_PDFPLUMBER:
        print("Warning: pdfplumber not installed. Install with `pip install pdfplumber` for best results.")
        print("This script relies on table extraction; without it, results may be empty.")
        # we still try to proceed to allow basic structure
    pages = extract_tables(args.pdf) if HAS_PDFPLUMBER else []
    combined_loc_map = defaultdict(lambda: defaultdict(list))

    for tables in pages:
        loc_map = map_page_tables_to_locations(tables)
        for loc, methods in loc_map.items():
            for m, entries in methods.items():
                combined_loc_map[loc][m].extend(entries)

    out = assemble_output(combined_loc_map)
    with open(args.json, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=2, ensure_ascii=False)
    write_csv(out, args.csv)
    print(f"Wrote JSON to {args.json}")
    print(f"Wrote CSV to {args.csv}")

if __name__ == "__main__":
    main()
