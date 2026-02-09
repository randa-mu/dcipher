# This ARG is overridden by Bake contexts to 'target:rust-base-internal'
# If built via 'docker build', it defaults to the slow but working chef image.
ARG base_stage_alias=lukemathwalker/cargo-chef:latest-rust-1.92
FROM ${base_stage_alias} AS builder
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

RUN cargo install cargo-chef sccache --locked

ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/sccache

# Build application
COPY Cargo.toml Cargo.lock ./

COPY bin/randomness-agent/ ./bin/randomness-agent
COPY crates/config ./crates/config
COPY crates/dcipher-agents ./crates/dcipher-agents
COPY crates/generated ./crates/generated
COPY crates/network ./crates/network
COPY crates/signer ./crates/signer
COPY crates/superalloy ./crates/superalloy
COPY crates/utils ./crates/utils
COPY crates/omnievent ./crates/omnievent
COPY modules/dcipher-proto ./modules/dcipher-proto

RUN cargo chef prepare --recipe-path recipe.json --bin randomness-agent
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release -p randomness-agent --example randomness

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/examples/randomness /usr/local/bin
ENTRYPOINT ["/usr/local/bin/randomness"]
