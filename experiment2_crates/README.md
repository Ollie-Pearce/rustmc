To replicate the results in Table 2, run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report the results in `experiment2_results.csv`


In order to verify tests from a single crate run

```
./verify_crate CRATE
```

Traces of test executions can be found in `test_traces`, by default we do not print the execution graph as this slows down verification significantly however a full trace of each execution can be printed by adding a `--print-exec-graphs` to the script used to verify a crate. Results are output to the `test_results` directory. 



The following tests have a very large statespace and could not be verified in an hour, we have commented them out however if yuo would like to reactivate them you can uncomment...





## Verifying New libraries:

In order to run RustMC on ... several steps must be taken #



1. Add the crate to `verify_test_benchmarks`
2. Copy the `verify_tests_script` and add new IR files from the dependencies of the crate to verify
3. If any stdlib functions are external you can clone the `Ollie-Pearce/rust` repo, manually add the `#[inline(always)]` attribute to them and run `./x build library` to build the standard library
4. With a rust toolchain built with ` ./x build library` external symbols from the standard library can be identified by running the `find_symbol` script and ...






---

Due to the ... resources required to link, transform and verify large LLVM modules this experiment may require significant system resources. Our tests were run with ...
