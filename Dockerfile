# syntax=docker/dockerfile:1.7
FROM ubuntu:24.04

ARG REPO="Ollie-Pearce/private_mixer"
ARG TAG="v0.7.0" 
ARG ASSET_NAME="rust_toolchain.tar.xz"
ARG GITHUB_TOKEN  

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:/opt/rustup/toolchains/RustMC/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

WORKDIR /root

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl jq xz-utils git \
    autoconf automake make libtool pkg-config \
    libffi-dev zlib1g-dev libedit-dev libxml2-dev \
    g++ clang util-linux \
    clang-18 llvm-18 llvm-18-dev \
    bash gawk vim \
 && rm -rf /var/lib/apt/lists/*

# rustup (no default toolchain)
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain none

# clone private repo using the token passed as argument
RUN bash -c 'set -euo pipefail; \
    git clone "https://x-access-token:${GITHUB_TOKEN}@github.com/${REPO}.git"'

RUN mkdir -p /root/custom_toolchain
RUN echo "Current working directory: ${PWD}" && ls -l
COPY rust_toolchain.tar.xz /root/
RUN tar -xf /root/rust_toolchain.tar.xz -C /root/custom_toolchain
RUN rustup toolchain link RustMC /root/custom_toolchain/stage1

RUN rustup default stable
RUN rustup component add cargo

WORKDIR /root/private_mixer
RUN autoreconf --install && ./configure --with-llvm=/usr/lib/llvm-18 && make -j"$(nproc)" && make install