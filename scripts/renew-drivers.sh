#!/bin/bash

# Driver script for running tests with GenMC.
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

# Get binary's full path
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
KATER="${KATER:-$DIR/../kater-tool/Release/kater}"

# Ensure kater exists
if ! command -v "${KATER}" -h &> /dev/null
then
    echo "kater was not found"
    exit 1
fi

# used variables
IN_DIR="${IN_DIR:-$DIR/../kater-tool/kat}"
OUT_DIR="${OUT_DIR:-$DIR/../src}"
DRIVERS="${DRIVERS:-rc11 imm sc tso ra}"

for driver in ${DRIVERS} # break lines
do
    $("${KATER}" -e -p"${OUT_DIR}" -n"${driver}" "${IN_DIR}/${driver}-genmc.kat")
    echo "exporting " $driver $?
done
