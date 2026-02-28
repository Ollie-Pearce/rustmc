#!/bin/bash
set -euo pipefail

INPUT="${1:?Usage: $0 <figure_name|number> (e.g., figure1 or 1)}"

# Normalize: accept bare numbers (e.g., "1") or full names (e.g., "figure1")
if [[ "$INPUT" =~ ^[0-9]+$ ]]; then
    FIGURE_NAME="figure${INPUT}"
else
    FIGURE_NAME="$INPUT"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FIGURE_DIR="$SCRIPT_DIR/figures/$FIGURE_NAME"
RESULTS_DIR="$SCRIPT_DIR/results"

if [ ! -d "$FIGURE_DIR" ]; then
    echo "Error: Figure directory not found: $FIGURE_DIR"
    exit 1
fi

mkdir -p "$RESULTS_DIR"

RUST_COMPILE_FLAGS=(
  "--emit=llvm-bc,llvm-ir"
  "-Zdylib-lto"
  "-Zpanic_abort_tests"
  "-C panic=abort"
  "-C codegen-units=1"
  "-C embed-bitcode=yes"
  "-C overflow-checks=off"
  "-C target-feature=-avx,-avx2,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-sse3,-ssse3,-sse4.1,-sse4.2,-xsave"
  "-C no-vectorize-slp"
  "-C no-vectorize-loops"
  "-C prefer-dynamic=no"
  "-C lto=fat"
  "-C opt-level=3"
  "-C debuginfo=2"
  "-C llvm-args=--bpf-expand-memcpy-in-order"
  "-C no-prepopulate-passes"
  "-C passes=ipsccp"
  "-C passes=globalopt"
  "-C passes=reassociate"
  "-C passes=argpromotion"
  "-C passes=typepromotion"
  "-C passes=lower-constant-intrinsics"
  "-C passes=memcpyopt"
  "-Z mir-opt-level=0"
  "--target=x86_64-unknown-linux-gnu"
)

GENMC_FLAGS=(
  --mixer
  --disable-assume-propagation
  --disable-load-annotation
  --disable-confirmation-annotation
  --disable-spin-assume
  --disable-loop-jump-threading
  --disable-code-condenser
  --disable-estimation
  --print-error-trace
  --disable-stop-on-system-error
  --unroll=2
)

OUTPUT_FILE="$RESULTS_DIR/${FIGURE_NAME}_output.txt"

echo "Verifying $FIGURE_NAME ..."

{
# Build GenMC
make -C "$PROJECT_ROOT"

# Compile Rust
cd "$FIGURE_DIR"
cargo clean

RUSTFLAGS="${RUST_COMPILE_FLAGS[*]}" \
rustup run nightly-2024-06-11-x86_64-unknown-linux-gnu cargo build  > /dev/null 2>&1

DEPS_DIR="$FIGURE_DIR/target/debug/deps"

# Compile any C files in the figure directory to bitcode
C_BC_NAMES=()
for c_file in "$FIGURE_DIR"/*.c; do
    [ -f "$c_file" ] || continue
    bc_name="$(basename "${c_file%.c}.bc")"
    clang -O0 -emit-llvm -c "$c_file" -o "$bc_name"
    mv "$bc_name" "$DEPS_DIR/"
    C_BC_NAMES+=("$bc_name")
done

# Extract crate name from Cargo.toml, replace hyphens with underscores
CRATE_NAME=$(grep '^name' "$FIGURE_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/' | tr '-' '_')

# Collect bitcode files: crate .bc + any C .bc
find "$DEPS_DIR" -name "${CRATE_NAME}-*.bc" > bitcode.txt
for bc in "${C_BC_NAMES[@]}"; do
    find "$DEPS_DIR" -name "$bc" >> bitcode.txt
done

# Link bitcode
llvm-link-18 --override="$PROJECT_ROOT/override/globals_override.ll" -o combined.bc @bitcode.txt > /dev/null 2>&1

# Run GenMC
"$PROJECT_ROOT/genmc" "${GENMC_FLAGS[@]}" "$FIGURE_DIR/combined.bc"

} > "$OUTPUT_FILE" 2>&1 || true

echo ""
echo "Results in $OUTPUT_FILE:"
echo ""
tail -5 "$OUTPUT_FILE"
