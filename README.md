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

To test that all is working as expected, you can do the following


### Verify one use case

TODO: Ollie pick an example (Fig 1?) and show expected output

### Run a sample of Experiment 1

Run `cd experiment1_loom/ && ./run_experiment1.py --csv test_inventory_artifact_small.csv` (~15 seconds). It should end with
  
 ```
================================================================================
SUMMARY
================================================================================
Total tests: 7
Matches expected: 5
Panics found: 0
Crashes: 1
Successes: 6
Errors: 0
```

### Run a sample of Experiment 2

TODO: Ollie pick a small crate as an example (and show expected output)



# Use cases

## Figures from paper

TODO: Ollie say how to run each/all examples

## Writing your own examples (re-usability)

TODO: Ollie explain briefly how to make your own example (or modify existing one)

# Experiment 1 (loom tests)

To replicate the results in Table 1, navigate to `experiment1_loom/`run the following command (takes ~15 minutes):

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
Crashes: 14
Successes: 39
Errors: 0
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

All tests were run on a machine with the following specifications: 
`CPU = Intel(R) Xeon(R) Silver 4410Y @ 2.0GHz (Sapphire Rapids), Cores (Physical) 48 (24), Memory: 128GB`

To replicate the results in Table 2, navigate to `experiment2_crates/` and run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report the results in the `test_results` directory

Some of the crates we use for testing contain many dependencies which makes linking and transforming the necessary LLVM bitcode demanding, to run a smaller set of benchmarks use the `--run_small` flag.




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
- spin-rs/src/mutex/ticket.rs | `lots_and_lots`
- spin-rs/src/mutex/spin.rs | `lots_and_lots_2`

#### Tests which panic

The following tests call the external `rust_begin_unwind` function in order to initiate a panic. We consider this a successful verification as they either have the `#[should_panic]` attribute or test....(ability to unwind after a panic?):

##### arcstr

```
test_from_parts_unchecked_err, repeat_string_panics, test_substr_using_panic0, test_substr_using_panic1, test_substr_using_panic2
```



##### spin

#### `panic, mutex_arc_access_in_unwind, rw_access_in_unwind`



##### parking_lot

`test_mutex_arc_access_in_unwind_1_1_1, test_mutex_arc_access_in_unwind_1_1, test_rw_arc_no_poison_wr, test_rw_arc_no_poison_wr, test_rw_arc_no_poison_rr, test_rw_arc_no_poison_rw, test_rw_arc_access_in_unwind, poison_bad, wait_for_force_to_finish`

##### arc-swap

`panic_with_hook`

### Time taken to verify crates:





## Verifying new crates (re-usability):

In order to run RustMC on ... several steps must be taken #



1. Add the crate to `verify_test_benchmarks`
2. Copy the `verify_tests_script` and add new IR files from the dependencies of the crate to verify
3. If any stdlib functions are external you can clone the `Ollie-Pearce/rust` repo, manually add the `#[inline(always)]` attribute to them and run `./x build library` to build the standard library
4. With a rust toolchain built with ` ./x build library` external symbols from the standard library can be identified by running the `find_symbol` script and ...


# High-level description of source

- Mixer/GenMC
- Rust tool chain
  
  



---

Due to the ... resources required to link, transform and verify large LLVM modules this experiment may require significant system resources. Our tests were run with ...
