#!/bin/bash

# Runs a bunch of tests multiple times with concurrency enabled
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, you can access it online at
# http://www.gnu.org/licenses/gpl-2.0.html.
#
# Author: Michalis Kokologiannakis <mixaskok@gmail.com>

# display help function
display_help() {
    echo "Usage: $0" >&2
}

if [[ $# -ne 0 ]]
then
    display_help
    exit 1
fi

# set -e

NR_CPUS=$(nproc) # max number of cores
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
TEST_DIR="${DIR}/../tests"
GENMC="${GENMC:-${DIR}/../genmc}"
RUNS="${RUNS:-3}"

source "${DIR}/terminal.sh"

# benchmark commands
# write those down explicitly so that we print nicer names (+ plotting, if desired)
declare -A correct; declare -a corder

correct["lastzero(19)"]=" -- -DN=19 ${TEST_DIR}/correct/synthetic/lastzero/variants/lastzero0.c"; corder+=("lastzero(19)")
correct["indexer(17)"]=" -- -DN=17 ${TEST_DIR}/correct/synthetic/indexer/variants/indexer0.c"; corder+=("indexer(17)")
correct["mcs-spinlock(4)"]=" -unroll=4 -disable-sr -disable-ipr -- -DN=4 -I${TEST_DIR}/correct/data-structures/mcs_spinlock ${TEST_DIR}/correct/data-structures/mcs_spinlock/variants/main0.c"; corder+=("mcs-spinlock(4)")

declare -A wrong; declare -a worder

wrong["lamport-deadlock"]="-check-liveness -unroll=5 -- ${TEST_DIR}/wrong/liveness/lamport-deadlock/variants/lamport-deadlock0.c"; worder+=("lamport-deadlock")
wrong["lock-deadlock"]="-check-liveness -- ${TEST_DIR}/wrong/liveness/lock-deadlock/variants/lock-deadlock0.c"; worder+=("lock-deadlock")
wrong["fib_bench"]=" -- ${TEST_DIR}/wrong/safety/fib_bench/variants/fib_bench0.c"; worder+=("fib_bench")


run_correct() {
    name=$1
    threads=$2
    total_time=0
    expected=""
    for i in `seq 1 ${RUNS}`
    do
	result=$("${GENMC}" -nthreads="${threads}" ${correct[$name]} 2>&1)
	if [[ "$?" -ne 0 ]]
	then
	    echo "XXX"
	    return 1
	fi
	time=$(echo "${result}" | awk '/Total wall-clock time/ { print substr($4, 1, length($4)-1) }')
	execs=$(echo "${result}" | awk '/complete executions/ { print $6 }')
	expected="${expected:-${execs}}"
	if test "${expected}" != "${execs}"
	then
	    echo "DIFF: ${expected} vs ${execs}"
	    echo "OUTPUT:"
	    echo "${result}"
	    return 2
	fi
	total_time=$(echo "${total_time}+${time}" | bc -l)
    done
    avg_time=$(echo "scale=2; ${total_time} / ${RUNS}" | bc -l)
    echo "${avg_time}"
    return 0
}

run_wrong() {
    name=$1
    threads=$2
    for i in `seq 1 ${RUNS}`
    do
	result=$("${GENMC}" -nthreads="${threads}" ${wrong[$name]} 2>&1)
	if [[ "$?" -ne 42 ]]
	then
	    echo "XXX"
	    return 1
	fi
    done
    echo "OK"
    return 0
}

status=0

### run correct
echo '--- Running correct'

# print header
echo -ne "N "
for name in "${corder[@]}"
do
    echo -ne "$name "
done
echo ''

# run all correct benchmarks + print results
i="${NR_CPUS}"
while [ $i -le $NR_CPUS ]
do
    echo -ne "$i "
    for name in "${corder[@]}"
    do
	result=$(run_correct "${name}" "${i}")
	if [[ $? -ne 0 ]]
	then
	    status=1
	fi
	echo -ne "$result "
    done
    echo ''
    i=$(( $i * 2 ))
done

echo ''

### run wrong
echo '--- Running wrong'

# print header
echo -ne "N "
for name in "${worder[@]}"
do
    echo -ne "$name "
done
echo ''

# run all correct benchmarks + print results
i="${NR_CPUS}"
while [ $i -le $NR_CPUS ]
do
    echo -ne "$i "
    for name in "${worder[@]}"
    do
	result=$(run_wrong "${name}" "${i}")
	if [[ $? -ne 0 ]]
	then
	    status=1
	fi
	echo -ne "$result "
    done
    echo ''
    i=$(( $i * 2 ))
done

if [[ "${status}" -eq 0 ]]
then
    echo '--- Testing proceeded as expected'
else
    echo '--- UNEXPECTED TESTING RESULTS'
fi
exit "${status}"
