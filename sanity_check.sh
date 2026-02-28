#!/bin/bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "============================================"
echo "  Sanity Check"
echo "============================================"

# a) Run figure1 from paper_use_cases
echo ""
echo "[1/3] Running paper_use_cases figure1..."
echo "--------------------------------------------"
"$SCRIPT_DIR/paper_use_cases/verify_figure.sh" figure1 > /dev/null 2>&1

# b) Verify arc_ported_genmc.rs in experiment1_loom
echo ""
echo "[2/3] Verifying arc_ported_genmc.rs (experiment 1)..."
echo "--------------------------------------------"
cd "$SCRIPT_DIR/experiment1_loom"
./verify_single.sh loom-tests-ported/arc_ported_genmc.rs > /dev/null 2>&1

# c) Verify the boxcar crate in experiment2_crates
echo ""
echo "[3/3] Verifying boxcar crate (experiment 2)..."
echo "Takes ~2 minutes"
echo "--------------------------------------------"
cd "$SCRIPT_DIR/experiment2_crates"
./verify_crate boxcar 

echo ""
echo "============================================"
echo "  Sanity check complete!"
echo "============================================"
echo ""
echo "Results can be found in:"
echo "  Figure 1:          $SCRIPT_DIR/paper_use_cases/results/figure1_output.txt"
echo " "
echo "  arc_ported_genmc:  $SCRIPT_DIR/experiment1_loom/test_traces/arc_ported_genmc/"
echo "                     $SCRIPT_DIR/experiment1_loom/test_results/arc_ported_genmc_all_summary.txt"
echo " "
echo "  boxcar crate:      $SCRIPT_DIR/experiment2_crates/test_results/boxcar_summary.txt"
echo "                     $SCRIPT_DIR/experiment2_crates/test_traces/boxcar/"
