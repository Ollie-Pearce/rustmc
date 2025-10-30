#!/bin/bash

MIXED_LANGUAGE=false
DEPDIR=$(pwd)

while [ $# -gt 1 ]; do
  case "$1" in
    --ffi)
      MIXED_LANGUAGE=true
      shift
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

if [ "$#" -lt 1 ]; then
	echo "Target Cargo Project not supplied. Exiting"
	exit 1
fi

TARGET_RUST_PROJECT=$1

make -C ..

# Rename #[test] functions so they are all unique
python_script="rename_duplicate_rust_tests.py"

# Loop until the output is not "No duplicates found or no changes required."
while true; do
    output=$(python3 "$python_script")

    if [ "$output" = "No duplicates found or no changes required." ]; then
        echo "Output unchanged"
        
        break
    fi

    echo "Output changed: $output"
    sleep 1  # Sleep for a second to avoid running continuously without pause
done

cd $TARGET_RUST_PROJECT

rm -rf target-ir
mkdir -p target-ir

PROJECT_NAME=$(grep -m1 '^name\s*=' Cargo.toml | sed -E 's/name\s*=\s*"([^"]+)".*/\1/')
PROJECT_NAME=$(printf '%s' "$PROJECT_NAME" | tr '-' '_')

# Add #[no_mangle] to #[test] functions that do not have it
find . -name "*.rs" | while read -r file; do
  awk '
    {
      if ($0 ~ /^[[:space:]]*#\[test\]/ && prev !~ /^[[:space:]]*#\[no_mangle\]/) {
        print "#[no_mangle]"
      }
      print
      prev = $0
    }
  ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done

# Collect Rust test file paths
echo "Collecting integration tests..."
rm -rf "$DEPDIR/integration_test_files.txt"
INTEGRATION_TEST_FILES="$DEPDIR/integration_test_files.txt"
: > "$INTEGRATION_TEST_FILES"

find . -path "*/tests/*.rs" -type f | sort > "$INTEGRATION_TEST_FILES"

# For each test file, extract names of functions annotated with #[test]
TEST_FN_DIR="$DEPDIR/test_functions/${PROJECT_NAME}"
mkdir -p "$TEST_FN_DIR"

while read -r file; do
  base="$(basename "${file%.*}")"
  awk '
    BEGIN { seen_test = 0 }
    /^[[:space:]]*#\[test\]/          { seen_test = 1; next }
    /^[[:space:]]*#\[/ && $0 !~ /#\[test\]/ { next }
    seen_test && /^[[:space:]]*(pub[[:space:]]+)?(async[[:space:]]+)?fn[[:space:]]/ {
      line = $0
      sub(/^[[:space:]]*(pub[[:space:]]+)?(async[[:space:]]+)?fn[[:space:]]+/, "", line)
      sub(/\(.*/, "", line)      # strip args
      gsub(/[[:space:]]/, "", line)
      print line
      seen_test = 0
      next
    }
    /^[[:space:]]*$/ { seen_test = 0 }
  ' "$file" > "$TEST_FN_DIR/${base}.txt"
done < "$INTEGRATION_TEST_FILES"

echo "Collecting unit tests..."
rm -rf "$DEPDIR/unit_test_functions.txt"
UNIT_TEST_FILE="$DEPDIR/unit_test_functions.txt"
> "$UNIT_TEST_FILE"

find . -path "./tests" -prune -o -name "*.rs" -print | while read -r file; do
  awk '
    BEGIN { in_test = 0 }
    /^[[:space:]]*#\[test\]/ { in_test = 1; next }
    in_test && /^[[:space:]]*fn[[:space:]]+[a-zA-Z0-9_]+/ {
      if (match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)) {
        print m[1]
      }
      in_test = 0
    }
  ' "$file"
done >> "$UNIT_TEST_FILE"

echo "Unit test function names written to: $UNIT_TEST_FILE"
cargo clean

