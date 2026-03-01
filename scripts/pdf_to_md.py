#!/usr/bin/env python3
"""Convert a PDF datasheet to a single markdown file using PyMuPDF.

Usage:
    uv run --with pymupdf python3 scripts/pdf_to_md.py Datasheets/max7301.pdf

Output goes to the same directory as the input PDF with a .md extension,
or specify --output PATH explicitly.
"""

import argparse
import pymupdf
from pathlib import Path


def extract_page_text(page: pymupdf.Page) -> str:
    """Extract text from a single page, preserving block structure."""
    blocks = page.get_text("blocks")  # (x0, y0, x1, y1, text, block_no, block_type)
    lines = []
    for b in blocks:
        if b[6] != 0:  # skip image blocks
            continue
        text = b[4].strip()
        if text:
            lines.append(text)
    return "\n\n".join(lines)


def pdf_to_markdown(pdf_path: str, output_path: str | None = None) -> None:
    pdf = Path(pdf_path)
    if not pdf.exists():
        raise FileNotFoundError(f"PDF not found: {pdf}")

    doc = pymupdf.open(str(pdf))

    # Derive output path
    if output_path is None:
        out = pdf.with_suffix(".md")
    else:
        out = Path(output_path)

    # Extract title from first page or metadata
    title = doc.metadata.get("title", "") if doc.metadata else ""
    if not title:
        title = pdf.stem

    md_lines = [f"# {title}\n"]
    md_lines.append(f"Source: `{pdf.name}` ({len(doc)} pages)\n")
    md_lines.append("---\n")

    for page_num in range(len(doc)):
        page = doc[page_num]
        text = extract_page_text(page)
        if text:
            md_lines.append(f"<!-- Page {page_num + 1} -->\n")
            md_lines.append(text)
            md_lines.append("")

    out.write_text("\n".join(md_lines), encoding="utf-8")
    print(f"Converted {pdf.name} → {out.name} ({len(doc)} pages)")


def main():
    parser = argparse.ArgumentParser(description="Convert PDF datasheet to markdown")
    parser.add_argument("pdf", help="Path to PDF file")
    parser.add_argument("--output", "-o", help="Output markdown path (default: same dir, .md ext)")
    args = parser.parse_args()
    pdf_to_markdown(args.pdf, args.output)


if __name__ == "__main__":
    main()
