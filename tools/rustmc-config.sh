#!/bin/bash
# Canonical compilation and verification flags for RustMC.
#
# Single source of truth, sourced by tools/cargo-rustmc (the `cargo rustmc test`
# front-end). Installed to $(pkgdatadir)/rustmc-config.sh by `make install`.

RUST_COMPILE_FLAGS=(
  "--emit=llvm-bc,llvm-ir"
  "-Zdylib-lto"
  "-Zpanic_abort_tests"
  "-C panic=abort"
  "-C codegen-units=1"
  "-C embed-bitcode=yes"
  "-C overflow-checks=off"
  "-C target-feature=-avx,-avx2,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-sse3,-ssse3,-sse4.1,-sse4.2,-xsave"
  "-C no-vectorize-slp"
  "-C no-vectorize-loops"
  "-C prefer-dynamic=no"
  "-C lto=fat"
  "-C opt-level=3"
  "-C debuginfo=2"
  "-C llvm-args=--bpf-expand-memcpy-in-order"
  "-C no-prepopulate-passes"
  "-C passes=ipsccp"
  "-C passes=globalopt"
  "-C passes=reassociate"
  "-C passes=argpromotion"
  "-C passes=typepromotion"
  "-C passes=lower-constant-intrinsics"
  "-C passes=memcpyopt"
  "-Z mir-opt-level=0"
  "--target=x86_64-unknown-linux-gnu"
)

GENMC_FLAGS=(
  --mixer
  --disable-spin-assume
  --disable-loop-jump-threading
  --disable-code-condenser
  --print-error-trace
  --disable-stop-on-system-error
  --unroll=2
)

RUST_TOOLCHAIN="nightly-2025-08-20-x86_64-unknown-linux-gnu"
