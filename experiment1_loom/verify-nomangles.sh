#!/bin/bash

if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <rust_file> [unroll_bound]"
    echo "  rust_file: Path to .rs file to verify"
    echo "  unroll_bound: Optional unroll bound for GenMC (default: 1)"
    echo "  This script will verify ALL #[no_mangle] functions in the file"
    exit 1
fi


TARGET_RUST_FILE=$1
UNROLL_BOUND=${2:-1}  # Default to 1 if not provided
DEPDIR=$(pwd)

# Find ALL #[no_mangle] functions in the file
ENTRY_FUNCTIONS=$(awk '
    BEGIN { seen_no_mangle = 0 }
    /^[[:space:]]*#\[no_mangle\]/ { seen_no_mangle = 1; next }
    seen_no_mangle && /^[[:space:]]*pub[[:space:]]+fn[[:space:]]+[a-zA-Z0-9_]+/ {
        match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)
        print m[1]
        seen_no_mangle = 0
    }
    seen_no_mangle && /^[[:space:]]*fn[[:space:]]+[a-zA-Z0-9_]+/ {
        match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)
        print m[1]
        seen_no_mangle = 0
    }
    /^[[:space:]]*$/ { seen_no_mangle = 0 }
' "$TARGET_RUST_FILE")

if [ -z "$ENTRY_FUNCTIONS" ]; then
    echo "ERROR: No #[no_mangle] functions found in $TARGET_RUST_FILE"
    exit 1
fi

echo "Found #[no_mangle] functions:"
echo "$ENTRY_FUNCTIONS"
echo ""

# Count total functions
TOTAL_FUNCTIONS=$(echo "$ENTRY_FUNCTIONS" | wc -l)
echo "Total functions to verify: $TOTAL_FUNCTIONS"
echo ""

# Extract filename without extension for output naming
FILENAME=$(basename "$TARGET_RUST_FILE" .rs)

# Create a temporary Cargo project to compile the single file
TEMP_PROJECT=$(mktemp -d)
cd "$TEMP_PROJECT"

# Initialize a new cargo project
cargo init --name "$FILENAME" --bin --edition 2021

# Copy the target Rust file to replace src/main.rs
cp "$DEPDIR/$TARGET_RUST_FILE" src/main.rs

# Create rust-toolchain.toml to specify RustMC toolchain
echo 'RustMC' > rust-toolchain

# Compile using the same approach as the original script
echo "Compiling $TARGET_RUST_FILE to LLVM bitcode..."

RUSTFLAGS="--emit=llvm-bc -C overflow-checks=off -C target-feature=-avx2 -C no-vectorize-slp -C no-vectorize-loops -C prefer-dynamic=no -C codegen-units=1 -C lto=no -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics -C passes=memcpyopt -Z mir-opt-level=0" cargo build --target x86_64-unknown-linux-gnu

if [ $? -ne 0 ]; then
    echo "ERROR: Rust compilation failed"
    cd "$DEPDIR"
    rm -rf "$TEMP_PROJECT"
    exit 1
fi

# Find the generated bitcode files (use absolute paths)
find "$(pwd)/target/x86_64-unknown-linux-gnu/debug/deps" -name "*.bc" -type f > "$DEPDIR/bitcode.txt"

cd "$DEPDIR"

if [ ! -s "$DEPDIR/bitcode.txt" ]; then
    echo "ERROR: No bitcode files generated"
    rm -rf "$TEMP_PROJECT"
    exit 1
fi

echo "Bitcode files:"
cat bitcode.txt

/usr/bin/llvm-link-18 --internalize -S --override=$DEPDIR/../override/my_pthread.ll -o combined_old.ll @bitcode.txt
/usr/bin/opt-18 -S -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.ll -o combined.ll

mkdir -p "test_traces/${FILENAME}"
mkdir -p "test_results"

# Check if combined.ll was created
if [ ! -f combined.ll ]; then
    echo "ERROR: combined.ll not found!"
    rm -rf "$TEMP_PROJECT"
    exit 1
fi

# Initialize summary file
SUMMARY_FILE="test_results/${FILENAME}_all_summary.txt"
echo "Verification Summary for $FILENAME" > "$SUMMARY_FILE"
echo "Generated on: $(date)" >> "$SUMMARY_FILE"
echo "Total functions: $TOTAL_FUNCTIONS" >> "$SUMMARY_FILE"
echo "========================================" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

# Counter for progress tracking
CURRENT=0
SUCCESS_COUNT=0
FAIL_COUNT=0


