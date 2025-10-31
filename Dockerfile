# syntax=docker/dockerfile:1.7
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:/opt/rustup/toolchains/RustMC/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

WORKDIR /root

RUN apt-get update && apt-get install -y --no-install-recommends \
    wget gnupg ca-certificates lsb-release \
    software-properties-common \
    ca-certificates curl jq xz-utils git \
    autoconf automake make libtool pkg-config \
    libffi-dev zlib1g-dev libedit-dev libxml2-dev \
    g++ clang util-linux \
    clang-18 llvm-18 llvm-18-dev \
    llvm-18-tools ripgrep \
    bash gawk vim openssh-client \
    time miller \
 && rm -rf /var/lib/apt/lists/*


RUN dpkg -l | grep llvm-18

RUN command -v llvm-ar-18 && \
    command -v llvm-readelf-18 && \
    command -v llvm-objcopy-18 && \
    command -v llvm-dis-18 && \
    command -v llvm-nm-18 && \
    command -v opt-18

# rustup (no default toolchain)
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain none

# clone private repo using the token passed as argument
RUN git clone https://github.com/Ollie-Pearce/rustmc.git



RUN mkdir -p /root/compressed_toolchain
RUN echo "Current working directory: ${PWD} at $(date)" && ls -l

RUN curl -L -H "Accept: application/octet-stream" "https://github.com/Ollie-Pearce/rust/releases/download/v1.2/rust_toolchain.tar.xz" -o "rust_toolchain.tar.xz"

RUN echo "=== File size and type at $(date)===" && ls -lh rust_toolchain.tar.xz && file rust_toolchain.tar.xz

RUN tar -xf rust_toolchain.tar.xz -C /root/compressed_toolchain
RUN rustup toolchain link RustMC /root/compressed_toolchain/stage1
RUN rm -rf rust_toolchain.tar.xz

RUN rustup default stable
RUN rustup component add cargo

RUN mkdir -p /root/full_toolchain
WORKDIR /root/full_toolchain
RUN git clone https://github.com/Ollie-Pearce/rust.git

WORKDIR /root/rustmc

RUN autoreconf --install && ./configure --with-llvm=/usr/lib/llvm-18 && make -j"$(nproc)" && make install


CMD ["bash"]

