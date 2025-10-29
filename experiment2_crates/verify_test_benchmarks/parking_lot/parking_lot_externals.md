## RUSTFLAGS:

```
cargo clean
RUSTFLAGS="-C codegen-units=1 -C embed-bitcode=yes --emit=llvm-ir" \
rustup run RustMC cargo test --lib --no-run
```

## Externals from ll file

```
@_ZN3std3sys3pal4unix4rand3imp20getrandom_fill_bytes21GETRANDOM_UNAVAILABLE17h04e842afe27544adE = external global %"core::sync::atomic::AtomicBool"

@_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17hbc3f149190aeaa63E = external global %"core::sync::atomic::AtomicBool"

@"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h3cea3bcc94e317ffE" = external thread_local global %"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<(u64, u64)>, !>"

@"_ZN3std4sync4mpmc7context7Context4with7CONTEXT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h1e32d3ce09f1da45E" = external thread_local global %"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<core::option::Option<std::sync::mpmc::context::Context>>, ()>"

@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h541136d3707a013fE = external global %"core::sync::atomic::AtomicUsize"

@"_ZN3std6thread10CURRENT_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h72832091327b60bfE" = external thread_local global i64

@"_ZN3std6thread7CURRENT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17ha4638e9f4485ca67E" = external thread_local global %"std::sys::thread_local::native::eager::Storage<core::cell::once::OnceCell<std::thread::Thread>>"

@_ZN10std_detect6detect5cache5CACHE17ha70d35da6fb9c084E = external global [2 x %"parking_lot_core::word_lock::WordLock"]
```


---

## nm output:

B: uninitialised
D: Initialised

- `_ZN3std3sys3pal4unix4rand3imp20getrandom_fill_bytes21GETRANDOM_UNAVAILABLE17h04e842afe27544adE` | parking_lot-4c28e88765db9228 | B 

- `_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17hbc3f149190aeaa63E` | parking_lot-4c28e88765db9228 | D | 

- `_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h3cea3bcc94e317ffE` | parking | B

- `_ZN3std4sync4mpmc7context7Context4with7CONTEXT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h1e32d3ce09f1da45E` | parking | B

- `_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h541136d3707a013fE` | parking_lot-4c28e88765db9228 | B

- `_ZN3std6thread10CURRENT_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h72832091327b60bfE` | parking_lot-4c28e88765db9228 | B

- `_ZN3std6thread7CURRENT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17ha4638e9f4485ca67E` | parking_lot-4c28e88765db9228 | B

- `_ZN10std_detect6detect5cache5CACHE17ha70d35da6fb9c084E` | parking_lot-4c28e88765db9228 | B


### Notes:

(The following is with -C save-temps)

grep -il -r '_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17hbc3f149190aeaa63E' *

parking_lot-fbd7487e7df2cb01 | DEFINED
parking_lot-fbd7487e7df2cb01.b3jnj3qo4aocbxzn9d8zmnnos.rcgu.bc | UNDEFINED
parking_lot-fbd7487e7df2cb01.b3jnj3qo4aocbxzn9d8zmnnos.rcgu.ll | EXTERNAL
parking_lot-fbd7487e7df2cb01.b3jnj3qo4aocbxzn9d8zmnnos.rcgu.o | UNDEFINED
parking_lot-fbd7487e7df2cb01.ll | EXTERNAL

### Searching rust toolchain:
I compiled with `RUSTFLAGS="-C embed-bitcode=yes" ./x build library`

Looking for `GRND_INSECURE_AVAILABLE` specifically. Note that after rebuild it is called: `_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17h41b38b22f8cdd6e7E`


When I search I get the following:

