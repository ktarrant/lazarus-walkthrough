#!/usr/bin/env python3
"""Parse the Important Item Locations PDF into structured JSON."""

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Tuple

from pdf_table import ColumnarPDFTable  # type: ignore


@dataclass
class FieldSpec:
    name: str
    column_idx: int


@dataclass
class CategoryBlock:
    name: str
    fields: List[FieldSpec]
    data_start: int
    data_end: int

    def to_records(self, table: ColumnarPDFTable) -> List[Dict[str, str]]:
        rows = []
        for row_idx in range(self.data_start, self.data_end + 1):
            record = {}
            has_value = False
            for field in self.fields:
                value = table.cell(field.column_idx, row_idx).strip()
                if value:
                    has_value = True
                record[field.name] = value
            if has_value:
                rows.append(record)
        return rows


def discover_column_groups(table: ColumnarPDFTable) -> List[Tuple[int, int]]:
    header_row = 3
    columns = sorted(table.columns.keys())
    starts = []
    for idx in columns:
        header = table.cell(idx, header_row).strip()
        prev_header = table.cell(idx - 1, header_row).strip() if idx - 1 in table.columns else ""
        if header and not prev_header:
            starts.append(idx)
    groups = []
    for pos, start in enumerate(starts):
        end = starts[pos + 1] if pos + 1 < len(starts) else (max(columns) + 1)
        groups.append((start, end))
    return groups


def discover_categories(table: ColumnarPDFTable) -> List[CategoryBlock]:
    categories: List[CategoryBlock] = []
    max_row = table.max_row()
    for start_col, end_col in discover_column_groups(table):
        column = table.get_column(start_col)
        starts = []
        for row_idx in sorted(column.keys()):
            title = column[row_idx].strip()
            if not title:
                continue
            header_row = row_idx + 1
            header_indicator = table.cell(start_col, header_row).strip()
            if not header_indicator.endswith(":"):
                continue

            fields = []
            for col_idx in range(start_col, end_col):
                header_text = table.cell(col_idx, header_row).strip()
                if header_text:
                    fields.append(FieldSpec(header_text.rstrip(":").strip(), col_idx))
            if not fields:
                continue
            starts.append((row_idx, title, fields))

        for idx, (row_idx, title, fields) in enumerate(starts):
            next_row = starts[idx + 1][0] if idx + 1 < len(starts) else max_row + 1
            data_start = row_idx + 2  # skip header row
            categories.append(
                CategoryBlock(
                    name=title,
                    fields=fields,
                    data_start=data_start,
                    data_end=next_row - 1,
                )
            )
    return categories


def build_manifest(table: ColumnarPDFTable) -> Dict[str, List[Dict[str, str]]]:
    manifest = {}
    for category in discover_categories(table):
        manifest[category.name] = category.to_records(table)
    return manifest


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Parse the Important Item Locations PDF into JSON"
    )
    parser.add_argument("pdf", help="Path to the Important Item Locations PDF")
    parser.add_argument(
        "--json",
        help="Destination for the JSON manifest",
        default="important-items.json",
    )
    args = parser.parse_args()

    pdf_path = Path(args.pdf)
    table = ColumnarPDFTable.from_pdf(pdf_path, columns_per_page=32)
    manifest = build_manifest(table)

    out_path = Path(args.json)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
