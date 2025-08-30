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
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

if [ "$#" -lt 1 ]; then
    echo "Target directory not supplied. Exiting"
    exit 1
fi

TARGET_DIR=$1

# Ensure test_results directory exists
mkdir -p test_results
rm -f test_results/*.txt

make

# Find all directories containing Cargo.toml
find "$TARGET_DIR" -name "Cargo.toml" -exec dirname {} \; | while read -r project_dir; do
    echo "Processing project in: $project_dir"
    cd "$project_dir" || continue
    
    PROJECT_NAME=$(grep -m1 '^name\s*=' Cargo.toml | sed -E 's/name\s*=\s*"([^"]+)".*/\1/')
    echo "Project name: $PROJECT_NAME"

    # 1) Add #[no_mangle] to #[test] functions that do not have it
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

    # 2) Rename #[test] functions so they are all unique
    python_script="$DEPDIR/rename_duplicate_rust_tests.py"
    while true; do
        output=$(python3 "$python_script")
        if [ "$output" = "No duplicates found or no changes required." ]; then
            break
        fi
        echo "Output changed: $output"
        sleep 1
    done

    # 3) collect test names
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

    if [ "$INCLUDE_DEPS" = "true" ]; then
        find "$(pwd)/target-ir/debug/deps" -name "*.bc" > "$DEPDIR/bitcode.txt"
    else
        find "$(pwd)/target-ir/debug/deps" -type f -name "${PROJECT_NAME}-*.bc" > "$DEPDIR/bitcode.txt"
    fi

    cd "$DEPDIR"

    llvm-link --internalize -S --override="$DEPDIR/override/my_pthread.ll" -o combined.ll @bitcode.txt

    while read -r test_func; do
        echo "Verifying test function: $test_func"
        timeout 20s ./genmc --mixer \
                --transform-output=myout.ll \
                --print-exec-graphs \
                --disable-function-inliner \
                --program-entry-function="$test_func" \
                --disable-estimation \
                --print-error-trace \
                --disable-stop-on-system-error \
                combined.ll > "test_results/${PROJECT_NAME}_${test_func}_verification.txt" 2>&1

        if [ $? -eq 124 ]; then
            echo "TIMEOUT" >> "test_results/${PROJECT_NAME}_${test_func}_verification.txt"
        fi
    done < "$TEST_FUNCS_FILE"
done