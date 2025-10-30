#!/usr/bin/env python3
"""
rename_duplicate_rust_tests.py

Walks the current directory, finds Rust test functions marked with #[test],
and if a test name has been seen before, renames subsequent ones by appending
an incremental suffix: _1, _2, ...

Example: two #[test] fns named `test_cache` become `test_cache` and `test_cache_1`.

Usage:
  python3 rename_duplicate_rust_tests.py
"""

import os
import re
import sys
from pathlib import Path

ROOT = Path(".")

# Matches a Rust function declaration line and captures the function name token.
FN_RE = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\b')

def is_attribute_line(s: str) -> bool:
    # True if the trimmed line starts with an attribute like #[...]
    t = s.lstrip()
    return t.startswith("#[")

def process_file(path: Path, seen_names: dict) -> bool:
    """
    Processes a single .rs file.
    If it performs any renames, writes the file in-place and returns True.
    """
    try:
        original = path.read_text(encoding="utf-8")
    except Exception as e:
        print(f"SKIP {path}: {e}", file=sys.stderr)
        return False

    lines = original.splitlines(keepends=True)
    out_lines = []
    changed = False

    waiting_for_fn_after_test_attr = False

    for line in lines:
        line_out = line

        # Detect #[test] attribute; keep the flag until we encounter the next fn decl.
        if "#[test" in line:
            waiting_for_fn_after_test_attr = True
            out_lines.append(line_out)
            continue

        if waiting_for_fn_after_test_attr:
            # Allow stacked attributes (e.g., #[should_panic]) and blank/whitespace lines
            if is_attribute_line(line) or line.strip() == "":
                out_lines.append(line_out)
                continue

            # Try to capture fn name on this line
            m = FN_RE.search(line)
            if m:
                old_name = m.group(1)
                count = seen_names.get(old_name, 0)
                if count == 0:
                    seen_names[old_name] = 1
                else:
                    new_name = f"{old_name}_{count}"
                    seen_names[old_name] = count + 1
                    # Replace only the function name token occurrence
                    start, end = m.span(1)
                    line_out = line[:start] + new_name + line[end:]
                    changed = True
                    # Optionally: print what changed
                    print(f"{path}: {old_name} -> {new_name}")
                waiting_for_fn_after_test_attr = False
                out_lines.append(line_out)
                continue
            else:
                # Non-attribute line without an fn; attribute no longer applies
                waiting_for_fn_after_test_attr = False
                out_lines.append(line_out)
                continue

        # Default: passthrough
        out_lines.append(line_out)

    if changed:
        try:
            path.write_text("".join(out_lines), encoding="utf-8")
        except Exception as e:
            print(f"ERROR writing {path}: {e}", file=sys.stderr)
            return False

    return changed

def main():
    seen_names = {}  # global across all files
    changed_any = False

    for dirpath, dirnames, filenames in os.walk(ROOT):
        # Skip common target/build directories
        parts = set(Path(dirpath).parts)
        if "target" in parts or ".git" in parts:
            continue
        for fn in filenames:
            if fn.endswith(".rs"):
                p = Path(dirpath) / fn
                if process_file(p, seen_names):
                    changed_any = True

    if not changed_any:
        print("No duplicates found or no changes required.")

if __name__ == "__main__":
    main()
