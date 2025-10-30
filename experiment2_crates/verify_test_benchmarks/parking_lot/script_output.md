++ pwd
+ OUTPUT_DIR=/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot
+ LLVM_HOME=/usr/lib/llvm-18/
+ RUSTUP_TOOLCHAIN_LIB=/home/ollie/.rustup/toolchains/RustMC/lib
+ cargo clean
     Removed 190 files, 74.5MiB total
+ rustup run RustMC cargo rustc -- -C save-temps --emit=llvm-ir
   Compiling libc v0.2.175
   Compiling parking_lot_core v0.9.11 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/core)
   Compiling cfg-if v1.0.3
   Compiling smallvec v1.15.1
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.13 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/lock_api)
   Compiling parking_lot v0.12.4 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
+ cd target/debug/deps
+ rm parking_lot-3044f5a8a58b0879.5ij3rymeerw9h7j9qhmejxqoq.rcgu.no-opt.bc
+ find . -name '*.ll' -print0
+ xargs -0 -n1 sed -i -e s/DIFlagFixedEnum/DIFlagEnumClass/g
+ find . -name '*.ll' -print0
+ xargs -0 -n1 /usr/lib/llvm-18//bin/llvm-as
+ find . -name '*.rcgu.ll' -print0
+ xargs -0 -n1 /usr/lib/llvm-18//bin/llc -filetype=obj
++ find . -maxdepth 1 -type f -name 'lib*.rlib' '!' -name 'libparking_lot-*.rlib' -printf './%f '
++ find /home/ollie/.rustup/toolchains/RustMC/lib -type f -name '*.rlib' -print
+ cc -no-pie -m64 -Wl,--gc-sections -L /home/ollie/.rustup/toolchains/RustMC/lib parking_lot-3044f5a8a58b0879.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o ./libsmallvec-fe3818687bdf6112.rlib ./liblock_api-6cab3f733191b133.rlib ./libscopeguard-42c356dacfe76985.rlib ./liblibc-e4ecee72ddb8de47.rlib ./libcfg_if-d3a0653d2626b08e.rlib ./libparking_lot_core-15973c20bf9122fb.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-6f581d9f2b6a379c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-703d4206c6fa558e.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c53ac2f50da4acb9.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-529014828ebfad92.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-8f53f212bbbe7a84.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-df0c49b4e2c37ae6.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsysroot-b854303479fafd3c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-c9a5d6f98d3d3ed3.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2831390875775ada.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-044615360d9e10f1.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-16e1e33c4b47e45d.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-a78a55a26fd015fe.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a19e33c1d17f7f02.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-55b5166dfa314382.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-bd0375ffd1a54e5a.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-6a1ece7e2b288276.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a977874a8096e240.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acf5f2b45cc0ef9c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-af00b2f7fb9077af.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-f3b9553059bae66f.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-13c19ccb33815b11.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-0df313335706c93c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-24bc2637c8c0f940.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-6c5e9e2ad7378181.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-fc268355879ea7e9.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-4693606b287d8e81.rlib -lpthread -ldl -lresolv -lm -lc -o /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/complicated_app_output
/usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/13/../../../x86_64-linux-gnu/crt1.o: in function `_start':
(.text+0x1b): undefined reference to `main'
collect2: error: ld returned 1 exit status
