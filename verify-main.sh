
if [ "$#" -lt 1 ]; then
	echo "Usage: $0 <rust_file> [entry_function]"
	echo "  rust_file: Path to .rs file to verify"
	echo "  entry_function: (optional) Name of #[no_mangle] function to use as entry point"
	exit 1
fi

TARGET_RUST_FILE=$1
ENTRY_FUNCTION=$2
DEPDIR=$(pwd)

# If no entry function specified, try to find a #[no_mangle] function in the file
if [ -z "$ENTRY_FUNCTION" ]; then
	ENTRY_FUNCTION=$(awk '
		BEGIN { seen_no_mangle = 0 }
		/^[[:space:]]*#\[no_mangle\]/ { seen_no_mangle = 1; next }
		seen_no_mangle && /^[[:space:]]*pub[[:space:]]+fn[[:space:]]+[a-zA-Z0-9_]+/ {
			match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)
			print m[1]
			exit
		}
		seen_no_mangle && /^[[:space:]]*fn[[:space:]]+[a-zA-Z0-9_]+/ {
			match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)
			print m[1]
			exit
		}
		/^[[:space:]]*$/ { seen_no_mangle = 0 }
	' "$TARGET_RUST_FILE")

	if [ -z "$ENTRY_FUNCTION" ]; then
		echo "ERROR: No #[no_mangle] function found in $TARGET_RUST_FILE"
		echo "Either add a #[no_mangle] function or specify the entry function name as second argument"
		echo "Example: $0 $TARGET_RUST_FILE my_entry_function"
		exit 1
	fi

	echo "Found entry function: $ENTRY_FUNCTION"
fi

make

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

/usr/bin/llvm-link-18 --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined_old.ll @bitcode.txt
/usr/bin/opt-18 -S -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.ll -o combined.ll

mkdir -p "test_traces/${FILENAME}"
mkdir -p "test_results"

echo " "
echo " ================= Verifying main function ================= "
echo " "

# Check if combined.ll was created
if [ ! -f combined.ll ]; then
    echo "ERROR: combined.ll not found!"
    exit 1
fi

echo "Verifying: $ENTRY_FUNCTION"
echo "Running genmc on combined.ll..."

timeout 1000s ./genmc --mixer \
        --transform-output=myout.ll \
        --print-exec-graphs \
        --disable-function-inliner \
        --disable-assume-propagation \
        --disable-load-annotation \
        --disable-confirmation-annotation \
        --disable-spin-assume \
        --program-entry-function="$ENTRY_FUNCTION" \
        --disable-estimation \
        --print-error-trace \
        --disable-stop-on-system-error \
        --unroll=2 \
        combined.ll > "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt" 2>&1

GENMC_EXIT=$?

if [ $GENMC_EXIT -eq 124 ]; then
    echo "TIMEOUT" >> "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt"
fi

echo " "
echo "GenMC exit code: $GENMC_EXIT"
echo "Last 30 lines of verification output:"
tail -30 "test_traces/${FILENAME}/${ENTRY_FUNCTION}_verification.txt"
echo " "
echo " ================= Finished Verifying ${ENTRY_FUNCTION} function ================= "
echo " "

cd test_traces/${FILENAME}/

success_search_string="Verification complete. No errors were detected."
if grep -q "$success_search_string" "${ENTRY_FUNCTION}_verification.txt"; then
    echo "Verification success!" > ../../test_results/${FILENAME}_summary.txt
else
    echo "Verification failed or encountered errors" > ../../test_results/${FILENAME}_summary.txt

    # Check for specific error types
    grep -q "LLVM ERROR: Code generator does not support intrinsic function" "${ENTRY_FUNCTION}_verification.txt" && \
        echo "Error type: Unsupported intrinsic" >> ../../test_results/${FILENAME}_summary.txt

    grep -q "Error: Attempt to read from uninitialized memory!" "${ENTRY_FUNCTION}_verification.txt" && \
        echo "Error type: Uninitialised read" >> ../../test_results/${FILENAME}_summary.txt

    grep -q "ERROR: Could not find program's entry point function!" "${ENTRY_FUNCTION}_verification.txt" && \
        echo "Error type: No entry point" >> ../../test_results/${FILENAME}_summary.txt

    grep -q "ERROR: Tried to execute an unknown external function:" "${ENTRY_FUNCTION}_verification.txt" && \
        echo "Error type: External function" >> ../../test_results/${FILENAME}_summary.txt

    grep -q "TIMEOUT" "${ENTRY_FUNCTION}_verification.txt" && \
        echo "Error type: Timeout" >> ../../test_results/${FILENAME}_summary.txt
fi

cd ../..

# Clean up temporary project
rm -rf "$TEMP_PROJECT"
