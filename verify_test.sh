if [ "$#" -lt 1 ]; then
	echo "Target Cargo Project not supplied. Exiting"
	exit 1
fi

TARGET_RUST_PROJECT=$1
MIXED_LANGUAGE=false
DEPDIR=$(pwd)

shift

while [ $# -gt 0 ]; do
  case "$1" in
    -ffi)
      MIXED_LANGUAGE=true
      shift  # consume this argument
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      shift
      ;;
  esac
done

cd genmc
make
cd ..

cd $TARGET_RUST_PROJECT

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

find . -name "*.rs" | while read -r file; do
  filename=$(basename "$file" .rs)
  awk -v prefix="${filename}_" '
    /^[[:space:]]*#\[test\]/ { in_test = 1; print; next }
    in_test && /^[[:space:]]*fn[[:space:]]+([a-zA-Z0-9_]+)/ {
      if (match($0, /fn[[:space:]]+([a-zA-Z0-9_]+)/, m)) {
        old_name = m[1]
        sub(old_name, prefix old_name)
      }
      in_test = 0
    }
    { print }
  ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done

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

RUSTFLAGS="-Zpanic_abort_tests -C overflow-checks=off -C prefer-dynamic=no -C codegen-units=1 -C lto=no -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C codegen-units=1 -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -Z mir-opt-level=0 --emit=llvm-bc" rustup run RustMC cargo test --target-dir target-ir --no-run --target x86_64-unknown-linux-gnu 

#if [ "$MIXED_LANGUAGE" = "true" ]; then
#	clang -O3 -emit-llvm -c *.c
#	mv *.bc $(pwd)/target/x86_64-unknown-linux-gnu/debug/deps
#fi

find $(pwd)/target-ir/debug/deps -name "*.bc" > $DEPDIR/bitcode.txt

cd $DEPDIR

llvm-link --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined.ll @bitcode.txt

cd $DEPDIR/genmc


while read -r test_func; do
  echo "Verifying test function: $test_func"
  timeout 20s ./genmc --transform-output=myout.ll \
          --print-exec-graphs \
          --disable-function-inliner \
          --program-entry-function="$test_func" \
          --disable-estimation \
          --print-error-trace \
          --disable-stop-on-system-error \
          ../combined.ll > "../test_results/${test_func}_verification.txt" 2>&1

  if [ $? -eq 124 ]; then
    echo "TIMEOUT" >> "../test_results/${test_func}_verification.txt"
  fi
done < "$TEST_FUNCS_FILE"


#./genmc --transform-output=myout.ll --print-exec-graphs --disable-function-inliner --program-entry-function="hashmap_insert_verification.txt" --disable-estimation --print-error-trace --disable-stop-on-system-error ../combined.ll > "../test_results/hashmap_insert_verification.txt"