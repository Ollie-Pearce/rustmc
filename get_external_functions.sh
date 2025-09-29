#!/bin/sh
set -eu

dir="${1:-}"
if [ -z "$dir" ]; then
  echo "Usage: $0 <directory>" >&2
  exit 1
fi

# Extract the substring between "unknown external function:" and "GenMC",
# count occurrences, and print "symbol | count".
grep -Rhao --binary-files=text 'unknown external function: .*GenMC' "$dir" \
| sed 's/^.*unknown external function:[[:space:]]*//' \
| sed 's/[[:space:]]*GenMC$//' \
| sed 's/[[:space:]]\+$//' \
| sort \
| uniq -c \
| sort -nr \
| awk '{c=$1; $1=""; sub(/^ /,""); printf "%s | %d\n", $0, c}'