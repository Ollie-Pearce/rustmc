#!/usr/bin/env bash
set -euo pipefail

all_crates=("thread_local" "boxcar" "state" "arc-swap" "arcstr" "ringbuf" "try-lock" "seize" "parking" "atomic_float" "archery" "spin" "parking_lot" "scc")
small_crates=("thread_local" "boxcar" "archery" "atomic_float" "try-lock")

crates=("${all_crates[@]}")
if [[ "${1:-}" == "--run-small" ]]; then
  crates=("${small_crates[@]}")
  shift
fi

mkdir -p crate_timings

for crate in "${crates[@]}"; do
  /usr/bin/time -v -o "crate_timings/${crate}_timing.txt" ./verify_crate "$crate"
done

# Collect results into CSV
python3 collect_results.py
