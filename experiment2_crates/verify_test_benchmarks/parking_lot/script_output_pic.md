++ pwd
+ OUTPUT_DIR=/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot
+ LLVM_HOME=/usr/lib/llvm-18
+ RUSTUP_TOOLCHAIN_LIB=/home/ollie/.rustup/toolchains/RustMC/lib
+ CRATE=parking_lot
+ cargo clean
     Removed 360 files, 482.7MiB total
+ RUSTFLAGS='-C embed-bitcode=yes -C save-temps -C relocation-model=pic --emit=llvm-ir -C lto=fat'
+ rustup run RustMC cargo test -p parking_lot --no-run
   Compiling libc v0.2.175
   Compiling cfg-if v1.0.3
   Compiling zerocopy v0.8.26
   Compiling parking_lot_core v0.9.11 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/core)
   Compiling serde v1.0.219
   Compiling scopeguard v1.2.0
   Compiling smallvec v1.15.1
   Compiling lock_api v0.4.13 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/lock_api)
   Compiling getrandom v0.2.16
   Compiling rand_core v0.6.4
   Compiling parking_lot v0.12.4 (/home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot)
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling bincode v1.3.3
    Finished `test` profile [unoptimized + debuginfo] target(s) in 28.96s
  Executable unittests src/lib.rs (target/debug/deps/parking_lot-dbf9a237c9fd6fb0)
  Executable tests/issue_203.rs (target/debug/deps/issue_203-73fa2afbc312a6e5)
  Executable tests/issue_392.rs (target/debug/deps/issue_392-f9372bf52fb4a451)
