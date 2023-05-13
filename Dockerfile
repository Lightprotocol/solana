FROM rust:1-bullseye AS builder

# Workaround for OOM issue in libgit2:
# https://github.com/docker/build-push-action/issues/621
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN apt update \
    && apt install -y \
    libssl-dev \
    libudev-dev \
    pkg-config \
    zlib1g-dev \
    llvm \
    clang \
    cmake \
    make \
    libprotobuf-dev \
    protobuf-compiler
COPY . /usr/local/src/solana
WORKDIR /usr/local/src/solana
RUN ./scripts/cargo-install-all.sh /usr/local/solana

FROM debian:bullseye-slim

COPY --from=builder /usr/local/solana /usr/local/solana
COPY entrypoint.sh /usr/local/bin

ENV PATH="$PATH:/usr/local/solana/bin"

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
