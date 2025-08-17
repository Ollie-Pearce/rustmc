FROM debian:trixie
LABEL maintainer="michalis@mpi-sws.org"

WORKDIR /root

RUN export TERM='xterm-256color'

# fetch all necessary packages
RUN apt-get update
RUN apt-get install -y -qq wget gnupg gnupg2 \
    autoconf make automake libffi-dev zlib1g-dev libedit-dev \
    libxml2-dev xz-utils g++ clang git util-linux
RUN apt-get install -y -qq clang-15 llvm-15 llvm-15-dev

# clone and build
RUN git clone -v https://github.com/MPI-SWS/genmc.git
RUN cd genmc && autoreconf --install && ./configure --with-llvm=/usr/lib/llvm-15 && make && make install
