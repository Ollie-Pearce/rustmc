#!/bin/bash
set -euo pipefail

dir="${1:-}"
[ -n "$dir" ] || { echo "Usage: $0 <directory-or-crate>" >&2; exit 1; }

# Normalise trace dir
case "$dir" in
  test_traces/*) ;;
  *) dir="test_traces/$dir" ;;
esac
[ -d "$dir" ] || { echo "Error: directory '$dir' not found" >&2; exit 1; }

crate="$(basename "$dir")"

# Resolve repo root and IR dir (override with TARGET_IR_ROOT if set)
if command -v git >/dev/null 2>&1 && git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  repo_root="$(git rev-parse --show-toplevel)"
else
  repo_root="$(pwd)"
fi
DEFAULT_IR_ROOT="$repo_root/verify_test_benchmarks/$crate/target-ir/debug/deps"
TARGET_IR_ROOT="${TARGET_IR_ROOT:-$DEFAULT_IR_ROOT}"
[ -d "$TARGET_IR_ROOT" ] || { echo "Error: IR directory '$TARGET_IR_ROOT' not found" >&2; exit 1; }

# Extract unknown external function symbols, normalise, and count
symbols="$(
  LC_ALL=C grep -Rhao --binary-files=text 'unknown external function: .*GenMC' "$dir" \
  | sed -E 's/^.*unknown external function:[[:space:]]*//; s/[[:space:]]*GenMC$//; s/[[:space:]]+$//' \
  | awk 'NF{cnt[$0]++} END{for (s in cnt) printf "%d\t%s\n", cnt[s], s}' \
  | sort -nr -k1,1
)"

# Nothing to report if no externals found in traces
[ -n "$symbols" ] || { echo "No unknown externals found in $dir"; exit 0; }

# For each symbol, find an *.ll line where define and the exact symbol occur on the SAME line.
# Match both @sym( and @"sym"( forms. Use perl with quotemeta to escape mangled names.
while IFS=$'\t' read -r count sym; do
  match_file="$(
    find "$TARGET_IR_ROOT" -type f -name '*.ll' -print0 \
    | xargs -0 -I{} env SYM="$sym" perl -ne '
        BEGIN { $re = quotemeta($ENV{"SYM"}); }
        if ( /^\s*define\b.*\@("?${re}"?)\(/ ) { print $ARGV, "\n"; exit }
      ' "{}" \
    | head -n1
  )"
  if [ -n "$match_file" ]; then
    status="In target-ir ($match_file)"
  else
    status="Not in target-ir"
  fi
  printf "%s | %s | %s\n" "$sym" "$count" "$status"
done <<< "$symbols"

# Files mentioning "entry point"
grep -RIl "entry point" "$dir" > no_entry_point || true
echo "Entries without 'entry point': $(wc -l < no_entry_point)"
printf "\n"
cat no_entry_point || true
echo "--------------------------------------------------------"

# Optional summary
summary_file="$repo_root/test_results/$(basename "$dir")_summary.txt"
[ -f "$summary_file" ] && cat "$summary_file" || echo "No summary file found for $(basename "$dir")"