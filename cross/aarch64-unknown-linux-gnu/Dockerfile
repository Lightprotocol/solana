FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge

ENV DEBIAN_FRONTEND=noninteractive

RUN dpkg --add-architecture arm64 \
    && apt-get update && apt-get install --assume-yes --no-install-recommends \
    libssl-dev:arm64 \
    libudev-dev:arm64 \
    pkg-config \
    zlib1g-dev:arm64 \
    llvm \
    clang \
    cmake \
    make \
    libprotobuf-dev:arm64 \
    protobuf-compiler
