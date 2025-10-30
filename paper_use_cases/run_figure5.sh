DEPDIR=$(pwd)

cd ..

make

cd $DEPDIR/rand_atomicity_violation_reproduction

cargo clean
 
export RUSTFLAGS=" -C overflow-checks=off -C panic=abort --emit=llvm-bc -C opt-level=0 -C debuginfo=2 -C llvm-args=--inline-threshold=9000 -C llvm-args=--bpf-expand-memcpy-in-order -C no-prepopulate-passes -C passes=ipsccp -C passes=globalopt -C passes=reassociate -C passes=argpromotion -C passes=typepromotion -C passes=lower-constant-intrinsics  -C passes=memcpyopt -C passes=dse"
rustup run RustMC cargo run --target x86_64-unknown-linux-gnu 


find $DEPDIR/rand_atomicity_violation_reproduction/target/x86_64-unknown-linux-gnu/debug/deps -name "*.bc" > bitcode.txt

llvm-link-18 --internalize --override=../../override/my_pthread.ll -o combined.bc @bitcode.txt 

../../genmc --mixer --program-entry-function=main --disable-estimation --print-error-trace --disable-stop-on-system-error --transform-output=myout2.ll $DEPDIR/rand_atomicity_violation_reproduction/combined.bc > $DEPDIR/benchmark_results/rand_atomicity_violation_reproduction.txt

