# Introduction

This paper introduces RustMC, a tool to detect errors in concurrent
programs written in Rust.

The purpose of this artefact is to allow the reviewer to verify the main claims we make in the paper:

1. Experiment 1 (Table 1) is reproducible

2. Experiment 2 (Figure 7) is reproducible

3. RustMC works as expected for the various examples given in the paper (Figures 1, 2, 8, 9, and 10).

4. Our artefact can be used to verify test cases in new crates beyond those described in the paper

We aim for the **Available**, **Functional**, and **Reusable** badges


The source code of our tool and our datasets are included in the
docker image and public repository. It includes

- The source code 
- The dataset for Experiment 1 (`experiment1_loom/loom-tests-ported/`)
- The dataset for Experiment 2 (`experiment2_crates/verify_test_benchmarks/`)
- Use cases from our paper (`paper_use_cases/`)


---

# Getting started

You will need Docker to run our artefact.



Download the [artefact from
Zenodo](https://doi.org/10.5281/zenodo.18806668)

To load the container, run

```
docker load < artefact.tar.gz
```

To run the container, execute:

```
docker run  --platform linux/amd64 -it rustmc:latest
```


All experiments were tested on the following machine:

```
- CPU = AMD Ryzen 9 5900X 12 Core Processor
- Cores (Physical) 24 (12)
- Memory: 48GB
- OS: Ubuntu 24.04.3 LTS 
```


---

# Early smoke tests

A `sanity_check.sh` script is provided to automate this process, step by step instuctions are also given below.
We assume the first commands are run from the home directory of the docker container.

### 1. Verify the example from Figure 1

Run `cd paper_use_cases && ./run_figure.sh 1`

In `results/figure1_output.txt` you should see an execution trace and an error message indicating the race in this program was detected. You can view the file with `more benchmark_results/figure1_output.txt`, and observe:

```
*** Compilation complete.
*** Transformation complete.
Final counter value: 10
Final counter value: 10
Error: Non-atomic race!
Event (3, 1) conflicts with event (2, 2) in graph:
<-1, 0> main:
	(0, 1): MALLOC _1
	(0, 2): Wna (, 0x0) main.rs:22
	(0, 3): Wna (, 0x0) main.rs:22
	(0, 4): Wna (, 0x0) main.rs:22
	(0, 5): MALLOC _4
...

*** Verification unsuccessful.
Number of complete executions explored: 0
Number of blocked executions seen: 1
Total wall-clock time: 2.86s
```

### 2. Run a sample of Experiment 1

Run `cd experiment1_loom/ && ./verify_single.sh loom-tests-ported/arc_ported_genmc.rs` (~15 seconds). It should end with
  
 ```

============================================
ALL VERIFICATIONS COMPLETE
============================================
Total functions verified: 6
Successful: 5
Failed: 1
```

### 3. Run a sample of Experiment 2

Run `cd experiment2_crates/ && ./verify_crate boxcar` (~2 minutes)

In `test_traces/boxcar` you should see the results of each test and in `test_results/boxcar_summary.txt` you should see the following:

```
Verification success: 11 / 11
Expected panic occurred: 0 / 11

ERRORS:
Unexpected panic: 0 / 11
Uninitialised heap read: 0 / 11
Could not identify test entry point: 0 / 11
External function errors: 0 / 11
Unsupported AtomicRMW operation on non-integer value: 0 / 11
External global errors: 0 / 11
Unsupported syscall: 0 / 11
Timeout: 0 / 11
Uncategorised errors: 0 / 11
```

---

# Replicating the results of the paper

## 1. Experiment 1 (loom tests)

To replicate the results in Table 1, navigate to `experiment1_loom/` and run the following command (takes ~15 minutes):

```
./run_experiment1.py
```

This will run RustMC on each test in the inventory and store the results in `experiment_results.csv`. The script should end with:

```
Results saved to: experiment_results.csv

----------------------------------------
SUMMARY
----------------------------------------
Total tests: 72
Matches expected: 60
Panics found: 16
```


You can view the csv file in a human-readable way with

```
mlr --icsv --opprint cat  experiment_results.csv 
```


If you'd like to run an individual file for further investigation, run 

```
./verify_single.sh loom-tests-ported/atomic_relaxed_ported_genmc.rs 
```

You will find the results of the verification in `test_traces/` and `test_results/`.


---

## 2. Experiment 2 (Crates)

To replicate the results in Table 2, navigate to `experiment2_crates/` and run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report a summary of each crate's results in the `test_results` directory.
The `collect_results.py` script outputs a summary of all test results across all crates. This script is called at the end of `run_experiment2` but can also be called independently.

After the experiment has completed you should see the following results:
```
========================================
SUMMARY
========================================
Total tests: 497
Successful verifications: 398
Expected panics: 18
Unexpected panics: 0
Timeouts: 0
Crashes: 81
```

Some of the crates we use for testing contain many dependencies which makes linking transforming and interpreting the necessary LLVM bitcode demanding, the entire experimennt takes around 1 hour and 45 minutes. To run a smaller set of benchmarks use the `--run-small` flag. 


In order to verify tests from a single crate run

```
./verify_crate CRATE
```

Traces of test executions can be found in `test_traces`, by default we do not print the execution graph as this slows down verification significantly however a full trace of each execution can be printed by adding a `--print-exec-graphs` to the script used to verify a crate. Results are output to the `test_results` directory. 

For a quick litmus test we recommend running the archery or thread_local crates with the above `verify_crate` script

### Edge cases

#### Tests with an excessive state space

The following tests have a very large state space and could not be
verified in an hour (they are considered as timeouts in Fig 8), we
have commented them out however if you would like to reactivate them
you can uncomment them in the following files:

- spin-rs/src/mutex/ticket.rs | `lots_and_lots` 
- spin-rs/src/mutex/spin.rs | `lots_and_lots_2` 
- spin-rs/src/mutex/fair | `lots_and_lots_1`
- arc-swap/src/lib.rs | `rcu`

#### Tests which panic

Some tests in the `arcstr`, `parking_lot` and `ringbuf` crates either have the `#[should_panic]` attribute or intentionally panic. We mark these tests as `Expected panics`

## 3. Verifying use cases and examples from the paper

The code corresponding to all figures in the paper can be found in the `paper_use_cases/` directory. 

To verify all of the snippets included in the figures of the paper, run 
```
./run_all_figures
```

To run an individual snippet, run 
```
./run_figure 1
```
Results are output in the `results` directory.

It is expected that Figure 2 exits with a `Thread panicked` message as the assert is violated by the atomicity violation and Figure 8 exits with an `attempt to access non-allocated memory` error because the data race causes an out-of-bounds access.

## 4. Verifying new crates:

For completeness, we give instructions to verify an additional crate (outside our benchmark set). 

1. **Identifying and downloading a compatible crate:**

- Find a version of the crate which supports Rust 1.81 on the <crates.io> package registry. Crates will usually either specify a Minimum Supported Rust Version (MSRV) or a supported Rust edition. 

If the MSRV listed is <=1.81 or the edition is <= 2018 the version should be supported by our toolchain. 

When you have found a suitable crate that supports Rust 1.81, navigate to `experiment2_crates/verify_test_benchmarks` and download the crate from the command line by running:
```
curl -H "User-Agent: my-app (your@email.com)" \
    https://crates.io/api/v1/crates/flume/0.12.0/download \
    -L -o flume.tar.gz
tar -xf flume.tar.gz
mv flume-0.12.0 flume
```

Replace `flume` and `0.12.0` with your own crate and version number. 

2. **Resolving dependencies:**
The Rust toolchain supported by our tool predates MSRV aware depdendency selection (discussed in this [RFC](https://github.com/rust-lang/rfcs/pull/3537)). As a result, even when a crate's MSRV is compatible with the toolchain cargo may attempt to use newer versions of dependencies which are not supported. 

The `lock_dependencies.sh` script can be used to resolve any incompatible dependencies. 
The script uses a more recent MSRV aware toolchain to generate a `Cargo.lock` file with compatible dependency versions.
If no MSRV is available dependencies will be resolved with respect to Rust version 1.81.

To compile the crate's tests using our supported toolchain navigate to the crate's directory and run:
```
rustup run nightly-2024-06-11-x86_64-unknown-linux-gnu cargo test
```

If the crate compiles successfully, please move on to step 3. If cargo introduces incompatible dependencies, see the below instructions.

From the `experiment2_crates/` directory run:
```
./lock_dependencies.sh verify_test_benchmarks/flume
```

Then test that the crate works with RustMC's supported toolchain by running:
```
cd verify_test_benchmarks/flume
rustup run nightly-2024-06-11-x86_64-unknown-linux-gnu cargo test
```

This should fix all dependencies to compatible versions.

Sometimes a crate's `Cargo.toml` specifies versions of dependencies which are not compatible with the stated MSRV. 
We have observed this occasionaly where developer dependencies are included.
In this case it is recommended to try a previous version of the crate.

3. **Verifying the crate:**
After successfully running `cargo test` with the supported toolchain the crate can be verified by running:

```
./verify_tests verify_test_benchmarks/flume
```

This script will:
- Enumerate all functions marked with the `#[test]` attribute.
- Add the `#[no_mangle]` attribute to all `#[test]` functions, renaming any which are duplicates.
- Compile the crate.
- Link the emitted LLVM bitcode together, overriding any `pthread` calls with internal wrapper functions which the verification component can intercept.
- Run GenMC on each test function, exploring all thread interleavings with a 1 hour timeout and a loop unroll bound of 2.
- Output traces in the `test_traces/` directory and a summary of results for all the crate's tests in `test_results/`.

A `--features` flag can be provided to provide flags for conditional compilation. 
Alternatively `--all-features` can be used to enable all conditional compilation features.


#### Common Errors:

**Crates failing to compile:**
- Our test harness does not currently support macro generated tests, if `verify_tests.sh` is run on a crate that uses procedural or declarative macros in order to generate tests the crate will fail to compile. To fix this temporarily remove any macro-generated tests. 
- Some crates use the `#![forbid(unsafe_code)]` attribute in order to prohibit the usage of any unafe code in the crate. The `#[no_mangle]` attribute used by `verify_tests` to identify test entry points is considered unsafe by this attribute. In this case please temporarily remove the attribute.
- Our toolchain does not support Rust's "Raw identifier" syntax for allowing reserved keywords to be used as an identifier. If compilation fails due to unexpected symbols in the headers of functions which start with `#r`, please remove these functions to compile.

**Reasons for test failure (error message in "test_traces"):**
- `Could not find program's entry point function`: RustMC could not find the definition of a function with the `#[test]` attribute in the crate's LLVM-IR. This is often caused by a missing conditional compilation flag, try looking in the `Cargo.toml` for a `[features]` heading. Features can be passed to the `verify_tests.sh` script using the `--features` flag.
- `External var arg function syscall is not supported by the interpreter`: Sometimes crates emit syscalls which we do not currently support.
- `Tried to execute an unkown external function`: Our tool does not currently support the `__rust_realloc` memory reallocation function and crashes if this function is called during interleaving exploration.
- `Uninitialised Read`: RustMC's undefined value transformation currently only supports stack allocations.
- `External global`: An undefined global is used somewhere in the LLVM module
