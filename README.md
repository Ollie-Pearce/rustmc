# Introduction

This paper introduces RustMC, a tool to detect errors in concurrent
programs written in Rust.

The purpose of this artifact is to allow the reviewer to verify the main claims we make in the paper:

1. RustMC works as expected for the various examples given in the paper (Figures 1, 5, 6, 9, and 10).

2. Experiment 1 (Table 1) is reproducable

3. Experiment 2 (Figures 7 and 8) are reproducable


We aim for all badges: Available (via our zenodo link), Functional
(see points 1-3 above), Reusable (code is documented and we provide
documentation for adding new benchmarks).


# Early smoke tests


### Verify one use case

Run `cd paper && ./run_figure1.sh`

In `benchmark_results/figure1_output.txt` you should see an execution trace and an error message indicating the race in this program was detected.

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

### Run a sample of Experiment 1

Run `cd experiment1_loom/ && ./run_experiment1.py --csv test_inventory_artifact_small.csv` (~15 seconds). It should end with
  
 ```
================================================================================
SUMMARY
================================================================================
Total tests: 7
Matches expected: 5
Panics found: 0
```

### Run a sample of Experiment 2

Run `cd experiment2_crates/ && ./verify_crate boxcar`

In `test_traces/boxcar` you should see the results of each test and in `test_results/boxcar` you should see the following:

```
Verification success: 11 / 11
Panic called: 0 / 11
Uninitialised read errors: 0 / 11
No entry point errors: 0 / 11
External function errors: 0 / 11
AtomicRMW errors: 0 / 11
External address errors: 0 / 11
constant unimplemented errors: 0 / 11
external var args errors: 0 / 11
Memset promotion errors: 0 / 11
memcpy errors: 0 / 11
segmentation fault errors: 0 / 11
```

# Experiment 1 (loom tests)

To replicate the results in Table 1, navigate to `experiment1_loom/` and run the following command (takes ~15 minutes):

```
./run_experiment1.py
```

This will run RustMC on each test in the inventory and store the results in `experiment_results.csv`. If all goes well, the script should end with:

```
Results saved to: experiment_results.csv

================================================================================
SUMMARY
================================================================================
Total tests: 72
Matches expected: 49
Panics found: 17
```


You can view the csv file in a human-readable way with

```
mlr --icsv --opprint cat  experiment_results.csv 
```


NB: compared to submitted paper, we have added a missing test which is
analysed successfully (test `mutex_into_inner` in file
`mutex`). Following an improvement of our tool chain, four tests that
use to _wrongly_ panic, now crash due to an unknown external function
(tests `initial_thread`, `threads_have_unique_ids`, `thread_names`,
`park_unpark` in file ` thread_api`). While this changes some numbers,
our overall conclusions remain the same.  The improvement involve
using statics from the stdlib rather than having them
zero-initialized.


If you'd like to run an individual file for further investigation, run 

```
./verify_single.sh loom-tests-ported/arc_ported_genmc.rs 
```

You will find the results of the verification in `test_traces/` and `test_results/`.


---

# Experiment 2 (Crates)
Due to the significant demands imposed by linking, transforming and verifying large LLVM modules this experiment may require significant system resources. All our tests were run inside the provided Docker container on a machine running Ubuntu 22.04.5 LTS with the following specifications: 
`CPU = Intel(R) Xeon(R) Silver 4410Y @ 2.0GHz (Sapphire Rapids), Cores (Physical) 48 (24), Memory: 128GB`

To replicate the results in Table 2, navigate to `experiment2_crates/` and run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report the results in the `test_results` directory

Some of the crates we use for testing contain many dependencies which makes linking transforming and interpreting the necessary LLVM bitcode demanding, to run a smaller set of benchmarks use the `--run_small` flag, this will only run tests which took less than half an hour to verify in our benchmarks which can be found in the "Time to verify" section.




In order to verify tests from a single crate run

```
./verify_crate CRATE
```

Traces of test executions can be found in `test_traces`, by default we do not print the execution graph as this slows down verification significantly however a full trace of each execution can be printed by adding a `--print-exec-graphs` to the script used to verify a crate. Results are output to the `test_results` directory. 

For a quick litmus test we recommend running the archery or thread_local crates with the above `verify_crate` script

### Edge cases

#### Tests with an excessive state space

The following tests have a very large state space and could not be verified in an hour, we have commented them out however if you would like to reactivate them you can uncomment them in the following files:

- parking_lot/src/mutex.rs | `lots_and_lots_1_1`
- parking_lot/src/fair_mutex.rs | `lots_and_lots_1_1_1`
- parking_lot/src/rwlock.rs | `test_ruw_arc`
- spin-rs/src/mutex/ticket.rs | `lots_and_lots`
- spin-rs/src/mutex/spin.rs | `lots_and_lots_2`

