#!/bin/bash

# Given two commits A and B, checks whether B introduced any
# noticeable regressions on A
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
    echo "Usage: $0 OLD_COMMIT NEW_COMMIT" >&2
}

filter_output() {
    in=$1
    out=$2
    cat "${in}" | awk -v t="${THRESHOLD}" ' \
    		{ if($0 ~ /Preparing to run/) {model=$9; co=$11;} } \
        	$(NF?NF-1:0) ~ /^[[:digit:]]+?\.[[:digit:]]+/ \
			     { if ($(NF-1) > t) print $2 "-" model "/" co " " $(NF-1) } \
		' \
    > "${out}"
}

if [[ $# -ne 2 ]]
then
    display_help
    exit 1
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
source "${DIR}/terminal.sh"

OLD_COMMIT=$1
NEW_COMMIT=$2

THRESHOLD="${THRESHOLD:-0.10}"

out_dir="/tmp"
res_prefix="__check_regression_$RANDOM"
out_old="${out_dir}/${res_prefix}_${OLD_COMMIT}.out"
out_new="${out_dir}/${res_prefix}_${NEW_COMMIT}.out"

# run tests w/ OLD and NEW
make clean
git checkout "${OLD_COMMIT}"
make -j `nproc`
TERM=xterm-mono GENMCFLAGS="-disable-estimation" "${DIR}"/fast-driver.sh > "${out_old}"

make clean
git checkout "${NEW_COMMIT}"
make -j `nproc`
TERM=xterm-mono GENMCFLAGS="-disable-estimation" "${DIR}"/fast-driver.sh > "${out_new}"

# keep only name/model/co + time for tests
out_old_filtered="${out_dir}/${res_prefix}_${OLD_COMMIT}_filtered.out"
out_new_filtered="${out_dir}/${res_prefix}_${NEW_COMMIT}_filtered.out"

filter_output "${out_old}" "${out_old_filtered}"
filter_output "${out_new}" "${out_new_filtered}"

out_joined="${out_dir}/${res_prefix}_joined.out"

# find common tests between commits and print overhead
LANG=en_EN join <(LANG=en_EN sort "${out_old_filtered}") \
    <(LANG=en_EN sort "${out_new_filtered}") | \
    awk '{ print $0 " " $3/$2 }' | \
    sort -n -k4 | \
    column -t > "${out_joined}"

# print most important differences
echo '--- Major regressions:'
tail -n3 "${out_joined}"

pivot=$(head -n3 "${out_joined}" | tail -n1 | awk '{ print $2 }')
if [[ $(echo "${pivot} < 1.0" | bc -l) -eq 1 ]]
then
    echo ''
    echo '--- Major speedups:'
    head -n3 "${out_joined}"
fi

# print total times
echo ''
echo -ne '--- Total time (old): '
cat "${out_old}" | awk '/Total time:/ { print $NF }'
echo -ne '--- Total time (new): '
cat "${out_new}" | awk '/Total time:/ { print $NF }'
