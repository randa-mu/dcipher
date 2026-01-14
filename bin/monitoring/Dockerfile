# This ARG is overridden by Bake contexts to 'rust-base-internal'
# If built via 'docker build', it defaults to the slow but working chef image.
ARG base_stage_alias=lukemathwalker/cargo-chef:latest-rust-1.92
FROM ${base_stage_alias} AS builder
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef sccache --locked

ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/sccache

# External crates are already cooked in rust-base-internal, we just need to copy source and build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/agent-utils ./crates/agent-utils
COPY crates/config ./crates/config
COPY crates/generated ./crates/generated
COPY crates/utils ./crates/utils
COPY crates/omnievent ./crates/omnievent
COPY crates/superalloy ./crates/superalloy
COPY modules/dcipher-proto ./modules/dcipher-proto
COPY bin/monitoring/ ./bin/monitoring

RUN cargo chef prepare --recipe-path internal.json --bin monitoring
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path internal.json --bin monitoring

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release -p monitoring --bin monitoring

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/monitoring /usr/local/bin
ENTRYPOINT ["/usr/local/bin/monitoring"]