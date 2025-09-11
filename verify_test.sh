
MIXED_LANGUAGE=false
INCLUDE_DEPS=true
DEPDIR=$(pwd)

while [ $# -gt 1 ]; do
  case "$1" in
    --ffi)
      MIXED_LANGUAGE=true
      shift  # consume this argument
      ;;
    --include-deps)
      INCLUDE_DEPS=true
      shift
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      shift
      ;;
  esac
done

if [ "$#" -lt 1 ]; then
	echo "Target Cargo Project not supplied. Exiting"
	exit 1
fi

TARGET_RUST_PROJECT=$1

rm test_results/*.txt

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

rm -rf target-ir/

PROJECT_NAME=$(grep -m1 '^name\s*=' Cargo.toml | sed -E 's/name\s*=\s*"([^"]+)".*/\1/')
PROJECT_NAME=$(printf '%s' "$PROJECT_NAME" | tr '-' '_')
echo "$PROJECT_NAME"

#git reset --hard HEAD

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

RUSTFLAGS="-Zpanic_abort_tests -C overflow-checks=off -C prefer-dynamic=no -C codegen-units=1 -C lto=no -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C codegen-units=1 -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -Z mir-opt-level=0 --emit=llvm-bc" rustup run RustMC cargo test --target-dir target-ir --no-run

#if [ "$MIXED_LANGUAGE" = "true" ]; then
#	clang -O3 -emit-llvm -c *.c
#	mv *.bc $(pwd)/target/x86_64-unknown-linux-gnu/debug/deps
#fi

echo "pwd is $(pwd)"
echo "DEPDIR is $DEPDIR"



if [ "$INCLUDE_DEPS" = "true" ]; then
  find "$(pwd)/target-ir/debug/deps" -name "*.bc" > "$DEPDIR/bitcode.txt"
else
  find "$(pwd)/target-ir/debug/deps" -type f -name "${PROJECT_NAME}-*.bc" > "$DEPDIR/bitcode.txt"
fi

cd $DEPDIR

#Maybe remove the DEPDIR var from below
#llvm-link --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined.ll @bitcode.txt

llvm-link --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined_old.ll @bitcode.txt
opt -S -mtriple=x86_64-unknown-linux-gnu -expand-reductions combined_old.ll -o combined.ll

while read -r test_func; do
  echo "Verifying test function: $test_func"
  timeout 80s ./genmc --mixer \
          --transform-output=myout.ll \
          --print-exec-graphs \
          --disable-function-inliner \
          --program-entry-function="$test_func" \
          --disable-estimation \
          --print-error-trace \
          --disable-stop-on-system-error \
          --unroll=2 \
          combined.ll > "test_results/${test_func}_verification.txt" 2>&1

  if [ $? -eq 124 ]; then
    echo "TIMEOUT" >> "test_results/${test_func}_verification.txt"
  fi
done < "$TEST_FUNCS_FILE"

#./genmc --mixer --transform-output=myout.ll --print-exec-graphs --disable-function-inliner --program-entry-function="double_substr_1" --disable-estimation --print-error-trace --disable-stop-on-system-error combined.ll 


cd test_results/

file_count=$(ls | wc -l)

success_search_string="Verification complete. No errors were detected."
success_count=$(grep -rl "$success_search_string" . | wc -l)
echo "Verification success: $success_count / $file_count"

unsupported_intrinsic_string="LLVM ERROR: Code generator does not support intrinsic function"
unsupported_intrinsic_count=$(grep -rl "$unsupported_intrinsic_string" . | wc -l)
echo "Unsupported intrinsic errors: $unsupported_intrinsic_count / $file_count"

uninitialised_read_string="Error: Attempt to read from uninitialized memory!"
uninitialised_read_count=$(grep -rl "$uninitialised_read_string" . | wc -l)
echo "Uninitialised read errors: $uninitialised_read_count / $file_count"

no_entry_string="ERROR: Could not find program's entry point function!"
no_entry_count=$(grep -rl "$no_entry_string" . | wc -l)
echo "No entry point errors: $no_entry_count / $file_count"

external_function_string="ERROR: Tried to execute an unknown external function:"
external_function_count=$(grep -rl "$external_function_string" . | wc -l)
echo "External function errors: $external_function_count / $file_count"

visit_atomic_rmw_string="visitAtomicRMWInst"
visit_atomic_rmw_count=$(grep -rl "$visit_atomic_rmw_string" . | wc -l)
echo "AtomicRMW errors: $visit_atomic_rmw_count / $file_count"


external_address_string="LLVM ERROR: Could not resolve external global address:"
external_address_count=$(grep -rl "$external_address_string" . | wc -l)
echo "External address errors: $external_address_count / $file_count"

memset_promotion_string="ERROR: Invalid call to memset()!"
memset_promotion_count=$(grep -rl "$memset_promotion_string" . | wc -l)
echo "Memset promotion errors: $memset_promotion_count / $file_count"

ilist_iterator_string="llvm::ilist_iterator_w_bits"
ilist_iterator_count=$(grep -rl "$ilist_iterator_string" . | wc -l)
echo "ilist iterator errors: $ilist_iterator_count / $file_count"
