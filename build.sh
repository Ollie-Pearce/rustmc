#!/bin/bash

REPO="Ollie-Pearce/rustmc"
TAG="v0.2"
ASSET_NAME="rust_toolchain.tar.xz"
RELEASE_JSON=$(curl -s "https://api.github.com/repos/$REPO/releases/tags/$TAG")
ASSET_API_URL=$(echo "$RELEASE_JSON" | jq -r ".assets[]? | select(.name==\"$ASSET_NAME\") | .url")

[ -z "$ASSET_API_URL" ] && { echo "ERROR: Rust Toolchain not found. Exiting"; exit 1; }

#Download, extract and link custom toolchain
curl -L -H "Accept: application/octet-stream" "$ASSET_API_URL" -o "$ASSET_NAME"
tar -xf "$ASSET_NAME" -C rust_toolchain
rustup toolchain link RustMC $PWD/rust_toolchain/stage1


#Build GenMC/RustMC
cd genmc
autoreconf --install
./configure
make
