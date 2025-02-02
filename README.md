# RustMC: Extending the GenMC stateless model checker to Rust

## Set-up and compile

### Linking Rust Toolchain and Building GenMC/RustMC:

The included build script will:

- Download, extract and link the custom toolchain 

- Build GenMC/RustMC

- To execute the build script run: `sh build.sh`

- The custom toolchain can be found at: <https://github.com/Ollie-Pearce/rust>

## Verifying a Rust project with RustMC:

- Run: `sh verify.sh /path/to/project/`

- Results will be output in `rustmc/verification.txt`

## Verifying a Rust program containing FFI dependencies with RustMC

- Include the associated .c files in the root directory of the Rust project

- Run `sh verify.sh /path/to/project/ -ffi`

- Results will be output in `rustmc/verification.txt`
