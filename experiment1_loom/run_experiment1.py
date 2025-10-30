#!/usr/bin/env python3
"""
Automated experiment runner for RustMC tests
Runs verify_single.sh on all tests in csv file and outputs results as CSV
"""

import csv
import subprocess
import re
import json
import os
from pathlib import Path
from datetime import datetime
from collections import defaultdict

class ExperimentRunner:
    def __init__(self, csv_path, ported_tests_dir, verify_script):
        self.csv_path = csv_path
        self.ported_tests_dir = Path(ported_tests_dir)
        self.verify_script = verify_script
        self.results = []

    def parse_csv(self):
        """Parse the CSV and return tests that should be run (not rejected)"""
        tests_to_run = defaultdict(list)

        with open(self.csv_path, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                # Skip if rejected (optional column - if present)
                reject = row.get('Reject?', '').strip()
                if reject and reject.lower().startswith('true'):
                    continue

                rustmc_file = row.get('RustMC test file', '').strip()
                if not rustmc_file or rustmc_file == 'N/A':
                    continue

                # Get unroll bound, default to 1 if not specified
                unroll_bound = row.get('unroll bound', '1').strip()
                if not unroll_bound:
                    unroll_bound = '1'

                test_info = {
                    'file_name': row.get('File Name', '').strip(),
                    'rustmc_file': rustmc_file,
                    'test_name': row.get('Test Name', '').strip(),
                    'should_panic': row.get('Should Panic', '').strip(),
                    'ignored': row.get('Ignored', '').strip(),
                    'unroll_bound': unroll_bound,
                }

                tests_to_run[rustmc_file].append(test_info)

        return tests_to_run

    def run_verification(self, rust_file, unroll_bound='1'):
        """Run verify_single.sh on a Rust file and capture output"""
        rust_file_path = self.ported_tests_dir / rust_file

        if not rust_file_path.exists():
            return None, f"File not found: {rust_file_path}"

        print(f"\n{'='*60}")
        print(f"Running verification on: {rust_file} (unroll bound: {unroll_bound})")
        print(f"{'='*60}")

        try:
            # Run verify_single.sh with unroll bound (timing is captured inside docker)
            result = subprocess.run(
                ['bash', self.verify_script, str(rust_file_path), str(unroll_bound)],
                capture_output=True,
                text=True,
                check=True,
                timeout=1800  # 30 minute timeout per file
            )

            output = result.stdout + result.stderr

            return output, None

        except subprocess.TimeoutExpired:
            return None, "TIMEOUT (1800)"
        except Exception as e:
            return None, f"Error running verification: {str(e)}"

    def parse_timing_file(self, timing_file_path):
        """Parse timing information from a test's timing file"""
        timing_info = {
            'max_resident_set_size_mb': 0,
            'user_time_seconds': 0.0,
            'system_time_seconds': 0.0,
            'total_time_seconds': 0.0,
            'max_resident_set_size_kb': 0
        }

        if not timing_file_path.exists():
            return None

        try:
            with open(timing_file_path, 'r') as f:
                content = f.read()

            # Extract user time - format: "User time (seconds): 0.42"
            user_time_match = re.search(r'User time \(seconds\):\s+(\d+\.\d+)', content)
            if user_time_match:
                timing_info['user_time_seconds'] = float(user_time_match.group(1))

            # Extract system time - format: "System time (seconds): 0.01"
            system_time_match = re.search(r'System time \(seconds\):\s+(\d+\.\d+)', content)
            if system_time_match:
                timing_info['system_time_seconds'] = float(system_time_match.group(1))

            # Extract maximum resident set size - format: "Maximum resident set size (kbytes): 102692"
            max_rss_match = re.search(r'Maximum resident set size \(kbytes\):\s+(\d+)', content)
            if max_rss_match:
                max_rss_kb = int(max_rss_match.group(1))
                timing_info['max_resident_set_size_kb'] = max_rss_kb
                timing_info['max_resident_set_size_mb'] = max_rss_kb / 1024.0

            # Calculate total CPU time
            timing_info['total_time_seconds'] = (
                timing_info['user_time_seconds'] +
                timing_info['system_time_seconds']
            )

            return timing_info

        except Exception as e:
            print(f"Warning: Could not parse timing file {timing_file_path}: {e}")
            return None

    def parse_verification_output(self, output, test_info):
        """Parse verification output to extract results for each test"""
        results = []

        # Get the filename to find trace files
        rust_file = test_info[0]['rustmc_file']
        base_name = rust_file.replace('.rs', '')

        # Look for test traces directory
        traces_dir = self.ported_tests_dir.parent / 'test_traces' / base_name

        # For each test in the file
        for test in test_info:
            test_name = test['test_name']

            result = {
                'file': test['rustmc_file'],
                'test_name': test_name,
                'should_panic': test['should_panic'],
                'unroll_bound': test.get('unroll_bound', '1'),
                'executions_explored': 0,
                'crashed': False,
                'panic_found': False,
                'successful': False,
                'timeout': False,
                'error_messages': [],
                'status': 'UNKNOWN'
            }

            # Try to read the individual trace file
            trace_file = traces_dir / f"{test_name}_verification.txt"

            if not trace_file.exists():
                # Fall back to searching in the main output
                result['error_messages'].append(f"Trace file not found: {trace_file}")
                result['status'] = 'NOT_RUN'
                results.append(result)
                continue

            # Read the trace file
            with open(trace_file, 'r') as f:
                trace_content = f.read()

            # Try to read the timing file for this test
            timing_file = traces_dir / f"{test_name}_timing.txt"
            test_timing = self.parse_timing_file(timing_file)
            result['timing'] = test_timing if test_timing else {}

            # Extract number of executions
            exec_match = re.search(
                r'Number of complete executions explored:\s*(\d+)',
                trace_content,
                re.MULTILINE
            )
            if exec_match:
                result['executions_explored'] = int(exec_match.group(1))

            # Check for timeout
            if 'TIMEOUT' in trace_content:
                result['timeout'] = True
                result['status'] = 'TIMEOUT'
                result['error_messages'].append('Verification timeout')

            # Check for successful verification
            if 'Verification complete. No errors were detected.' in trace_content:
                result['successful'] = True
                result['status'] = 'SUCCESS'

            # Check for panic (rust_begin_unwind)
            if 'Verification unsuccessful.' in trace_content or 'core9panicking5panic' in trace_content or 'rust_begin_unwindGenMC' in trace_content :
                result['panic_found'] = True
                result['error_messages'].append('Verification unsuccessful.')
                if result['status'] == 'UNKNOWN':
                    result['status'] = 'PANIC_FOUND'

            # Check for various error types
            error_patterns = {
                'LLVM ERROR:': 'LLVM error',
                'ERROR: Tried to execute an unknown external function:': 'Unknown external function',
                'Attempting get constant value for:': 'Constant value error',
                'Assertion violation': 'Assertion violation',
                'Safety violation': 'Safety violation',
                'Error: Attempt to read from uninitialized memory': 'Uninitialized memory read',
            }

            for pattern, desc in error_patterns.items():
                if pattern in trace_content:
                    result['crashed'] = True
                    # Extract the error context
                    error_match = re.search(
                        rf'{re.escape(pattern)}[^\n]*',
                        trace_content,
                        re.MULTILINE
                    )
                    if error_match:
                        error_msg = error_match.group(0).strip()
                        result['error_messages'].append(f"{desc}: {error_msg[:200]}")

            # Determine final status if still unknown
            if not result['status'] == 'PANIC_FOUND' and not result['status'] == 'SUCCESS' and not result['status'] == 'TIMEOUT':
                result['status'] = 'CRASH'

            # Compare to expected
            should_panic = test['should_panic'].lower() == 'yes'

            if should_panic and result['panic_found']:
                result['matches_expected'] = True
                result['match_reason'] = 'Should panic and panic found'
            elif should_panic and not result['panic_found'] and result['successful']:
                result['matches_expected'] = False
                result['match_reason'] = 'Should panic but verified successfully'
            elif not should_panic and result['successful']:
                result['matches_expected'] = True
                result['match_reason'] = 'Should not panic and verified successfully'
            elif not should_panic and result['panic_found']:
                result['matches_expected'] = False
                result['match_reason'] = 'Should not panic but panic found'
            else:
                result['matches_expected'] = None
                result['match_reason'] = 'Crashed when should succeed'                
            results.append(result)

        return results

    def run_experiment_on_file(self, rust_file, test_info):
        """Run experiment on a single file"""
        # Get unroll bound from first test (all tests in same file should have same bound)
        unroll_bound = test_info[0].get('unroll_bound', '1') if test_info else '1'

        output, error = self.run_verification(rust_file, unroll_bound)

        if error:
            # Create error results for all tests in this file
            results = []
            for test in test_info:
                results.append({
                    'file': rust_file,
                    'test_name': test['test_name'],
                    'should_panic': test['should_panic'],
                    'unroll_bound': test.get('unroll_bound', '1'),
                    'executions_explored': 0,
                    'crashed': True,
                    'panic_found': False,
                    'successful': False,
                    'error_messages': [error],
                    'status': 'CRASH',
                    'matches_expected': False,
                    'timing': {}
                })
            return results

        # Parse the output (timing is parsed per-test inside this method)
        results = self.parse_verification_output(output, test_info)

        # Add raw output for reference
        for result in results:
            result['raw_output'] = output

        return results

    def run_all(self, file_filter=None):
        """Run experiment on all files or specific file"""
        tests_to_run = self.parse_csv()

        if file_filter:
            tests_to_run = {k: v for k, v in tests_to_run.items() if k == file_filter}

        print(f"\nFound {len(tests_to_run)} files to test")
        print(f"Files: {', '.join(tests_to_run.keys())}\n")

        all_results = []

        for rust_file, test_info in tests_to_run.items():        
            results = self.run_experiment_on_file(rust_file, test_info)
            all_results.extend(results)

        self.results = all_results
        return all_results

    def generate_report(self, output_file='experiment_results.csv'):
        """Generate CSV report"""
        # Generate CSV output only
        self.generate_csv(output_file)
        print(f"\nResults saved to: {output_file}")
        return output_file

    def generate_csv(self, output_file='experiment_results.csv'):
        """Generate CSV file with one line per test"""
        import csv

        with open(output_file, 'w', newline='') as f:
            # Define CSV columns
            fieldnames = [
                'file',
                'test_name',
                'should_panic',
                'unroll_bound',
                'status',
                'successful',
                'panic_found',
                'crashed',
                'timeout',
                'executions_explored',
                'matches_expected',
                'max_memory_mb',
                'cpu_user_seconds',
                'cpu_system_seconds',
                'cpu_total_seconds'
            ]

            writer = csv.DictWriter(f, fieldnames=fieldnames)
            writer.writeheader()

            for result in self.results:
                # Get timing info (now per-test)
                timing = result.get('timing', {})

                row = {
                    'file': result.get('file', ''),
                    'test_name': result.get('test_name', ''),
                    'should_panic': result.get('should_panic', ''),
                    'unroll_bound': result.get('unroll_bound', '1'),
                    'status': result.get('status', ''),
                    'panic_found': result.get('panic_found', False),
                    'timeout': result.get('timeout', False),
                    'executions_explored': result.get('executions_explored', 0),
                    'matches_expected': result.get('matches_expected', ''),
                    'max_memory_mb': f"{timing.get('max_resident_set_size_mb', 0):.2f}",
                    'cpu_user_seconds': f"{timing.get('user_time_seconds', 0):.3f}",
                    'cpu_system_seconds': f"{timing.get('system_time_seconds', 0):.3f}",
                    'cpu_total_seconds': f"{timing.get('total_time_seconds', 0):.3f}"
                }

                writer.writerow(row)


def main():
    import argparse

    parser = argparse.ArgumentParser(description='Run RustMC verification experiments')
    parser.add_argument('--csv', default='test_inventory_artifact.csv',
                        help='Path to test inventory CSV')
    parser.add_argument('--ported-tests', default='loom-tests-ported',
                        help='Directory containing ported test files')
    parser.add_argument('--verify-script', default='verify_single.sh',
                        help='Path to verify_single.sh script')
    parser.add_argument('--file', help='Run only on specific file (e.g., atomic_relaxed_ported_genmc.rs)')
    parser.add_argument('--output', default='experiment_results.csv',
                        help='Output file for results')

    args = parser.parse_args()

    runner = ExperimentRunner(
        csv_path=args.csv,
        ported_tests_dir=args.ported_tests,
        verify_script=args.verify_script
    )

    results = runner.run_all(file_filter=args.file)
    runner.generate_report(output_file=args.output)

    # Print summary statistics
    print("\n" + "="*80)
    print("SUMMARY")
    print("="*80)
    print(f"Total tests: {len(results)}")
    print(f"Matches expected: {sum(1 for r in results if r.get('matches_expected'))}")
    print(f"Panics found: {sum(1 for r in results if r['panic_found'])}")


if __name__ == '__main__':
    main()