#### Tests which panic

The following tests call an external function in order to initiate a panic. We consider this a successful verification as they either have the `#[should_panic]` attribute or intentionally panic.

##### arcstr
```
test_from_parts_unchecked_err, repeat_string_panics, test_substr_using_panic0, test_substr_using_panic1, test_substr_using_panic2
```

##### spin
```
panic, mutex_arc_access_in_unwind, rw_access_in_unwind
```

##### parking_lot
```
test_mutex_arc_access_in_unwind_1_1_1, test_mutex_arc_access_in_unwind_1_1, test_rw_arc_no_poison_wr, test_rw_arc_no_poison_wr, test_rw_arc_no_poison_rr, test_rw_arc_no_poison_rw, test_rw_arc_access_in_unwind, poison_bad, wait_for_force_to_finish
```

##### arc-swap
```
panic_with_hook
```

### Time to verify:
All times were taken from the `time ./verify_crate` command and include building, linking, transformation and verification

- atomic_float: 0m6.746s
- spin: 12m4.211s
- ringbuf: 0m15.755s
- seize: 32m55.264s
- thread_local: 0m18.491s
- parking: 0m19.504s
- arcstr 1m46.961s
- arc-swap 3m31.588s
- state: 0m39.276s
- try_lock: 0m1.576s
- parking_lot: 1h24m16.143s
- archery: 6m30.792s



NB: Following the previously mentioned improvements to our toolchain and a change to the verification driver script for the archery crate we now support an additional archery test and 10 addition spin tests. Unfortunately this change disabled support for try-lock's `fmt_debug()` test. 


## Verifying new crates (re-usability):

The following several steps may be taken in order to run RustMC on a crate outside our benchmark set 



1. Add the crate to `verify_test_benchmarks`

2. Copy the `verify_tests_script` and add new IR files from the dependencies of the crate to verify

3. If any rust standard library functions are external you can add the `#[inline(always)]` attribute to the corresponding function in `rust/library` directory.

4. Sometimes crates will make use of external statics from the standard library. In order to define these statics, first build the toolchain with `RUSTFLAGS="-C embed-bitcode" ./x build library` this will also set any functions in the `library/` directory you have applied the inline attribute to appear in the llvm-ir generated by the rust compiler. Then move the scripts in the `extract_externals_from_stdlib_scripts` directory into the custom toolchain's `rust/build/x86_64-unknown-linux-gnu` directory, to retrieve the llvm-ir for the external symbols you require:
    - Run the `find_symbol` script with the external static you require as an argument
    - Compile the identified rlibs into llvm-ir using the `extract_ir_from_rlib` script and add the external statics from the resulting module to `override/my_pthread.ll`.

5. Run `rustup toolchain link RustMC /rust/build/x86_64-unknown-linux-gnu/stage1` in order to link your newly built toolchain.

6. A driver script is needed in order to:
    - Identify test functions 
    - Link together the prerequisite llvm bitcode files into a standalone module 
    - Run the verification component on the resulting module, iterating over any identified test functions as entry points.

  We suggest using the `verify_tests.sh` script as a starting point. This script links a few common dependencies in order to resolve external function errors but does not link all of a crates llvm bitcode modules together as this can lead to a significant performance slowdown due to the due to the complexity of transforming, interpreting, and verifying large-scale LLVM modules.


# Use cases
Todo: I think move this below experiment 2
## Figures from paper

The bug reproductions described in various figures in the paper can be found in the `paper_use_cases/` directory. To verify all of the snippets included in the figures of the paper, run `./run_all_figures`. To run an individual snippet run the corresponding script for the snippet's figure number in the paper, e.g. `./run_figure1.sh` in order to verify a program containing the data race bug described in figure 1. Results are output in the `benchmark_results` directory.



## Writing your own examples (re-usability)

You can follow the below steps in order to create a Rust program which can be verified by RustMC:
- use `cargo new your_project`
- Edit the `Cargo.toml` file and set edition to `edition = "2021"`
- In `main.rs`, add the following attributes to the top of the file: 
    ```
    #![no_main]
    #![feature(start)]
    #![feature(thread_spawn_unchecked)]
    #![no_builtins]
    ```
- Add a `start` function with the following definition:
```
#[start]
#[no_mangle]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}
```
- Add a main function with the following definition:
```
#[no_mangle]
fn main() -> i32 {
    0
}
```
- In order to run RustMC on the program you will need to link the bitcode files produced by rust's `--emit=llvm-bc` flag and provide this as input using the `--program-entry-function=main` flag. It should be simple enough to adapt one of the existing scripts for running one of our use cases in order to achieve this.

# High-level description of source

- Mixer/GenMC
- Rust tool chain
- Experiment 1, Loom tests
- Experiment 2, crates
- Use cases from paper
  
  



---


