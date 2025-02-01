# RustMC: Extending the GenMC stateless model checker to Rust

## Set-up and compile

### Installing Rust source:

To build the adapted Rust standard library follow the instructions
below.

- use git `submodule update --init --recursive` to clone the custom toolchain submodule

- Naviagate to `rust/`

- Insert the following in config.toml 
	
```
[llvm]
download-ci-llvm = false
```
	
- Navigate to RustMC/rust and execute `./x build library`

- Run `rustup toolchain link RustMC rust/build/x86_64-unknown-linux-gnu/stage1`

### Building GenMC/RustMC:

- Navigate to `RustMC/genmc_for_rust/genmc`

- Run:
```
autoreconf --install
./configure
make
```

## Verifying a Rust project with RustMC:

- Run: `sh verify.sh /path/to/project/`

- Results will be output in `rustmc/verification.txt`


## Verifying a Rust program containing FFI dependencies with RustMC

- Include the associated .c files in the root directory of the Rust project

- Run `sh verify.sh /path/to/project/ -ffi`

- Results will be output in `rustmc/verification.txt`
