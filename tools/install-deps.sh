#!/usr/bin/env bash
#
# install-deps.sh -- install RustMC's build prerequisites.
#
# Debian/Ubuntu only (apt). On other distros, install the equivalents of the
# packages listed below plus LLVM/Clang 18, then build with the instructions in
# README.md.
#
# This is the same package set the Docker images build from, so it is known to
# be sufficient to `./configure && make && make install`.
#
#   ./tools/install-deps.sh
#
# Uses sudo automatically when not run as root.
set -euo pipefail

PACKAGES=(
  # build tooling (autotools + a C++20 compiler)
  autoconf automake libtool make pkg-config g++
  # libraries RustMC links against
  libffi-dev zlib1g-dev libedit-dev libxml2-dev
  # LLVM / Clang 21 and the llvm-{link,dis,nm}-21 tools
  clang-21 llvm-21 llvm-21-dev llvm-21-tools
  # misc used by the build/front-end
  git curl xz-utils util-linux ca-certificates
)

RUST_TOOLCHAIN="nightly-2025-08-20"

if ! command -v apt-get >/dev/null 2>&1; then
  echo "This script supports Debian/Ubuntu (apt) only." >&2
  echo "Install the equivalents of these packages on your distro:" >&2
  printf '  %s\n' "${PACKAGES[@]}" >&2
  exit 1
fi

SUDO=""
if [ "$(id -u)" -ne 0 ]; then
  SUDO="sudo"
fi

echo ">> Installing system packages via apt..."
$SUDO apt-get update
$SUDO apt-get install -y --no-install-recommends "${PACKAGES[@]}"

echo
echo ">> System prerequisites installed."

# --- Rust toolchain -------------------------------------------------------
if command -v rustup >/dev/null 2>&1; then
  echo ">> Installing Rust toolchain $RUST_TOOLCHAIN via rustup..."
  rustup toolchain install "$RUST_TOOLCHAIN"
else
  echo ">> rustup not found. Install it, then run:"
  echo "     rustup toolchain install $RUST_TOOLCHAIN"
  echo "   (get rustup from https://rustup.rs)"
fi

echo
echo ">> Done. Build RustMC with:"
echo "     autoreconf --install"
echo "     ./configure --with-llvm=/usr/lib/llvm-21"
echo "     make -j\$(nproc) && sudo make install"
