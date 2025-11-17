#!/usr/bin/env python3
# pdf_to_csv_combined.py
#
# Extract all tables from a multi-page PDF into a single combined CSV.
# Output columns: page,table,row,col,text
#
# Usage:
#   python pdf_to_csv_combined.py "Pokemon Lazarus Documentation - Encounters.pdf" \
#       --out Pokemon_Lazarus_Encounters_all_tables_long.csv
#
# Requirements:
#   pip install pdfplumber
#
import json
import argparse
from pathlib import Path
import pprint
import re

import pdfplumber  # type: ignore

species_re = re.compile(r'^\s*(.*?)\s*\(([\d.]+%)\)\s*$')

def get_header_from_ridx(headers: dict, ridx: int) -> str:
    header_names = list(headers.keys())
    header_ridx = list(headers.values())
    if ridx > header_ridx[-1]:
        return header_names[-1]
    for start_ridx, end_ridx, header in zip(header_ridx[1:-1], header_ridx[2:], header_names[1:-1]):
        if ridx >= start_ridx and ridx < end_ridx:
            return header
    # No match, must be before first header
    return None

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("pdf", help="Path to the PDF file")
    ap.add_argument("--out", default=None, help="Path to the combined JSON output")
    args = ap.parse_args()

    pdf_path = Path(args.pdf)
    out_file = Path(args.out) if args.out else pdf_path.with_name(f"encounters.json")

    combined_table = {}
    
    with pdfplumber.open(pdf_path) as pdf:
        for pidx, page in enumerate(pdf.pages, start=1):
            tables = page.extract_tables() or []
            for tidx, table in enumerate(tables, start=1):
                # Normalize and skip fully empty rows
                cleaned = [[(c or "").strip() for c in row] for row in table]
                for r_idx, row in enumerate(cleaned, start=1):
                    for c_idx, cell in enumerate(row, start=1):
                        key = c_idx - 1 + (pidx - 1) * 16
                        try:
                            column = combined_table[key]
                        except KeyError:
                            column = {}
                            combined_table[key] = column
                        column[r_idx] = cell
                        combined_table[key] = column

    headers = {
        header[:-1]: rowid
        for (rowid, header) in combined_table[0].items()
        if header
    }
    cur_location = None
    encounters = {}
    for c_idx in combined_table.keys():
        if c_idx == 0: continue
        location = combined_table[c_idx][headers["Location"]]
        if location:
            cur_location = location
            encounters[location] = {}
            for row_idx in combined_table[c_idx]:
                header = get_header_from_ridx(headers, row_idx)
                if not header: continue
                cell = combined_table[c_idx][row_idx]
                cell_match = species_re.match(cell)
                if not cell_match: continue
                try:
                    enc_cat = encounters[location][header]
                except KeyError:
                    enc_cat = []
                    encounters[location][header] = enc_cat
                entry = {"Pokemon": cell_match.group(1), "Rate": cell_match.group(2)}
                if header == "Fishing":
                    # Add the ridx to the entry for reference later
                    entry["ridx"] = row_idx
                enc_cat.append(entry)

        elif cur_location:
            try:
                fishing = encounters[cur_location]["Fishing"]
            except KeyError:
                continue
            for entry in fishing:
                rod_type = combined_table[c_idx][entry.pop("ridx")]
                entry["Rod"] = rod_type

    with open(out_file, "w") as fobj:
        json.dump(encounters, fobj)

if __name__ == "__main__":
    main()
