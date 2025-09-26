MIXED_LANGUAGE=false
DEPDIR=$(pwd)
INCLUDE_DEPS=false

while [ $# -gt 1 ]; do
  case "$1" in
    --ffi)
      MIXED_LANGUAGE=true
      shift
      ;;
    --include-deps)
      INCLUDE_DEPS=true
      shift
      ;;
    --test-dir=*)
      TEST_DIR="${1#*=}"
      echo "test dir: $TEST_DIR"
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

rm -rf test_traces/
rm -rf test_results/
mkdir -p test_results
mkdir -p test_traces

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

# collect test names
echo "Collecting #[test] function names..."
TEST_FUNCS_FILE="$DEPDIR/test_functions.txt"
> "$TEST_FUNCS_FILE"

find . -name "*.rs" | while read -r file; do
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
done >> "$TEST_FUNCS_FILE"



cargo clean

# Create temp file for output
cargo_output_file=$(mktemp)

RUSTFLAGS="--emit=dep-info,link,llvm-bc,llvm-ir -Zpanic_abort_tests -C overflow-checks=off -C target-feature=-avx2 -C no-vectorize-slp -C no-vectorize-loops -C prefer-dynamic=no -C codegen-units=1 -C lto=no -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -Z mir-opt-level=0 --target=x86_64-unknown-linux-gnu" rustup run RustMC cargo test --workspace --target-dir target-ir --no-run > "$cargo_output_file" 2>&1

#if [ "$MIXED_LANGUAGE" = "true" ]; then
#	clang -O3 -emit-llvm -c *.c
#	mv *.bc $(pwd)/target/x86_64-unknown-linux-gnu/debug/deps
#fi



if [ "$INCLUDE_DEPS" = "true" ]; then
  find "$(pwd)/target-ir/debug/deps" -name "*.bc" > "$DEPDIR/bitcode.txt"
else
  find "$(pwd)/target-ir/debug/deps" -type f \( -name "${TEST_DIR}-*.bc" -o -name "lib-*.bc" \) > "$DEPDIR/bitcode.txt"
fi

cd $DEPDIR

/usr/bin/llvm-link-18 --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined_old.ll @bitcode.txt
/usr/bin/opt-18 -S -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.ll -o combined.ll

mkdir -p test_traces/${PROJECT_NAME}/

while read -r test_func; do
  echo "Verifying test function: $test_func"
  timeout 1000s ./genmc --mixer \
          --transform-output=myout.ll \
          --print-exec-graphs \
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
          combined.ll > "test_traces/${PROJECT_NAME}/${test_func}_verification.txt" 2>&1

  if [ $? -eq 124 ]; then
      echo "TIMEOUT" >> "test_traces/${PROJECT_NAME}/${test_func}_verification.txt"
  fi
done < "$TEST_FUNCS_FILE"

#./genmc --mixer --transform-output=myout.ll --print-exec-graphs --disable-function-inliner --program-entry-function="test_as_ptr_1_1 --disable-estimation --print-error-trace --disable-stop-on-system-error --unroll=2 combined.ll


#/usr/bin/time -v ./genmc --mixer --transform-output=myout.ll --print-exec-graphs --disable-function-inliner --disable-assume-propagation --disable-load-annotation --disable-confirmation-annotation --disable-spin-assume --program-entry-function="test_as_ptr_1_1" --disable-estimation --print-error-trace --disable-stop-on-system-error --unroll=2 combined.ll


# Above gives us:
# LLVM ERROR: Could not resolve external global address: _ZN3std4sync4mpmc7context7Context4with7CONTEXT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h1e32d3ce09f1da45E

cd test_traces/${PROJECT_NAME}/

file_count=$(ls | wc -l)

success_search_string="Verification complete. No errors were detected."
success_count=$(grep -rl "$success_search_string" . | wc -l)
echo "Verification success: $success_count / $file_count" > ../../test_results/${PROJECT_NAME}_summary.txt

unsupported_intrinsic_string="LLVM ERROR: Code generator does not support intrinsic function"
unsupported_intrinsic_count=$(grep -rl "$unsupported_intrinsic_string" . | wc -l)
echo "Unsupported intrinsic errors: $unsupported_intrinsic_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

uninitialised_read_string="Error: Attempt to read from uninitialized memory!"
uninitialised_read_count=$(grep -rl "$uninitialised_read_string" . | wc -l)
echo "Uninitialised read errors: $uninitialised_read_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

no_entry_string="ERROR: Could not find program's entry point function!"
no_entry_count=$(grep -rl "$no_entry_string" . | wc -l)
echo "No entry point errors: $no_entry_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

external_function_string="ERROR: Tried to execute an unknown external function:"
external_function_count=$(grep -rl "$external_function_string" . | wc -l)
echo "External function errors: $external_function_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

visit_atomic_rmw_string="visitAtomicRMWInst"
visit_atomic_rmw_count=$(grep -rl "$visit_atomic_rmw_string" . | wc -l)
echo "AtomicRMW errors: $visit_atomic_rmw_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

external_address_string="LLVM ERROR: Could not resolve external global address:"
external_address_count=$(grep -rl "$external_address_string" . | wc -l)
echo "External address errors: $external_address_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

memset_promotion_string="ERROR: Invalid call to memset()!"
memset_promotion_count=$(grep -rl "$memset_promotion_string" . | wc -l)
echo "Memset promotion errors: $memset_promotion_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt
ilist_iterator_string="llvm::ilist_iterator_w_bits"
ilist_iterator_count=$(grep -rl "$ilist_iterator_string" . | wc -l)
echo "ilist iterator errors: $ilist_iterator_count / $file_count" >> ../../test_results/${PROJECT_NAME}_summary.txt

cd ../..
#rm combined.ll combined_old.ll
