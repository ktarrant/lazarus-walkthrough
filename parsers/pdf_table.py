from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterator, Tuple

import pdfplumber  # type: ignore


@dataclass
class ColumnarPDFTable:
    """Represents a PDF table where each page extends the table horizontally."""

    columns: Dict[int, Dict[int, str]]
    max_row_index: int

    @classmethod
    def from_pdf(cls, pdf_path: Path, columns_per_page: int = 16) -> "ColumnarPDFTable":
        """Parse the PDF and flatten every page's tables into column buckets."""
        columns: Dict[int, Dict[int, str]] = {}
        max_row = 0
        with pdfplumber.open(pdf_path) as pdf:
            for pidx, page in enumerate(pdf.pages, start=1):
                tables = page.extract_tables() or []
                for table in tables:
                    normalized = [[(cell or "").strip() for cell in row] for row in table]
                    for r_idx, row in enumerate(normalized, start=1):
                        max_row = max(max_row, r_idx)
                        for c_idx, cell in enumerate(row, start=1):
                            col_idx = c_idx - 1 + (pidx - 1) * columns_per_page
                            columns.setdefault(col_idx, {})[r_idx] = cell
        return cls(columns, max_row)

    def iter_columns(self) -> Iterator[Tuple[int, Dict[int, str]]]:
        """Yield columns in ascending order of their index."""
        for idx in sorted(self.columns.keys()):
            yield idx, self.columns[idx]

    def get_column(self, idx: int) -> Dict[int, str]:
        return self.columns.get(idx, {})

    def header_map(self, column_idx: int = 0, strip_trailing_colon: bool = True) -> Dict[str, int]:
        headers = {}
        column = self.get_column(column_idx)
        for row_id, text in column.items():
            if not text:
                continue
            label = text[:-1] if strip_trailing_colon and text.endswith(":") else text
            headers[label] = row_id
        return headers

    def cell(self, column_idx: int, row_idx: int) -> str:
        return self.columns.get(column_idx, {}).get(row_idx, "")

    def max_row(self) -> int:
        return self.max_row_index
