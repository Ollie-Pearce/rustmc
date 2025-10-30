# Introduction

Claims / badges we're aiming for.

# Smoke tests

small version of both experiments and what you should see 

- one use case
- small Exp 1
- one small crate from Exp 2

# Use cases

## Figures from paper

## Writing your own examples

# Experiment 1 (loom tests)

To replicate the results in Table 1, navigate to `experiment1_loom/`run the following command:

```
./run_experiment1.py --csv test_inventory_artifact.csv --ported-tests loom-tests-ported/
```

This will run RustMC on each test in the inventory and store the results in `experiment_results.csv`. You can view the csv file in a human-readable way with

```
mlr --icsv --opprint cat  experiment_results.csv 
```

If you'd like to run an individual file for further investigation, run 

```
./verify_single.sh loom-tests-ported/arc_ported_genmc.rs 
```

You will find the results of the verification in `test_traces/` and `test_results/`.



---

# Experiment 2 (Crates)

To replicate the results in Table 2, navigate to `experiment2_crates/` and run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report the results in the `test_results` directory


In order to verify tests from a single crate run

```
./verify_crate CRATE
```

Traces of test executions can be found in `test_traces`, by default we do not print the execution graph as this slows down verification significantly however a full trace of each execution can be printed by adding a `--print-exec-graphs` to the script used to verify a crate. Results are output to the `test_results` directory. 

For a quick litmus test we recommend running the archery or thread_local crates with the above `verify_crate` script

### Edge cases

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

## Verifying new crates:

In order to run RustMC on ... several steps must be taken #



1. Add the crate to `verify_test_benchmarks`
2. Copy the `verify_tests_script` and add new IR files from the dependencies of the crate to verify
3. If any stdlib functions are external you can clone the `Ollie-Pearce/rust` repo, manually add the `#[inline(always)]` attribute to them and run `./x build library` to build the standard library
4. With a rust toolchain built with ` ./x build library` external symbols from the standard library can be identified by running the `find_symbol` script and ...






---

Due to the ... resources required to link, transform and verify large LLVM modules this experiment may require significant system resources. Our tests were run with ...
