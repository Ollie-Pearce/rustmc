# RustMC: Extending the GenMC stateless model checker to Rust

## Set-up and compile

### Installing Rust source:

To build the adapted Rust standard library follow the instructions
below.


- Navigate to RustMC/rust and execute `./x setup`
  
- select `b` for `compiler`

- insert the following in config.toml 
	
```
[llvm]
download-ci-llvm = false
```
	
- next run `rustup toolchain link RustMC rust/build/x86_64-unknown-linux-gnu/stage1`

### Building GenMC/RustMC:

- Navigate to `RustMC/genmc_for_rust/genmc`

- Run:
```
autoreconf --install
./configure
make
```

## Verifying a Rust program with RustMC:
- Run: `sh verify.sh /path/to/rust/file.rs`

- Results will be output in `rustmc/verification.txt`