#### `/Desktop/RustMC/rust/build/x86_64-unknown-linux-gnu$ grep -il -r '_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17h41b38b22f8cdd6e7E' *`
```
stage0-rustc/x86_64-unknown-linux-gnu/release/librustc_driver.so
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_driver_impl-0veogjjj2z8ah/s-hcczy6htou-1koc4wl-5uflahcjsd173an3qco7erbno/bs3h2kxisbrlt2fducs8pita5.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_driver_impl-0veogjjj2z8ah/s-hcczy6htou-1koc4wl-5uflahcjsd173an3qco7erbno/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_errors-16ei9z2qhu5uw/s-hcczxco0ky-0ykh2mn-7oi6gzoxdx3wsxxcm56dy9564/9lzeaxgnzb4ytt1f407g322pc.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_errors-16ei9z2qhu5uw/s-hcczxco0ky-0ykh2mn-7oi6gzoxdx3wsxxcm56dy9564/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_session-2y7l1z1uhqdm9/s-hcczxdatm2-1c19kb0-9g1v7v1gha5n6ayrg2codc810/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_session-2y7l1z1uhqdm9/s-hcczxdatm2-1c19kb0-9g1v7v1gha5n6ayrg2codc810/1tcdu1j70mr7ur6cb15mygun9.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_error_messages-33jmjccwo2oqo/s-hcczx8m0ej-0pfdpsl-dns701xs81btfwv7e216k2zue/157bny748ujde8c0nyjpbyr8r.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_error_messages-33jmjccwo2oqo/s-hcczx8m0ej-0pfdpsl-dns701xs81btfwv7e216k2zue/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_codegen_ssa-2ayzmjzmomox8/s-hcczxnwegg-1d9ivwo-edud887vpjni3euy4sw63e3sk/aho365sfoghipex9lkzxsirf5.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_codegen_ssa-2ayzmjzmomox8/s-hcczxnwegg-1d9ivwo-edud887vpjni3euy4sw63e3sk/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_log-1p87p6rfv6wa4/s-hcczx7rlpx-0qgbg5b-eycbfotvfy8z1zwl10wwuu89w/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_log-1p87p6rfv6wa4/s-hcczx7rlpx-0qgbg5b-eycbfotvfy8z1zwl10wwuu89w/c1m9wbkrmou8289j9mfk4ivl4.o
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_codegen_llvm-1xmhuvuxc7x24/s-hcczxva7dc-1109q2h-6vfxv04owaesw3a3jhswi7g38/query-cache.bin
stage0-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc_codegen_llvm-1xmhuvuxc7x24/s-hcczxva7dc-1109q2h-6vfxv04owaesw3a3jhswi7g38/445dsn3wo5eh39v9gqjeec4hj.o
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-60843e7f227f9a81.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgimli-4b39afb74a03afe4.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-48ca4d4767d77622.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_log-7b62c8dd20d822f8.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libregex_automata-e6f21ad035b4e246.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-85de6145de4b0bf7.so
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-88235b4b7a4287f8.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver_impl-f21d6cb61bff1423.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libobject-cc90faab38347ddd.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-b5f41eb8f68f6061.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-cd33824246986ce3.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-58c3a197be821384.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgsgdt-476dfe7b9d325c63.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libregex_automata-08ca9e5837e7046a.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libwasmparser-6ad8cb87e46daf45.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_utils-0f9d4483645df0a5.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_messages-dbb1b2b8173be9b2.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_llvm-796d87fb14ba2232.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libthorin-94d0bd81d1fefe61.rlib
stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-10a658cc9878da19.rlib
stage0-std/x86_64-unknown-linux-gnu/release/incremental/test-0skcmjb8z3f8l/s-hcczx273j2-0i2nfqe-2c9b4edr99wx1hhuqgfpvcy2e/query-cache.bin
stage0-std/x86_64-unknown-linux-gnu/release/incremental/test-0skcmjb8z3f8l/s-hcczx273j2-0i2nfqe-2c9b4edr99wx1hhuqgfpvcy2e/9kuvgkj6j97lgrikp9qxovneu.o
stage0-std/x86_64-unknown-linux-gnu/release/incremental/std-1lt015jqdgsbw/s-hcczwzsuun-1eodk2x-8c8sk9qickvobnluw3egdyjjx/query-cache.bin
stage0-std/x86_64-unknown-linux-gnu/release/incremental/std-1lt015jqdgsbw/s-hcczwzsuun-1eodk2x-8c8sk9qickvobnluw3egdyjjx/17s6zxfos0iwohp7sdgq24ahp.o
stage0-std/x86_64-unknown-linux-gnu/release/libstd.so
stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-f5fb4ec96d5f85f9.so
stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-f5fb4ec96d5f85f9.rlib
stage0-std/x86_64-unknown-linux-gnu/release/deps/libtest-ff9d088dbe788510.rlib
stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f5fb4ec96d5f85f9.so
stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f5fb4ec96d5f85f9.rlib
stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-85de6145de4b0bf7.so
stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-ff9d088dbe788510.rlib
stage1/lib/libstd-f5fb4ec96d5f85f9.so
stage1/lib/librustc_driver-85de6145de4b0bf7.so
```


Chat gpt gave me this script:

```
#!/usr/bin/env bash
set -euo pipefail

SYM='_ZN3std3sys3pal4unix4rand3imp20getrandom_fill_bytes21GETRANDOM_UNAVAILABLE17h04e842afe27544adE'

find . -type f \( -name '*.o' -o -name '*.rlib' \) -print0 \
| xargs -0 -P"$(nproc)" -I{} sh -c '
  f="$1"
  llvm-nm --quiet --defined-only --print-file-name "$f" 2>/dev/null
' sh {} \
| rg -F "$SYM"
```

And it returns the following: 

```
./stage0-std/x86_64-unknown-linux-gnu/release/incremental/std-1lt015jqdgsbw/s-hcczwzsuun-1eodk2x-8c8sk9qickvobnluw3egdyjjx/17s6zxfos0iwohp7sdgq24ahp.o: 0000000000000000 D _ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17h41b38b22f8cdd6e7E
./stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-f5fb4ec96d5f85f9.rlib:std-f5fb4ec96d5f85f9.17s6zxfos0iwohp7sdgq24ahp.rcgu.o: 0000000000000000 D _ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17h41b38b22f8cdd6e7E
./stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f5fb4ec96d5f85f9.rlib:std-f5fb4ec96d5f85f9.17s6zxfos0iwohp7sdgq24ahp.rcgu.o: 0000000000000000 D _ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17h41b38b22f8cdd6e7E
```

rlibs have an additional colon which describes the object file they take the symbol from 

- `./stage0-std/x86_64-unknown-linux-gnu/release/incremental/std-1lt015jqdgsbw/s-hcczwzsuun-1eodk2x-8c8sk9qickvobnluw3egdyjjx/17s6zxfos0iwohp7sdgq24ahp.o`, Ir sent via email
- `./stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-f5fb4ec96d5f85f9.rlib`
- `./stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f5fb4ec96d5f85f9.rlib`