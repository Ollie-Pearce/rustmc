RustMC: Extending the GenMC
stateless model checker to Rust


To build the Rust std library, RustMC and verify a Rust program:

1) Installing Rust source:

    - Navigate to RustMC/rust and execute "./x setup"

    - select "b" for "compiler"

    - insert the following in config.toml 

        [llvm]
        download-ci-llvm = false

    - next run "rustup toolchain link RustMC build/x86_64-unknown-linux-gnu/stage1

2) Building genmc:
	- Navigate to RustMC/genmc_for_rust/genmc

	- Run:
        	autoreconf --install
	        ./configure
	        make

4) Verifying a Rust program:
	- Run:
  		sh verify.sh /path/to/rust/file.rs

	- Results will be output in rustmc/verification.txt 
