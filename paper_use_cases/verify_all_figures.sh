#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

for fig in figure1 figure2 figure8 figure9 figure10; do
    "$SCRIPT_DIR/verify_figure.sh" "$fig"
done
