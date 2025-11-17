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

import pdfplumber  # type: ignore

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
                cleaned = [row for row in cleaned if any(cell for cell in row)]
                for r_idx, row in enumerate(cleaned, start=1):
                    for c_idx, cell in enumerate(row, start=1):
                        key = c_idx + pidx * 16
                        try:
                            column = combined_table[key]
                        except KeyError:
                            column = {}
                            combined_table[key] = column
                        column[r_idx] = cell
                        combined_table[key] = column

    with open(out_file, "w") as fobj:
        json.dump(combined_table, fobj)

if __name__ == "__main__":
    main()
