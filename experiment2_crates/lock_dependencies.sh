#!/bin/bash
# lock_dependencies.sh - Use Cargo's MSRV-aware dependency
# resolver to generate a Cargo.lock with dependency versions compatible with
# the crate's minimum supported Rust version (rust-version field).
#
# Usage: ./lock_dependencies.sh [--dry-run] <crate_directory>
#
# Requires Rust 1.84.0+ (which stabilised the MSRV-aware resolver).
#
# How it works:
#   1. Reads rust-version from the crate's Cargo.toml (falls back to 1.81 if unset).
#   2. Creates a temporary .cargo/config.toml that enables the MSRV-aware
#      resolver (incompatible-rust-versions = "fallback").
#   3. Runs "cargo generate-lockfile" so Cargo picks dependency versions
#      compatible with the declared MSRV.
#   4. Cleans up the temporary config (preserving any pre-existing config).
#
# Unlike lock_dependencies.sh (which rewrites Cargo.toml version constraints),
# this script leaves Cargo.toml untouched and relies entirely on the resolver
# to produce an MSRV-compatible Cargo.lock.

set -euo pipefail

DRY_RUN=0
CRATE_DIR=""

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=1 ;;
        -h|--help)
            echo "Usage: $0 [--dry-run] <crate_directory>"
            echo ""
            echo "Generate a Cargo.lock using Cargo's MSRV-aware dependency resolver."
            echo "Uses the crate's rust-version if set, otherwise defaults to 1.81."
            echo ""
            echo "Options:"
            echo "  --dry-run    Show what would happen without making changes"
            echo "  -h, --help   Show this help message"
            echo ""
            echo "Requires Rust >= 1.84.0 for the MSRV-aware resolver."
            exit 0
            ;;
        *) CRATE_DIR="$arg" ;;
    esac
done

if [[ -z "$CRATE_DIR" ]]; then
    echo "Usage: $0 [--dry-run] <crate_directory>" >&2
    exit 1
fi

if [[ ! -f "$CRATE_DIR/Cargo.toml" ]]; then
    echo "Error: $CRATE_DIR/Cargo.toml not found" >&2
    exit 1
fi

# --- Check Rust toolchain version ---------------------------------------------------

RUSTC_VER=$(rustc --version 2>/dev/null | grep -oP '\d+\.\d+\.\d+' || true)
if [[ -z "$RUSTC_VER" ]]; then
    echo "Error: rustc not found. Please install Rust >= 1.84.0." >&2
    exit 1
fi

RUSTC_MINOR=$(echo "$RUSTC_VER" | cut -d. -f2)
if [[ "$RUSTC_MINOR" -lt 84 ]]; then
    echo "Error: rustc $RUSTC_VER is too old. The MSRV-aware resolver requires Rust >= 1.84.0." >&2
    exit 1
fi

# --- Determine the MSRV -------------------------------------------------------------

DEFAULT_MSRV="1.81"
MSRV_SET_IN_TOML=0

# Extract rust-version from Cargo.toml
MSRV=$(python3 -c "
import re, sys
with open(sys.argv[1]) as f:
    text = f.read()
m = re.search(r'^rust-version\s*=\s*\"([^\"]+)\"', text, re.MULTILINE)
if m:
    print(m.group(1))
" "$CRATE_DIR/Cargo.toml")

if [[ -n "$MSRV" ]]; then
    MSRV_SET_IN_TOML=1
    echo "Detected rust-version from Cargo.toml: $MSRV"
else
    MSRV="$DEFAULT_MSRV"
    echo "No rust-version in Cargo.toml, using default: $MSRV"
fi

# --- Set up temporary .cargo/config.toml --------------------------------------------

CARGO_CONFIG_DIR="$CRATE_DIR/.cargo"
CARGO_CONFIG_FILE="$CARGO_CONFIG_DIR/config.toml"
CARGO_TOML="$CRATE_DIR/Cargo.toml"
CREATED_DIR=0
CREATED_FILE=0
BACKED_UP=0
BACKUP_FILE=""
ADDED_RUST_VERSION=0

cleanup() {
    # Restore .cargo/config.toml
    if [[ "$BACKED_UP" -eq 1 && -n "$BACKUP_FILE" ]]; then
        mv "$BACKUP_FILE" "$CARGO_CONFIG_FILE"
    elif [[ "$CREATED_FILE" -eq 1 ]]; then
        rm -f "$CARGO_CONFIG_FILE"
    fi
    if [[ "$CREATED_DIR" -eq 1 ]]; then
        rmdir "$CARGO_CONFIG_DIR" 2>/dev/null || true
    fi
    # Remove temporarily added rust-version from Cargo.toml
    if [[ "$ADDED_RUST_VERSION" -eq 1 ]]; then
        sed -i '/^rust-version = "'"$MSRV"'"$/d' "$CARGO_TOML"
    fi
}
trap cleanup EXIT

if [[ "$DRY_RUN" -eq 1 ]]; then
    echo ""
    if [[ "$MSRV_SET_IN_TOML" -eq 0 ]]; then
        echo "Would temporarily add rust-version = \"$MSRV\" to Cargo.toml"
    fi
    echo "Would configure resolver in $CARGO_CONFIG_FILE:"
    echo '  [resolver]'
    echo '  incompatible-rust-versions = "fallback"'
    echo ""
    echo "Would run: cargo generate-lockfile (in $CRATE_DIR)"
    echo ""
    echo "Cargo would resolve dependencies compatible with rust-version $MSRV."
    echo "(dry run - no changes made)"
    exit 0
fi

# Create .cargo directory if needed
if [[ ! -d "$CARGO_CONFIG_DIR" ]]; then
    mkdir -p "$CARGO_CONFIG_DIR"
    CREATED_DIR=1
fi

# Back up existing config or note that we created one
if [[ -f "$CARGO_CONFIG_FILE" ]]; then
    BACKUP_FILE="${CARGO_CONFIG_FILE}.bak.$$"
    cp "$CARGO_CONFIG_FILE" "$BACKUP_FILE"
    BACKED_UP=1
    # Append resolver config if not already present
    if ! grep -q 'incompatible-rust-versions' "$CARGO_CONFIG_FILE"; then
        printf '\n[resolver]\nincompatible-rust-versions = "fallback"\n' >> "$CARGO_CONFIG_FILE"
    fi
else
    CREATED_FILE=1
    cat > "$CARGO_CONFIG_FILE" << 'EOF'
[resolver]
incompatible-rust-versions = "fallback"
EOF
fi

echo "Configured MSRV-aware resolver in $CARGO_CONFIG_FILE"

# --- Temporarily set rust-version in Cargo.toml if missing ---------------------------

if [[ "$MSRV_SET_IN_TOML" -eq 0 ]]; then
    # Insert rust-version after the [package] header so the resolver can use it
    sed -i '/^\[package\]$/a rust-version = "'"$MSRV"'"' "$CARGO_TOML"
    ADDED_RUST_VERSION=1
    echo "Temporarily added rust-version = \"$MSRV\" to Cargo.toml"
fi

# --- Generate the lockfile -----------------------------------------------------------

echo "Running cargo generate-lockfile for MSRV $MSRV ..."
(cd "$CRATE_DIR" && cargo generate-lockfile 2>&1)

echo ""
echo "Cargo.lock generated with MSRV-compatible dependency versions."
echo "Cargo.toml was not modified."
