#!/bin/bash
# Shared RustMC verification-pipeline helpers, sourced by tools/cargo-rustmc
# (the `cargo rustmc test` front-end). Installed to
# $(pkgdatadir)/rustmc_pipeline.sh by `make install`.
#
# The caller MUST set, before invoking these functions:
#   GENMC_BIN    path to the genmc binary
#   OVERRIDE_LL  path to globals_override.ll
#   GENMC_FLAGS  array of genmc flags (from rustmc-config.sh)

# Run GenMC with --discover-rust-tests on a bitcode file.
# Discovers all tests, transforms once, verifies all, writes per-test trace files.
# Args: combined_bc trace_dir [expected_panics_file] [unsupported_tests_file]
run_genmc_discover() {
  local combined_bc="$1" trace_dir="$2" expected_panics="${3:-}" unsupported_tests="${4:-}"
  mkdir -p "$trace_dir"

  # Optional test-name filter, matching `cargo test [TESTNAME]`. Set by the
  # front-end (cargo-rustmc); genmc runs only tests whose demangled name
  # contains this substring.
  local filter_args=()
  [ -n "${RUST_TEST_FILTER:-}" ] && filter_args+=("--rust-test-filter=${RUST_TEST_FILTER}")

  local rc=0
  "${GENMC_BIN}" "${GENMC_FLAGS[@]}" \
    --discover-rust-tests \
    "${filter_args[@]}" \
    --per-test-timeout=3600 \
    --trace-output-dir="$trace_dir" \
    "$combined_bc" 2>&1 || rc=$?

  # Mark expected-panic tests that did panic. Trace files are named
  # <module>__..__<fn>__h<hash>; recover the test function name (the segment
  # immediately before the hash) and mark the trace if that function is
  # annotated #[should_panic].
  if [ -n "$expected_panics" ] && [ -s "$expected_panics" ]; then
    for trace_file in "$trace_dir"/*_verification.txt; do
      [ -f "$trace_file" ] || continue
      if grep -q "Thread panicked" "$trace_file" 2>/dev/null; then
        local trace_basename test_fn
        trace_basename=$(basename "$trace_file" _verification.txt)
        test_fn=${trace_basename%__h*}   # drop trailing __h<hash>
        test_fn=${test_fn##*__}          # last __-delimited segment = fn name
        if grep -qxF "$test_fn" "$expected_panics" 2>/dev/null; then
          echo "Expected panic occurred" >> "$trace_file"
        fi
      fi
    done
  fi

  # Mark tests declared out of scope (unbounded stress workloads, etc.). Match
  # is unconditional on the module-qualified name (`<crate>__<module>..__<fn>`,
  # i.e. the trace basename with the trailing `__h<hash>` removed) regardless of
  # how the checker terminated, so the raw result never enters the pass/fail
  # counts. Only names listed in the unsupported-tests file are affected.
  if [ -n "${unsupported_tests:-}" ] && [ -s "${unsupported_tests:-}" ]; then
    for trace_file in "$trace_dir"/*_verification.txt; do
      [ -f "$trace_file" ] || continue
      local trace_basename qualified_name
      trace_basename=$(basename "$trace_file" _verification.txt)
      qualified_name=${trace_basename%__h*}   # drop trailing __h<hash>
      if grep -v '^[[:space:]]*#' "$unsupported_tests" 2>/dev/null \
          | grep -qxF "$qualified_name" 2>/dev/null; then
        echo "Unsupported test: out-of-scope workload" >> "$trace_file"
      fi
    done
  fi
}

# Select the integration-test bitcode file (the one containing a main symbol).
# Args: search_dir stem output_file
# Returns 0 if found, 1 if not.
select_integration_bitcode() {
  local search_dir="$1" stem="$2" output="$3"
  : > "$output"
  for bc in "${search_dir}/${stem}"-*.bc; do
    [ -f "$bc" ] || continue
    if llvm-nm-21 "$bc" 2>/dev/null | grep -q " T main$"; then
      echo "$bc" > "$output"
      return 0
    fi
  done
  return 1
}

# Link bitcode files with the globals override and disassemble.
# Args: input_list output_bc
link_and_disassemble() {
  local input_list="$1" output_bc="$2"
  llvm-link-21 --internalize --override="${OVERRIDE_LL}" \
    -o "$output_bc" "@${input_list}"
  llvm-dis-21 -o "${output_bc%.bc}.ll" "$output_bc"
}

# Summarize verification results from a trace directory into a summary file.
# Must be called with cwd = the trace directory.
# Args: summary_file
summarize_results() {
  local summary_file="$1"
  local file_count
  file_count=$(find . -maxdepth 1 -type f | wc -l)
  : > "$summary_file"

  # Ordered list of "label|pattern" entries (matches collect_results.py categories)
  local categories=(
    "Verification success|Verification complete. No errors were detected."
    "Expected panic occurred|Expected panic occurred"
    "Unsupported test (out of scope)|Unsupported test:"
    "Unexpected panic|Thread panicked"
    "Uninitialised heap read|Error: Attempt to read from uninitialized memory!"
    "Could not identify test entry point|ERROR: Could not find program's entry point function!"
    "External function errors|ERROR: Tried to execute an unknown external function:"
    "Unsupported AtomicRMW operation on non-integer value|visitAtomicRMWInst"
    "External global errors|LLVM ERROR: Could not resolve external global address:"
    "Unsupported syscall|Calling external var arg function"
    "Timeout|TIMEOUT"
  )

  local categorized_files
  categorized_files=$(mktemp)

  local expected_panic_files_tmp
  expected_panic_files_tmp=$(mktemp)

  local label pattern count
  for entry in "${categories[@]}"; do
    label="${entry%%|*}"
    pattern="${entry#*|}"
    if [ "$label" = "Expected panic occurred" ]; then
      grep -rl "$pattern" . 2>/dev/null | sort -u > "$expected_panic_files_tmp" 2>/dev/null || true
      count=$(cat "$expected_panic_files_tmp" | tee -a "$categorized_files" | wc -l)
    elif [ "$label" = "Unexpected panic" ]; then
      # Exclude files already counted as expected panics
      local all_panic
      all_panic=$(grep -rl "$pattern" . 2>/dev/null | sort -u || true)
      if [ -n "$all_panic" ] && [ -s "$expected_panic_files_tmp" ]; then
        count=$(echo "$all_panic" | comm -23 - "$expected_panic_files_tmp" | tee -a "$categorized_files" | wc -l)
      elif [ -n "$all_panic" ]; then
        count=$(echo "$all_panic" | tee -a "$categorized_files" | wc -l)
      else
        count=0
      fi
    else
      count=$(grep -rl "$pattern" . 2>/dev/null | tee -a "$categorized_files" | wc -l)
    fi
    if [ "$label" = "Unexpected panic" ]; then
      echo "" >> "$summary_file"
      echo "ERRORS:" >> "$summary_file"
    fi
    echo "$label: $count / $file_count" >> "$summary_file"
  done
  rm -f "$expected_panic_files_tmp"

  # Deduplicate categorized files and compute "other"
  sort -u "$categorized_files" | sed 's|^\./||' > "${categorized_files}.sorted"

  local all_files
  all_files=$(mktemp)
  find . -maxdepth 1 -type f | sed 's|^\./||' | sort > "$all_files"

  local other_files
  other_files=$(comm -23 "$all_files" "${categorized_files}.sorted")
  local other_count=0
  if [ -n "$other_files" ]; then
    other_count=$(echo "$other_files" | wc -l)
  fi
  echo "Uncategorised errors: $other_count / $file_count" >> "$summary_file"
  if [ "$other_count" -gt 0 ]; then
    echo "  uncategorized files:" >> "$summary_file"
    echo "$other_files" | while read -r f; do
      echo "    $f" >> "$summary_file"
    done
  fi

  rm -f "$categorized_files" "${categorized_files}.sorted" "$all_files"
}
