# syntax=docker/dockerfile:1.7
FROM ubuntu:24.04

ARG REPO="Ollie-Pearce/private_mixer"
ARG TAG="v0.7.0" 

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
    bash gawk vim openssh-client time \
 && rm -rf /var/lib/apt/lists/*

# rustup (no default toolchain)
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain none

RUN mkdir -p /root/.ssh && \
    chmod 700 /root/.ssh && \
    ssh-keyscan github.com >> /root/.ssh/known_hosts

RUN eval "$(ssh-agent -s)"

# clone private repo using the token passed as argument
RUN --mount=type=secret,id=ssh_key,target=/root/.ssh/id_rsa,mode=0600 \
    GIT_SSH_COMMAND="ssh -i /root/.ssh/id_rsa -o StrictHostKeyChecking=no" \
    git clone git@github.com:Ollie-Pearce/private_mixer.git
RUN mkdir -p /root/custom_toolchain
RUN echo "Current working directory: ${PWD} at $(date)" && ls -l

RUN curl -L -H "Accept: application/octet-stream" "https://github.com/Ollie-Pearce/rust/releases/download/v1.2/rust_toolchain.tar.xz" -o "rust_toolchain.tar.xz"

RUN echo "=== File size and type at $(date)===" && ls -lh rust_toolchain.tar.xz && file rust_toolchain.tar.xz

RUN echo "Current working directory 2: ${PWD} at $(date)" && ls -l

RUN tar -xf rust_toolchain.tar.xz -C /root/custom_toolchain
RUN rustup toolchain link RustMC /root/custom_toolchain/stage1

RUN rustup default stable
RUN rustup component add cargo

WORKDIR /root/private_mixer
RUN autoreconf --install && ./configure --with-llvm=/usr/lib/llvm-18 && make -j"$(nproc)" && make install