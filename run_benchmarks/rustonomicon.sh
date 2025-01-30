cd ..

DEPDIR=$(pwd)

cd $DEPDIR/genmc

make

cd $DEPDIR/rust_benchmark/rustonomicon

cargo clean
 
export RUSTFLAGS=" -C overflow-checks=off -C panic=abort --emit=llvm-bc -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -C passes=dse"
rustup run RustMC cargo run --target x86_64-unknown-linux-gnu 

#rustup run nightly-x86_64-unknown-linux-gnu cargo run -Z build-std=panic_abort,std --target x86_64-unknown-linux-gnu 


cd ..

find rustonomicon/target/x86_64-unknown-linux-gnu/debug/deps -name "*.bc" > bitcode.txt

cat bitcode.txt 

llvm-link --internalize -S --override=$DEPDIR/override/my_pthread.ll -o combined.ll @bitcode.txt 

cd $DEPDIR/genmc

./genmc --program-entry-function=main --print-exec-graphs -disable-estimation --print-error-trace --disable-stop-on-system-error --transform-output=myout2.ll $DEPDIR/rust_benchmark/combined.ll > rustonomicon_output.txt
