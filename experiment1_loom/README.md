# Experiment 1 (loom tests)

To replicate the results in Table 1, run the following command:

```
./run_experiment1.py --csv test_inventory_artifact.csv --ported-tests loom-tests-ported/
```

This will run RustMC on each tests in the inventory and store the results in `experiment_results.csv`. You can view the csv file in a human friendly way with

```
mlr --icsv --opprint cat  experiment_results.csv 
```

If you'd like to run an invidivual file for further investigation, run 

```
./verify_single.sh loom-tests-ported/arc_ported_genmc.rs 
```

You will find the results of the verification in `test_traces/` and `test_results`.



