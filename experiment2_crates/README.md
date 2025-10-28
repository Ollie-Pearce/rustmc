To replicate the results in Table 2, run the following command:

```
./run_experiment2 
```

This will run RustMC on all the tests in the sample set and report the results in `experiment2_results.csv`


In order to verify tests from a single crate run

```
./verify_crate.sh /verify_test_benchmarks/CRATE
```

Traces of test executions can be found in `test_traces` and results can be found in `test_results`


---

Due to the ... resources required to link, transform and verify large LLVM modules this experiment may require significant system resources. Our tests were run with ...
