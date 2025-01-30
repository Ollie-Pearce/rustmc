


To build the Rust std library, GenMC and an example rust program:

1) Installing Rust source:

    - navigate to RustMC/rust and execute "./x build library"

    - You may need to run "./x setup" first

    - next run "rustup toolcahin link RustMC build/x86_64-unknown-linux-gnu/stage1

2) Building genmc and the Rust program:

    - Navigate to RustMC/genmc_for_rust/genmc

    - Run:
        	autoreconf --install
	        ./configure
	        make



    - To test GenMC on a Rust program use run.sh, this script:

        - Compiles a benchmark to llvm bytecode

        - Links this bytecode into an LLVM-IR module. Overriding system threading functions.

        - Runs genmc on the LLVM-IR module


######################################################################################
 
In order to run a build genmc on the working test case (with manually stored undef values) run:

    - sh run_working_case.sh
