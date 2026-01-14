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

COPY bin/onlyswaps-solver/ ./bin/onlyswaps-solver
COPY crates/agent-utils ./crates/agent-utils
COPY crates/config ./crates/config
COPY crates/onlyswaps-client ./crates/onlyswaps-client
COPY crates/omnievent ./crates/omnievent
COPY crates/generated ./crates/generated
COPY crates/utils ./crates/utils
COPY crates/superalloy ./crates/superalloy
COPY modules/dcipher-proto ./modules/dcipher-proto

RUN cargo chef prepare --recipe-path recipe.json --bin onlyswaps-solver
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json --bin onlyswaps-solver

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release -p onlyswaps-solver --bin onlyswaps-solver --all-features

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      openssl \
      curl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/onlyswaps-solver /usr/local/bin
# probably want to use a real config :)
COPY ./bin/onlyswaps-solver/config-default.toml /app/config.toml
ENTRYPOINT ["/usr/local/bin/onlyswaps-solver", "--config", "/app/config.toml"]