# Create temp file for output
cargo_output_file=$(mktemp)

RUSTFLAGS="--emit=llvm-bc,llvm-ir \
-Zpanic_abort_tests \
-C overflow-checks=off \
-C target-feature=-avx2 \
-C no-vectorize-slp \
-C no-vectorize-loops \
-C prefer-dynamic=no \
-C codegen-units=1 \
-C lto=no \
-C opt-level=2 \
-C debuginfo=2 \
-C llvm-args=--inline-threshold=9000 \
-C llvm-args=--bpf-expand-memcpy-in-order \
-C no-prepopulate-passes \
-C passes=ipsccp \
-C passes=globalopt \
-C passes=reassociate \
-C passes=argpromotion \
-C passes=typepromotion \
-C passes=lower-constant-intrinsics \
-C passes=memcpyopt \
-Z mir-opt-level=0 \
--target=x86_64-unknown-linux-gnu" \
rustup run RustMC cargo test --all-features --workspace --target-dir target-ir --no-run > "$cargo_output_file" 2>&1

cd $DEPDIR

rm -rf test_results/${PROJECT_NAME}_summary.txt
rm -rf test_traces/${PROJECT_NAME}
mkdir -p "test_traces/${PROJECT_NAME}"

echo " "
echo " ================= Verifying Integration Tests ================= "
echo " "
while read -r test_file; do
  stem="$(basename "${test_file%.*}")"

  echo "stem is: $stem"

  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f \
    \( -name "${stem}-*.bc" -o -name "lib-*.bc" \) \
    > "$DEPDIR/bitcode.txt"

  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f \
    -name "${PROJECT_NAME}*.bc" \
  | xargs -r grep -L '@main' >> "$DEPDIR/bitcode.txt"

  llvm-link-18 --internalize --override="../override/my_pthread.ll" -o combined_old.bc @bitcode.txt

  opt-18 -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.bc -o combined.bc

  while read -r test_func; do
    echo "Verifying: $stem :: $test_func"

    out="test_traces/${PROJECT_NAME}/${stem}_${test_func}_verification.txt"

    timeout 3600s ../genmc --mixer \
      --disable-function-inliner \
      --disable-assume-propagation \
      --disable-load-annotation \
      --disable-confirmation-annotation \
      --disable-spin-assume \
      --program-entry-function="$test_func" \
      --disable-estimation \
      --print-error-trace \
      --disable-stop-on-system-error \
      --unroll=2 \
      combined.bc > "$out" 2>&1

    [ $? -eq 124 ] && echo "TIMEOUT" >> "$out"
  done < "$TEST_FN_DIR/${stem}.txt"
done < "$INTEGRATION_TEST_FILES"

echo " "
echo " ================= Finished Verifying Integration Tests ================= "
echo " "

cd $TARGET_RUST_PROJECT
find "$(pwd)/target-ir/debug/deps" -type f -name "${PROJECT_NAME}-*.bc" > "$DEPDIR/bitcode.txt"
cd $DEPDIR

llvm-link-18 --internalize --override=../override/my_pthread.ll -o combined_old.bc @bitcode.txt
opt-18 -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.bc -o combined.bc

# --- Report which externals are defined elsewhere in IR ---
IR_DEPS_DIR="$TARGET_RUST_PROJECT/target-ir/debug/deps"
[ -d "$IR_DEPS_DIR" ] || { echo "IR dir not found: $IR_DEPS_DIR" >&2; exit 1; }

report_file="test_results/${PROJECT_NAME}_extern_def_sites.txt"
mkdir -p test_results
: > "$report_file"

# Collect externals from this crate's IR: lines starting with 'declare ... @sym('
# Handle quoted names (@"...") and unquoted (@sym).
mapfile -t EXTERNAL_SYMS < <(
  grep -hE '^[[:space:]]*declare[[:space:]].*\@' "$IR_DEPS_DIR"/${PROJECT_NAME}-*.ll 2>/dev/null \
  | perl -ne 'if (/^\s*declare\b.*\@("?([^"(]+)"?)\(/) { print "$2\n"; }' \
  | sort -u
)