# DISABLED genmc flag:
#           --print-exec-graphs
# 	    --print-error-trace 
# Loop through each entry function
while IFS= read -r ENTRY_FUNCTION; do
    CURRENT=$((CURRENT + 1))

    echo " "
    echo "================= [$CURRENT/$TOTAL_FUNCTIONS] Verifying: $ENTRY_FUNCTION ================="
    echo " "

    /usr/bin/time -v -o "test_traces/${FILENAME}/${ENTRY_FUNCTION}_timing.txt" \
        timeout 1000s ../genmc --mixer \
            --transform-output=myout.ll \
            --disable-function-inliner \
            --disable-assume-propagation \
            --disable-load-annotation \
            --disable-confirmation-annotation \
            --disable-spin-assume \
            --program-entry-function="$ENTRY_FUNCTION" \
            --disable-estimation \
            --disable-stop-on-system-error \
            --unroll="$UNROLL_BOUND" \
            combined.ll > "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt" 2>&1

    GENMC_EXIT=$?

    if [ $GENMC_EXIT -eq 124 ]; then
        echo "TIMEOUT" >> "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt"
    fi

    echo "GenMC exit code: $GENMC_EXIT"
    echo "Last 30 lines of verification output:"
    tail -30 "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt"
    echo " "

    # Analyze results
    cd test_traces/${FILENAME}/

    success_search_string="Verification complete. No errors were detected."

    echo "Function: $ENTRY_FUNCTION" >> "../../$SUMMARY_FILE"

    if grep -q "$success_search_string" "${ENTRY_FUNCTION}_verification.txt"; then
        echo "  Status: SUCCESS ✓" >> "../../$SUMMARY_FILE"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        echo "  ✓ Verification SUCCESS for $ENTRY_FUNCTION"
    else
        echo "  Status: FAILED ✗" >> "../../$SUMMARY_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "  ✗ Verification FAILED for $ENTRY_FUNCTION"

        # Check for specific error types
        if grep -q "LLVM ERROR: Code generator does not support intrinsic function" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: Unsupported intrinsic" >> "../../$SUMMARY_FILE"
        fi

        if grep -q "Error: Attempt to read from uninitialized memory!" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: Uninitialised read" >> "../../$SUMMARY_FILE"
        fi

        if grep -q "ERROR: Could not find program's entry point function!" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: No entry point" >> "../../$SUMMARY_FILE"
        fi

        if grep -q "ERROR: Tried to execute an unknown external function:" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: External function" >> "../../$SUMMARY_FILE"
            # Extract the external function name
            grep "ERROR: Tried to execute an unknown external function:" "${ENTRY_FUNCTION}_verification.txt" | head -1 >> "../../$SUMMARY_FILE"
        fi

        if grep -q "TIMEOUT" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: Timeout" >> "../../$SUMMARY_FILE"
        fi

        if grep -q "Assertion violation" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: Assertion violation" >> "../../$SUMMARY_FILE"
        fi

        if grep -q "Safety violation" "${ENTRY_FUNCTION}_verification.txt"; then
            echo "  Error: Safety violation" >> "../../$SUMMARY_FILE"
        fi
    fi

    echo "" >> "../../$SUMMARY_FILE"

    cd ../..

    echo "================= Finished [$CURRENT/$TOTAL_FUNCTIONS] $ENTRY_FUNCTION ================="
    echo ""

done <<< "$ENTRY_FUNCTIONS"

# Final summary
echo "" >> "$SUMMARY_FILE"
echo "========================================" >> "$SUMMARY_FILE"
echo "FINAL RESULTS:" >> "$SUMMARY_FILE"
echo "  Total:   $TOTAL_FUNCTIONS" >> "$SUMMARY_FILE"
echo "  Success: $SUCCESS_COUNT" >> "$SUMMARY_FILE"
echo "  Failed:  $FAIL_COUNT" >> "$SUMMARY_FILE"
echo "========================================" >> "$SUMMARY_FILE"

echo ""
echo "============================================"
echo "ALL VERIFICATIONS COMPLETE"
echo "============================================"
echo "Total functions verified: $TOTAL_FUNCTIONS"
echo "Successful: $SUCCESS_COUNT"
echo "Failed: $FAIL_COUNT"
echo ""
echo "Summary saved to: $SUMMARY_FILE"
echo "Individual traces saved to: test_traces/${FILENAME}/"
echo "============================================"

# Clean up temporary project
rm -rf "$TEMP_PROJECT"
