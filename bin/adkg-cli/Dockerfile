# This ARG is overridden by Bake contexts to 'target:rust-base-internal'
# If built via 'docker build', it defaults to the slow but working chef image.
ARG base_stage_alias=lukemathwalker/cargo-chef:latest-rust-1.92
FROM ${base_stage_alias} AS builder
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y protobuf-compiler

RUN cargo install cargo-chef sccache --locked

ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/sccache

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/adkg ./crates/adkg
COPY crates/config ./crates/config
COPY crates/network ./crates/network
COPY crates/utils ./crates/utils
COPY crates/omnievent ./crates/omnievent
COPY crates/superalloy ./crates/superalloy
COPY modules/dcipher-proto ./modules/dcipher-proto
COPY bin/adkg-cli ./bin/adkg-cli

RUN cargo chef prepare --recipe-path internal.json --bin adkg-cli
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path internal.json --bin adkg-cli

# Build application
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release -p adkg-cli --bin adkg-cli

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/adkg-cli /usr/local/bin/adkg-cli
CMD ["adkg-cli"]
