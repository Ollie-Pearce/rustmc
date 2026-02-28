#!/bin/bash

# Script to verify a single Rust file using the rustmc Docker container
# Usage: ./verify_single.sh <rust_file.rs> [unroll_bound]

if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <rust_file.rs> [unroll_bound]"
    echo "  rust_file.rs: Path to Rust file to verify"
    echo "  unroll_bound: Optional unroll bound for GenMC (default: 1)"
    echo "Example: $0 smoke_ported.rs 5"
    exit 1
fi

RUST_FILE="$1"
UNROLL_BOUND=${2:-1}  # Default to 1 if not provided

if [ ! -f "$RUST_FILE" ]; then
    echo "Error: File '$RUST_FILE' not found!"
    exit 1
fi

echo "Verifying: $RUST_FILE (unroll bound: $UNROLL_BOUND)"
echo "=========================================="
echo ""

bash -c "./verify-nomangles.sh $RUST_FILE $UNROLL_BOUND"

echo ""
echo "=========================================="
echo "Verification complete!"
echo "Results saved to:"
echo "  - test_traces/$(basename "$RUST_FILE" .rs)/"
echo "  - test_results/$(basename "$RUST_FILE" .rs)_all_summary.txt"
