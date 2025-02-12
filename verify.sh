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

cargo clean

export RUSTFLAGS=" -C overflow-checks=off -C panic=abort --emit=llvm-bc -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -C passes=dse"
rustup run RustMC cargo run --target x86_64-unknown-linux-gnu 

if [ "$MIXED_LANGUAGE" = "true" ]; then
	clang -O3 -emit-llvm -c *.c
	mv *.bc $(pwd)/target/x86_64-unknown-linux-gnu/debug/deps
fi

find $(pwd)/target/x86_64-unknown-linux-gnu/debug/deps -name "*.bc" > $DEPDIR/bitcode.txt

cd $DEPDIR

llvm-link --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined.ll @bitcode.txt 

cd $DEPDIR/genmc

./genmc --print-exec-graphs --disable-function-inliner --program-entry-function=main --disable-estimation --print-error-trace --disable-stop-on-system-error $DEPDIR/combined.ll > $DEPDIR/verification.txt
