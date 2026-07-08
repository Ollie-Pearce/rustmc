# RustMC

A model checker for concurrent Rust programs. It compiles a crate's tests to LLVM IR and
systematically explores thread interleavings to find concurrency bugs (data
races, atomicity violations, etc.). The tool is described in this [paper](https://olliepearce.xyz/papers/rustmc.pdf)

## Requirements

- LLVM-21 / Clang-21 and the `llvm-{link,dis,nm}-21` tools on `PATH`
- Rust `nightly-2025-08-20` (`rustup toolchain install nightly-2025-08-20`)
- autotools + a C++20 compiler and the usual build libs (libffi, zlib, libedit, libxml2)

On Debian/Ubuntu, install everything above in one step:

```bash
./tools/install-deps.sh
```

For other distros, install the equivalent packages listed in the script.

## Install

```bash
autoreconf --install
./configure --with-llvm=/usr/lib/llvm-21
make
sudo make install
```

## Use

From a crate's root directory:

```bash
cargo rustmc test                       # verify all unit + integration tests
cargo rustmc test --all-features        # extra args pass through to `cargo test`
cargo rustmc test --features weak,serde
```

Per-test traces and a summary are written to `target/rustmc/`; a pass/fail
report is printed and the command exits non-zero if any test fails.
