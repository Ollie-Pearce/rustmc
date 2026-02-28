#!/usr/bin/env python3
"""
Collects experiment 2 results from test_traces/ and test_results/ directories
and produces a CSV summary file.
"""

import csv
import re
import os
import sys
from pathlib import Path


# Error classification patterns (same order as verify_tests)
ERROR_PATTERNS = [
    ("success", "Verification complete. No errors were detected."),
    ("expected_panic", "Expected panic occurred"),
    ("unexpected_panic", "Thread panicked"),
    ("uninitialised_heap_read", "Attempt to read from uninitialized memory"),
    ("no_entry_point", "ERROR: Could not find program's entry point function!"),
    ("external_function", "ERROR: Tried to execute an unknown external function:"),
    ("atomic_rmw", "visitAtomicRMWInst"),
    ("external_global", "LLVM ERROR: Could not resolve external global address:"),
    ("unsupported_syscall", "Calling external var arg function"),
    ("segfault", "timeout: the monitored command dumped core"),
    ("timeout", "TIMEOUT"),
]

# Patterns that count as a successful verification (not a crash)
SUCCESS_STATUSES = {"success", "expected_panic"}


def classify_trace(content):
    """Classify a trace file into a status category and extract metadata."""
    matched = []
    for tag, pattern in ERROR_PATTERNS:
        if pattern in content:
            matched.append(tag)

    # Extract executions explored
    exec_match = re.search(
        r"Number of complete executions explored:\s*(\d+)", content
    )
    executions = int(exec_match.group(1)) if exec_match else 0

    # Extract wall-clock time
    time_match = re.search(r"Total wall-clock time:\s*([\d.]+)s", content)
    wall_time = float(time_match.group(1)) if time_match else 0.0

    # Determine primary status
    if "success" in matched:
        status = "SUCCESS"
    elif "expected_panic" in matched:
        status = "EXPECTED_PANIC"
    elif "timeout" in matched:
        status = "TIMEOUT"
    elif "unexpected_panic" in matched:
        status = "UNEXPECTED_PANIC"
    elif matched:
        status = "CRASH"
    else:
        status = "CRASH"

    # Determine error category for crash details
    error_category = ""
    for tag in matched:
        if tag not in ("success", "expected_panic", "timeout"):
            error_category = tag
            break

    return {
        "status": status,
        "executions": executions,
        "wall_time_s": wall_time,
        "error_category": error_category,
        "matched_tags": matched,
    }


def collect_results(traces_dir):
    """Walk test_traces/ and classify every trace file."""
    traces_path = Path(traces_dir)
    rows = []

    if not traces_path.exists():
        print(f"Error: {traces_path} does not exist", file=sys.stderr)
        sys.exit(1)

    for crate_dir in sorted(traces_path.iterdir()):
        if not crate_dir.is_dir():
            continue

        crate_name = crate_dir.name

        for trace_file in sorted(crate_dir.iterdir()):
            if not trace_file.is_file() or not trace_file.name.endswith(
                "_verification.txt"
            ):
                continue

            test_name = trace_file.name.removesuffix("_verification.txt")
            content = trace_file.read_text(errors="replace")
            info = classify_trace(content)

            rows.append(
                {
                    "crate": crate_name,
                    "test_name": test_name,
                    "status": info["status"],
                    "executions_explored": info["executions"],
                    "wall_time_s": f"{info['wall_time_s']:.2f}",
                    "error_category": info["error_category"],
                }
            )

    return rows


def write_csv(rows, output_path):
    """Write results to a CSV file."""
    fieldnames = [
        "crate",
        "test_name",
        "status",
        "executions_explored",
        "wall_time_s",
        "error_category",
    ]

    with open(output_path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def print_summary(rows):
    """Print a human-readable summary."""
    total = len(rows)
    successes = sum(1 for r in rows if r["status"] == "SUCCESS")
    expected_panics = sum(1 for r in rows if r["status"] == "EXPECTED_PANIC")
    panics = sum(1 for r in rows if r["status"] == "UNEXPECTED_PANIC")
    timeouts = sum(1 for r in rows if r["status"] == "TIMEOUT")
    crashes = sum(1 for r in rows if r["status"] == "CRASH")

    print()
    print("=" * 40)
    print("SUMMARY")
    print("=" * 40)
    print(f"Total tests: {total}")
    print(f"Successful verifications: {successes}")
    print(f"Expected panics: {expected_panics}")
    print(f"Unexpected panics: {panics}")
    print(f"Timeouts: {timeouts}")
    print(f"Crashes: {crashes}")

    # Per-crate breakdown
    crates = sorted(set(r["crate"] for r in rows))
    print()
    print(f"{'Crate':<35} {'Total':>6} {'Pass':>6} {'XPanic':>6} {'UPanic':>6} {'T/O':>6} {'Crash':>6}")
    print("-" * 84)
    for crate in crates:
        crate_rows = [r for r in rows if r["crate"] == crate]
        c_total = len(crate_rows)
        c_success = sum(1 for r in crate_rows if r["status"] == "SUCCESS")
        c_expected_panic = sum(1 for r in crate_rows if r["status"] == "EXPECTED_PANIC")
        c_panic = sum(1 for r in crate_rows if r["status"] == "UNEXPECTED_PANIC")
        c_timeout = sum(1 for r in crate_rows if r["status"] == "TIMEOUT")
        c_crash = sum(1 for r in crate_rows if r["status"] == "CRASH")
        print(f"{crate:<35} {c_total:>6} {c_success:>6} {c_expected_panic:>6} {c_panic:>6} {c_timeout:>6} {c_crash:>6}")


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Collect experiment 2 results into a CSV"
    )
    parser.add_argument(
        "--traces-dir",
        default="test_traces",
        help="Path to test_traces directory (default: test_traces)",
    )
    parser.add_argument(
        "--output",
        default="experiment_results.csv",
        help="Output CSV file (default: experiment_results.csv)",
    )
    args = parser.parse_args()

    rows = collect_results(args.traces_dir)

    if not rows:
        print("No trace files found.", file=sys.stderr)
        sys.exit(1)

    write_csv(rows, args.output)
    print(f"Results saved to: {args.output}")
    print_summary(rows)


if __name__ == "__main__":
    main()