+ cd target/debug/deps
+ rm -f bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.no-opt.bc cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.no-opt.bc getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.no-opt.bc issue_203-73fa2afbc312a6e5.db729nmclaq98fskf2fbfphc1.rcgu.no-opt.bc issue_392-f9372bf52fb4a451.9n6xunkv5fg1y89krc3yyljyq.rcgu.no-opt.bc libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.no-opt.bc lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.no-opt.bc parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.no-opt.bc parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.no-opt.bc parking_lot-dbf9a237c9fd6fb0.b3jnj3qo4aocbxzn9d8zmnnos.rcgu.no-opt.bc ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.no-opt.bc rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.no-opt.bc rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.no-opt.bc rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.no-opt.bc scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.no-opt.bc serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.no-opt.bc smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.no-opt.bc zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.no-opt.bc
+ find . -name '*.ll' -print0
+ xargs -0 -n1 sed -i -e s/DIFlagFixedEnum/DIFlagEnumClass/g
+ find . -name '*.ll' -print0
+ xargs -0 -n1 /usr/lib/llvm-18/bin/llvm-as
++ mktemp -d
+ TMP_BC_DIR=/tmp/tmp.6JjFTIVAfL
+ find . -maxdepth 1 -type f -name 'lib*.rlib' -print0
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libsmallvec-4e7f5b90e0767ed6.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libgetrandom-1a58de3ca32d1801.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand-447650d53fc3a237.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libcfg_if-f3caa95b25fe572a.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libserde-259250a215ec60b2.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libppv_lite86-c6e2dfd00251267c.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libbincode-115b023db540a9da.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand_core-efffbb197ab8c2f9.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libzerocopy-781b6db9e77ef4c2.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./liblibc-88736dc2266de98c.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libparking_lot_core-bb84ce9d129accef.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./liblock_api-75e72ec437bf20f2.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand_chacha-80714fc6572bae49.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libparking_lot-d2af7ab1d64b2a4d.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libscopeguard-2ceb237c866ef480.rlib
+ grep -F .bc
+ read -r M
+ IFS=
+ read -r -d '' A
+ find . -maxdepth 1 -type f -name 'lib*.rlib' -print0
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libsmallvec-4e7f5b90e0767ed6.rlib
+ grep -F .o
+ read -r O
++ basename ./libsmallvec-4e7f5b90e0767ed6.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libsmallvec-4e7f5b90e0767ed6
+ /usr/lib/llvm-18/bin/llvm-ar x ./libsmallvec-4e7f5b90e0767ed6.rlib smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o
++ basename ./libsmallvec-4e7f5b90e0767ed6.rlib .rlib
++ basename ./libsmallvec-4e7f5b90e0767ed6.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libsmallvec-4e7f5b90e0767ed6/libsmallvec-4e7f5b90e0767ed6.smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o.bc smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o
+ rm -f smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libgetrandom-1a58de3ca32d1801.rlib
+ grep -F .o
+ read -r O
++ basename ./libgetrandom-1a58de3ca32d1801.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libgetrandom-1a58de3ca32d1801
+ /usr/lib/llvm-18/bin/llvm-ar x ./libgetrandom-1a58de3ca32d1801.rlib getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o
++ basename ./libgetrandom-1a58de3ca32d1801.rlib .rlib
++ basename ./libgetrandom-1a58de3ca32d1801.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libgetrandom-1a58de3ca32d1801/libgetrandom-1a58de3ca32d1801.getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o.bc getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o
+ rm -f getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand-447650d53fc3a237.rlib
+ grep -F .o
+ read -r O
++ basename ./librand-447650d53fc3a237.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/librand-447650d53fc3a237
+ /usr/lib/llvm-18/bin/llvm-ar x ./librand-447650d53fc3a237.rlib rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o
++ basename ./librand-447650d53fc3a237.rlib .rlib
++ basename ./librand-447650d53fc3a237.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/librand-447650d53fc3a237/librand-447650d53fc3a237.rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o.bc rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o
+ rm -f rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libcfg_if-f3caa95b25fe572a.rlib
+ grep -F .o
+ read -r O
++ basename ./libcfg_if-f3caa95b25fe572a.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libcfg_if-f3caa95b25fe572a
+ /usr/lib/llvm-18/bin/llvm-ar x ./libcfg_if-f3caa95b25fe572a.rlib cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o
++ basename ./libcfg_if-f3caa95b25fe572a.rlib .rlib
++ basename ./libcfg_if-f3caa95b25fe572a.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libcfg_if-f3caa95b25fe572a/libcfg_if-f3caa95b25fe572a.cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o.bc cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o
+ rm -f cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libserde-259250a215ec60b2.rlib
+ grep -F .o
+ read -r O
++ basename ./libserde-259250a215ec60b2.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libserde-259250a215ec60b2
+ /usr/lib/llvm-18/bin/llvm-ar x ./libserde-259250a215ec60b2.rlib serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o
++ basename ./libserde-259250a215ec60b2.rlib .rlib
++ basename ./libserde-259250a215ec60b2.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libserde-259250a215ec60b2/libserde-259250a215ec60b2.serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o.bc serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o
+ rm -f serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libppv_lite86-c6e2dfd00251267c.rlib
+ grep -F .o
+ read -r O
++ basename ./libppv_lite86-c6e2dfd00251267c.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libppv_lite86-c6e2dfd00251267c
+ /usr/lib/llvm-18/bin/llvm-ar x ./libppv_lite86-c6e2dfd00251267c.rlib ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o
++ basename ./libppv_lite86-c6e2dfd00251267c.rlib .rlib
++ basename ./libppv_lite86-c6e2dfd00251267c.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libppv_lite86-c6e2dfd00251267c/libppv_lite86-c6e2dfd00251267c.ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o.bc ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o
+ rm -f ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libbincode-115b023db540a9da.rlib
+ grep -F .o
+ read -r O
++ basename ./libbincode-115b023db540a9da.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libbincode-115b023db540a9da
+ /usr/lib/llvm-18/bin/llvm-ar x ./libbincode-115b023db540a9da.rlib bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o
++ basename ./libbincode-115b023db540a9da.rlib .rlib
++ basename ./libbincode-115b023db540a9da.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libbincode-115b023db540a9da/libbincode-115b023db540a9da.bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o.bc bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o
+ rm -f bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand_core-efffbb197ab8c2f9.rlib
+ grep -F .o
+ read -r O
++ basename ./librand_core-efffbb197ab8c2f9.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/librand_core-efffbb197ab8c2f9
+ /usr/lib/llvm-18/bin/llvm-ar x ./librand_core-efffbb197ab8c2f9.rlib rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o
++ basename ./librand_core-efffbb197ab8c2f9.rlib .rlib
++ basename ./librand_core-efffbb197ab8c2f9.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/librand_core-efffbb197ab8c2f9/librand_core-efffbb197ab8c2f9.rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o.bc rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o
+ rm -f rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libzerocopy-781b6db9e77ef4c2.rlib
+ grep -F .o
+ read -r O
++ basename ./libzerocopy-781b6db9e77ef4c2.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libzerocopy-781b6db9e77ef4c2
+ /usr/lib/llvm-18/bin/llvm-ar x ./libzerocopy-781b6db9e77ef4c2.rlib zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o
++ basename ./libzerocopy-781b6db9e77ef4c2.rlib .rlib
++ basename ./libzerocopy-781b6db9e77ef4c2.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libzerocopy-781b6db9e77ef4c2/libzerocopy-781b6db9e77ef4c2.zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o.bc zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o
+ rm -f zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./liblibc-88736dc2266de98c.rlib
+ grep -F .o
+ read -r O
++ basename ./liblibc-88736dc2266de98c.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/liblibc-88736dc2266de98c
+ /usr/lib/llvm-18/bin/llvm-ar x ./liblibc-88736dc2266de98c.rlib libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o
++ basename ./liblibc-88736dc2266de98c.rlib .rlib
++ basename ./liblibc-88736dc2266de98c.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/liblibc-88736dc2266de98c/liblibc-88736dc2266de98c.libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o.bc libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o
+ rm -f libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libparking_lot_core-bb84ce9d129accef.rlib
+ grep -F .o
+ read -r O
++ basename ./libparking_lot_core-bb84ce9d129accef.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libparking_lot_core-bb84ce9d129accef
+ /usr/lib/llvm-18/bin/llvm-ar x ./libparking_lot_core-bb84ce9d129accef.rlib parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o
++ basename ./libparking_lot_core-bb84ce9d129accef.rlib .rlib
++ basename ./libparking_lot_core-bb84ce9d129accef.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libparking_lot_core-bb84ce9d129accef/libparking_lot_core-bb84ce9d129accef.parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o.bc parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o
+ rm -f parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./liblock_api-75e72ec437bf20f2.rlib
+ grep -F .o
+ read -r O
++ basename ./liblock_api-75e72ec437bf20f2.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/liblock_api-75e72ec437bf20f2
+ /usr/lib/llvm-18/bin/llvm-ar x ./liblock_api-75e72ec437bf20f2.rlib lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o
++ basename ./liblock_api-75e72ec437bf20f2.rlib .rlib
++ basename ./liblock_api-75e72ec437bf20f2.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/liblock_api-75e72ec437bf20f2/liblock_api-75e72ec437bf20f2.lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o.bc lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o
+ rm -f lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./librand_chacha-80714fc6572bae49.rlib
+ grep -F .o
+ read -r O
++ basename ./librand_chacha-80714fc6572bae49.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/librand_chacha-80714fc6572bae49
+ /usr/lib/llvm-18/bin/llvm-ar x ./librand_chacha-80714fc6572bae49.rlib rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o
++ basename ./librand_chacha-80714fc6572bae49.rlib .rlib
++ basename ./librand_chacha-80714fc6572bae49.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/librand_chacha-80714fc6572bae49/librand_chacha-80714fc6572bae49.rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o.bc rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o
+ rm -f rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libparking_lot-d2af7ab1d64b2a4d.rlib
+ grep -F .o
+ read -r O
++ basename ./libparking_lot-d2af7ab1d64b2a4d.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libparking_lot-d2af7ab1d64b2a4d
+ /usr/lib/llvm-18/bin/llvm-ar x ./libparking_lot-d2af7ab1d64b2a4d.rlib parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o
++ basename ./libparking_lot-d2af7ab1d64b2a4d.rlib .rlib
++ basename ./libparking_lot-d2af7ab1d64b2a4d.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libparking_lot-d2af7ab1d64b2a4d/libparking_lot-d2af7ab1d64b2a4d.parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o.bc parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o
+ rm -f parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-ar t ./libscopeguard-2ceb237c866ef480.rlib
+ grep -F .o
+ read -r O
++ basename ./libscopeguard-2ceb237c866ef480.rlib .rlib
+ mkdir -p /tmp/tmp.6JjFTIVAfL/libscopeguard-2ceb237c866ef480
+ /usr/lib/llvm-18/bin/llvm-ar x ./libscopeguard-2ceb237c866ef480.rlib scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o
++ basename ./libscopeguard-2ceb237c866ef480.rlib .rlib
++ basename ./libscopeguard-2ceb237c866ef480.rlib .rlib
+ /usr/lib/llvm-18/bin/llvm-objcopy --dump-section .llvmbc=/tmp/tmp.6JjFTIVAfL/libscopeguard-2ceb237c866ef480/libscopeguard-2ceb237c866ef480.scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o.bc scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o
+ rm -f scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o
+ read -r O
+ IFS=
+ read -r -d '' A
+ /usr/lib/llvm-18/bin/llvm-link -o /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/parking_lot.whole.linked.bc ./parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.bc ./parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.bc /tmp/tmp.6JjFTIVAfL/libbincode-115b023db540a9da/libbincode-115b023db540a9da.bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libcfg_if-f3caa95b25fe572a/libcfg_if-f3caa95b25fe572a.cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libgetrandom-1a58de3ca32d1801/libgetrandom-1a58de3ca32d1801.getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/liblibc-88736dc2266de98c/liblibc-88736dc2266de98c.libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/liblock_api-75e72ec437bf20f2/liblock_api-75e72ec437bf20f2.lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libparking_lot_core-bb84ce9d129accef/libparking_lot_core-bb84ce9d129accef.parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libparking_lot-d2af7ab1d64b2a4d/libparking_lot-d2af7ab1d64b2a4d.parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libppv_lite86-c6e2dfd00251267c/libppv_lite86-c6e2dfd00251267c.ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/librand-447650d53fc3a237/librand-447650d53fc3a237.rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/librand_chacha-80714fc6572bae49/librand_chacha-80714fc6572bae49.rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/librand_core-efffbb197ab8c2f9/librand_core-efffbb197ab8c2f9.rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libscopeguard-2ceb237c866ef480/libscopeguard-2ceb237c866ef480.scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libserde-259250a215ec60b2/libserde-259250a215ec60b2.serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libsmallvec-4e7f5b90e0767ed6/libsmallvec-4e7f5b90e0767ed6.smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o.bc /tmp/tmp.6JjFTIVAfL/libzerocopy-781b6db9e77ef4c2/libzerocopy-781b6db9e77ef4c2.zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o.bc
+ /usr/lib/llvm-18/bin/llvm-link -o /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/parking_lot.whole.linked.bc ./parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.bc ./parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.bc
+ /usr/lib/llvm-18/bin/llvm-dis -o /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/parking_lot.whole.linked.ll /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/parking_lot.whole.linked.bc
+ find . -name '*.rcgu.ll' -print0
+ xargs -0 -n1 /usr/lib/llvm-18/bin/llc -relocation-model=pic -filetype=obj
++ find . -maxdepth 1 -type f -name 'lib*.rlib' '!' -name 'libparking_lot-*.rlib' -printf './%f '
++ find /home/ollie/.rustup/toolchains/RustMC/lib -type f -name '*.rlib' -print
+ cc -shared -m64 -Wl,--gc-sections -L /home/ollie/.rustup/toolchains/RustMC/lib bincode-115b023db540a9da.bincode.a2bc5b909205d591-cgu.0.rcgu.o cfg_if-f3caa95b25fe572a.cfg_if.dd46565cfb486caf-cgu.0.rcgu.o getrandom-1a58de3ca32d1801.getrandom.f943fbc89156f2ec-cgu.0.rcgu.o issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o issue_392-f9372bf52fb4a451.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o libc-88736dc2266de98c.libc.4063ef62bf32da2d-cgu.0.rcgu.o lock_api-75e72ec437bf20f2.4k0xcpka4u6bgy1an650acpe9.rcgu.o parking_lot_core-bb84ce9d129accef.60c36vdynhtlp7uczwaqqcahn.rcgu.o parking_lot-d2af7ab1d64b2a4d.5ij3rymeerw9h7j9qhmejxqoq.rcgu.o parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o ppv_lite86-c6e2dfd00251267c.ppv_lite86.ceb4a482cc0c7aa7-cgu.0.rcgu.o rand-447650d53fc3a237.rand.f4c812ff02f94c4a-cgu.0.rcgu.o rand_chacha-80714fc6572bae49.rand_chacha.5ea5bc5e14aed6e5-cgu.0.rcgu.o rand_core-efffbb197ab8c2f9.rand_core.c086b23c34023205-cgu.0.rcgu.o scopeguard-2ceb237c866ef480.scopeguard.c4bf227c17ddef65-cgu.0.rcgu.o serde-259250a215ec60b2.serde.1bd42af20419ebd7-cgu.0.rcgu.o smallvec-4e7f5b90e0767ed6.smallvec.1af5d5c2773757d3-cgu.0.rcgu.o zerocopy-781b6db9e77ef4c2.zerocopy.723ef8ed0aa73e4c-cgu.0.rcgu.o ./libsmallvec-4e7f5b90e0767ed6.rlib ./libgetrandom-1a58de3ca32d1801.rlib ./librand-447650d53fc3a237.rlib ./libcfg_if-f3caa95b25fe572a.rlib ./libserde-259250a215ec60b2.rlib ./libppv_lite86-c6e2dfd00251267c.rlib ./libbincode-115b023db540a9da.rlib ./librand_core-efffbb197ab8c2f9.rlib ./libzerocopy-781b6db9e77ef4c2.rlib ./liblibc-88736dc2266de98c.rlib ./libparking_lot_core-bb84ce9d129accef.rlib ./liblock_api-75e72ec437bf20f2.rlib ./librand_chacha-80714fc6572bae49.rlib ./libscopeguard-2ceb237c866ef480.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-6f581d9f2b6a379c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-703d4206c6fa558e.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c53ac2f50da4acb9.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-529014828ebfad92.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-8f53f212bbbe7a84.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-df0c49b4e2c37ae6.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsysroot-b854303479fafd3c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-c9a5d6f98d3d3ed3.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2831390875775ada.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-044615360d9e10f1.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-16e1e33c4b47e45d.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-a78a55a26fd015fe.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a19e33c1d17f7f02.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-55b5166dfa314382.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-bd0375ffd1a54e5a.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-6a1ece7e2b288276.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a977874a8096e240.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acf5f2b45cc0ef9c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-af00b2f7fb9077af.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-f3b9553059bae66f.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-13c19ccb33815b11.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-0df313335706c93c.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-24bc2637c8c0f940.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-6c5e9e2ad7378181.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-fc268355879ea7e9.rlib /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-4693606b287d8e81.rlib -o /home/ollie/Desktop/mixer/genmc-tool/verify_test_benchmarks/parking_lot/libparking_instrumented.so
/usr/bin/ld: issue_392-f9372bf52fb4a451.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o: in function `main':
test.35b686fe3b46fa23-cgu.00:(.text+0x8020): multiple definition of `main'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:test.35b686fe3b46fa23-cgu.00:(.text+0x2e1b0): first defined here
/usr/bin/ld: issue_392-f9372bf52fb4a451.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o: in function `rust_eh_personality':
/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: multiple definition of `rust_eh_personality'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: first defined here
/usr/bin/ld: issue_392-f9372bf52fb4a451.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: multiple definition of `std::sys::pal::unix::args::imp::ARGV_INIT_ARRAY'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: first defined here
/usr/bin/ld: parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o: in function `main':
test.35b686fe3b46fa23-cgu.00:(.text+0x9cb90): multiple definition of `main'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:test.35b686fe3b46fa23-cgu.00:(.text+0x2e1b0): first defined here
/usr/bin/ld: parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o: in function `rust_eh_personality':
/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: multiple definition of `rust_eh_personality'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: first defined here
/usr/bin/ld: parking_lot-dbf9a237c9fd6fb0.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: multiple definition of `std::sys::pal::unix::args::imp::ARGV_INIT_ARRAY'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: first defined here
/usr/bin/ld: /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acf5f2b45cc0ef9c.rlib(std-acf5f2b45cc0ef9c.std.9a85f25c70f8f1b4-cgu.07.rcgu.o): in function `rust_eh_personality':
/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: multiple definition of `rust_eh_personality'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/personality/gcc.rs:259: first defined here
/usr/bin/ld: /home/ollie/.rustup/toolchains/RustMC/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acf5f2b45cc0ef9c.rlib(std-acf5f2b45cc0ef9c.std.9a85f25c70f8f1b4-cgu.15.rcgu.o):/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: multiple definition of `std::sys::pal::unix::args::imp::ARGV_INIT_ARRAY'; issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o:/home/ollie/Desktop/RustMC/rust/library/std/src/sys/pal/unix/args.rs:150: first defined here
/usr/bin/ld: issue_203-73fa2afbc312a6e5.test-fc268355879ea7e9.test.35b686fe3b46fa23-cgu.00.rcgu.o.rcgu.o: relocation R_X86_64_TPOFF32 against `_ZN9issue_2031B29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h9d08b37a73019862E' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: failed to set dynamic section sizes: bad value
collect2: error: ld returned 1 exit status
