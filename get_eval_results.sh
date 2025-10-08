#!/bin/sh
set -eu

dir="${1:-}"
if [ -z "$dir" ]; then
  echo "Usage: $0 <directory>" >&2
  exit 1
fi

# Prepend test_traces if not already part of the path
if [ "${dir#test_traces/}" = "$dir" ]; then
  dir="test_traces/$dir"
fi

# Verify directory exists
if [ ! -d "$dir" ]; then
  echo "Error: directory '$dir' not found" >&2
  exit 1
fi

# Extract unknown external function symbols and count
grep -Rhao --binary-files=text 'unknown external function: .*GenMC' "$dir" \
  | sed 's/^.*unknown external function:[[:space:]]*//' \
  | sed 's/[[:space:]]*GenMC$//' \
  | sed 's/[[:space:]]\+$//' \
  | sort \
  | uniq -c \
  | sort -nr \
  | awk '{c=$1; $1=""; sub(/^ /,""); printf "%s | %d\n", $0, c}'

# Write files containing "entry point" to no_entry_point, count, and print summary
grep -RIl "entry point" "$dir" > no_entry_point || true
echo "Entries without 'entry point': $(wc -l < no_entry_point)"

# Print summary file if exists
summary_file="test_results/$(basename "$dir")_summary.txt"
if [ -f "$summary_file" ]; then
  cat "$summary_file"
else
  echo "No summary file found for $(basename "$dir")"
fi