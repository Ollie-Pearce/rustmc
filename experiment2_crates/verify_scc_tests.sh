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

make

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
echo "$PROJECT_NAME"

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
-C opt-level=3 \
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
rustup run RustMC cargo test --workspace --target-dir target-ir --no-run > "$cargo_output_file" 2>&1

cd $DEPDIR

rm -rf "test_traces/${PROJECT_NAME}"
mkdir -p "test_traces/${PROJECT_NAME}"

echo " "
echo " ================= Verifying Integration Tests ================= "
echo " "
while read -r test_file; do
  stem="$(basename "${test_file%.*}")"

  echo "stem is: $stem"

  #find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f \
  #  \( -name "${stem}-*.bc" -o -name "lib-*.bc" \) \
  #  > "$DEPDIR/bitcode.txt"

  #if [ "$stem" != "$PROJECT_NAME" ]; then
  #  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f \
  #    -name "${PROJECT_NAME}*.ll" \
  #  | xargs -r grep -L '@main' >> "$DEPDIR/bitcode.txt"
  #fi

  # Above find sometimes links the same file multiple times, so make unique I think it's if the stem has the same name as the library

  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f -name "scc-*.bc" > "$DEPDIR/bitcode.txt"
  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f -name "fastrand-*.bc" >> "$DEPDIR/bitcode.txt" #needed for concurrent-queue
  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f -name "sdd-*.bc" >> "$DEPDIR/bitcode.txt"
  find "$TARGET_RUST_PROJECT/target-ir/debug/deps" -type f -name "proptest-*.bc" >> "$DEPDIR/bitcode.txt"


  echo "Bitcode files:"
  cat bitcode.txt

  llvm-link-18 --internalize \
    --override=../override/my_pthread.ll \
    -o combined_old.bc @bitcode.txt

  opt-18 -mtriple=x86_64-unknown-linux-gnu \
    -expand-reductions combined_old.bc -o combined.bc
    
  while read -r test_func; do
    echo "Verifying: $stem :: $test_func"

    out="test_traces/${PROJECT_NAME}/${stem}_${test_func}_verification.txt"

    timeout 3600s ../genmc --mixer \
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
find "$(pwd)/target-ir/debug/deps" -type f -name "pretty_assertions-*.bc" >> "$DEPDIR/bitcode.txt"
find "$(pwd)/target-ir/debug/deps" -type f -name "diff-*.bc" >> "$DEPDIR/bitcode.txt"
find "$(pwd)/target-ir/debug/deps" -type f -name "yansi-*.bc" >> "$DEPDIR/bitcode.txt"
cd $DEPDIR

echo "Bitcode files:"
cat bitcode.txt

llvm-link-18 --internalize --override../override/my_pthread.ll -o combined_old.bc @bitcode.txt
opt-18 -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.bc -o combined.bc

echo " "
echo " ================= Verifying Unit Tests ================= "
echo " "

while read -r test_func; do
  echo "Verifying test function: $test_func"
  timeout 3600s ../genmc --mixer \
          --disable-assume-propagation \
          --disable-load-annotation \
          --disable-confirmation-annotation \
          --disable-spin-assume \
          --program-entry-function="$test_func" \
          --disable-estimation \
          --print-error-trace \
          --disable-stop-on-system-error \
          --unroll=2 \
          combined.bc > "test_traces/${PROJECT_NAME}/${test_func}_verification.txt" 2>&1

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