# syntax=docker/dockerfile:1.7
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:/opt/rustup/toolchains/RustMC/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

WORKDIR /root

RUN apt-get update && apt-get install -y --no-install-recommends \
    miller time \
    wget gnupg ca-certificates lsb-release \
    software-properties-common \
    ca-certificates curl jq xz-utils git \
    autoconf automake make libtool pkg-config \
    libffi-dev zlib1g-dev libedit-dev libxml2-dev \
    g++ clang util-linux \
    clang-18 llvm-18 llvm-18-dev \
    llvm-18-tools ripgrep \
    bash gawk vim openssh-client \
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

RUN rustup default stable
RUN rustup component add cargo
RUN rustup install nightly-2024-06-11-x86_64-unknown-linux-gnu


ARG CACHEBUST=1
# clone private repo using the token passed as argument
RUN git clone https://github.com/Ollie-Pearce/rustmc.git

WORKDIR /root/rustmc

RUN autoreconf --install && ./configure --with-llvm=/usr/lib/llvm-18 && make -j"$(nproc)" && make install


CMD ["bash"]