if ((${#EXTERNAL_SYMS[@]}==0)); then
  echo "No externals (declare) found in ${PROJECT_NAME}-*.ll under $IR_DEPS_DIR" | tee -a "$report_file"
else
  for sym in "${EXTERNAL_SYMS[@]}"; do
    re="$(perl -e 'print quotemeta(shift)' "$sym")"

    # Same-line 'define' with exact symbol, quoted or unquoted
    mapfile -t matches < <(
      grep -RHEn --include='*.ll' "^[[:space:]]*define[[:space:]].*\@(\"?$re\"?)\(" "$IR_DEPS_DIR" \
      | cut -d: -f1 | sort -u
    )

    # Only print positives
    if ((${#matches[@]})); then
      printf '%s | In target-ir (%s)\n' \
        "$sym" "$(printf '%s' "${matches[*]}")" >> "$report_file"
    fi
  done
fi

# --- Link all IR files that DEFINE externals from this crate ---
IR_DEPS_DIR="$TARGET_RUST_PROJECT/target-ir/debug/deps"

# 1) Recompute externals (declare lines) -> symbols
mapfile -t EXTERNAL_SYMS < <(
  grep -hE '^[[:space:]]*declare[[:space:]].*\@' "$IR_DEPS_DIR"/${PROJECT_NAME}-*.ll 2>/dev/null \
  | perl -ne 'if (/^\s*declare\b.*\@("?([^"(]+)"?)\(/) { print "$2\n"; }' \
  | sort -u
)

# 2) For each symbol, find *.ll files with SAME-LINE definition:  ^\s*define ... @sym(  or  @"sym"(
declare -a DEF_LL_FILES=()

if ((${#EXTERNAL_SYMS[@]})); then
  for sym in "${EXTERNAL_SYMS[@]}"; do
    re="$(perl -e 'print quotemeta(shift)' "$sym")"
    # Collect matching files; append to DEF_LL_FILES
    while IFS= read -r f; do
      DEF_LL_FILES+=("$f")
    done < <(
      grep -RHEn --include='*.ll' "^[[:space:]]*define[[:space:]].*\@(\"?$re\"?)\(" "$IR_DEPS_DIR" \
      | cut -d: -f1
    )
  done
fi

# 3) De-duplicate file list
if ((${#DEF_LL_FILES[@]})); then
  mapfile -t DEF_LL_FILES < <(printf '%s\n' "${DEF_LL_FILES[@]}" | sort -u)
else
  echo "No definition sites found for externals in ${PROJECT_NAME}-*.ll"
fi

# 4) Persist lists and link
mkdir -p test_results
printf '%s\n' "${DEF_LL_FILES[@]}" > "test_results/${PROJECT_NAME}_extern_def_ll_files.txt"

# Add this crate’s own IR files to the link set
mapfile -t OWN_LL_FILES < <(find "$IR_DEPS_DIR" -maxdepth 1 -type f -name "${PROJECT_NAME}-*.ll" | sort -u)

# Prefer linking bitcode where available; fall back to .ll
declare -a LINK_INPUTS=()
for f in "${DEF_LL_FILES[@]}" "${OWN_LL_FILES[@]}"; do
  b="${f%.ll}.bc"
  if [ -f "$b" ]; then
    LINK_INPUTS+=("$b")
  else
    LINK_INPUTS+=("$f")
  fi
done

printf '%s\n' "${LINK_INPUTS[@]}" > "test_results/${PROJECT_NAME}_extern_def_link_inputs.txt"

if ((${#LINK_INPUTS[@]})); then
  llvm-link-18 -o extern_defs_only.bc "${LINK_INPUTS[@]}"
  llvm-link-18  --override=../override/my_pthread.ll -o extern_defs_only.ll "${LINK_INPUTS[@]}"
fi

opt-18 -mtriple=x86_64-unknown-linux-gnu -expand-reductions extern_defs_only.ll -o extern_defs_only.ll

echo " "
echo " ================= Verifying Unit Tests ================= "
echo " "

while read -r test_func; do
  echo "Verifying test function: $test_func"
  timeout 3600s ../genmc --mixer \
          --disable-function-inliner \
          --disable-assume-propagation \
          --disable-load-annotation \
          --disable-confirmation-annotation \
          --disable-spin-assume \
          --program-entry-function="$test_func" \
          --disable-estimation \
          --print-error-trace \
          --disable-stop-on-system-error \
          --unroll=2 \
          extern_defs_only.ll > "test_traces/${PROJECT_NAME}/${test_func}_verification.txt" 2>&1

  if [ $? -eq 124 ]; then
      echo "TIMEOUT" >> "test_traces/${PROJECT_NAME}/${test_func}_verification.txt"
  fi
done < "$UNIT_TEST_FILE"

echo " "
echo " ================= Finished Verifying Unit Tests ================= "
echo " "

cd test_traces/${PROJECT_NAME}/

file_count=$(ls | wc -l)

success_search_string="Verification complete. No errors were detected."
success_count=$(grep -rl "$success_search_string" . | wc -l)
echo "Verification success: $success_count / $file_count" > "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

thread_panicked_string="Thread panicked"
thread_panic_count=$(grep -rl "$thread_panicked_string" . | wc -l)
echo "Panic called: $thread_panic_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

uninitialised_read_string="Error: Attempt to read from uninitialized memory!"
uninitialised_read_count=$(grep -rl "$uninitialised_read_string" . | wc -l)
echo "Uninitialised read errors: $uninitialised_read_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

no_entry_string="ERROR: Could not find program's entry point function!"
no_entry_count=$(grep -rl "$no_entry_string" . | wc -l)
echo "No entry point errors: $no_entry_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

external_function_string="ERROR: Tried to execute an unknown external function:"
external_function_count=$(grep -rl "$external_function_string" . | wc -l)
echo "External function errors: $external_function_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

visit_atomic_rmw_string="visitAtomicRMWInst"
visit_atomic_rmw_count=$(grep -rl "$visit_atomic_rmw_string" . | wc -l)
echo "AtomicRMW errors: $visit_atomic_rmw_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

external_address_string="LLVM ERROR: Could not resolve external global address:"
external_address_count=$(grep -rl "$external_address_string" . | wc -l)
echo "External address errors: $external_address_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

constant_unimplemented_string="Constant unimplemented for type"
constant_unimplemented_count=$(grep -rl "$constant_unimplemented_string" . | wc -l)
echo "constant unimplemented errors: $constant_unimplemented_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

external_var_arg_string="Calling external var arg function"
external_var_arg_count=$(grep -rl "$external_var_arg_string" . | wc -l)
echo "external var args errors: $external_var_arg_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

memset_promotion_string="ERROR: Invalid call to memset()!"
memset_promotion_count=$(grep -rl "$memset_promotion_string" . | wc -l)
echo "Memset promotion errors: $memset_promotion_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

memcpy_count=$(
  grep -rlE "Invalid call to memcpy\(\)!|Assertion \`!NodePtr->isKnownSentinel\(\)' failed\." . | wc -l
)
echo "memcpy errors: $memcpy_count / $file_count" >> ""$DEPDIR/test_results/${PROJECT_NAME}_summary.txt""

segfault_string="Segmentation fault"
segfault_count=$(grep -rl "$segfault_string" . | wc -l)
echo "segmentation fault errors: $segfault_count / $file_count" >> "$DEPDIR/test_results/${PROJECT_NAME}_summary.txt"

cd ../../test_results
rm -rf archery_extern_def_link_inputs.txt archery_extern_def_link_inputs.txt archery_extern_def_sites.txt