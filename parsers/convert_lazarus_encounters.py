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
import argparse
import json
import re
from pathlib import Path

from pdf_table import ColumnarPDFTable  # type: ignore

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

    table = ColumnarPDFTable.from_pdf(pdf_path, columns_per_page=16)

    headers = {
        header: rowid for header, rowid in table.header_map().items()
        if header
    }
    cur_location = None
    encounters = {}
    for c_idx, column in table.iter_columns():
        if c_idx == 0:
            continue
        location = column.get(headers["Location"], "")
        if location:
            cur_location = location
            encounters[location] = {}
            for row_idx, cell in column.items():
                header = get_header_from_ridx(headers, row_idx)
                if not header:
                    continue
                cell_match = species_re.match(cell)
                if not cell_match:
                    continue
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
                rod_type = table.cell(c_idx, entry.pop("ridx"))
                entry["Rod"] = rod_type

    with open(out_file, "w") as fobj:
        json.dump(encounters, fobj)

if __name__ == "__main__":
    main()
