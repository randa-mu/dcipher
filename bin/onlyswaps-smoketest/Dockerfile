FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin onlyswaps-smoketest

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --bin onlyswaps-smoketest

# Build application
COPY Cargo.toml Cargo.lock ./

COPY bin/onlyswaps-smoketest/ ./bin/onlyswaps-smoketest
COPY crates/agent-utils ./crates/agent-utils
COPY crates/config ./crates/config
COPY crates/generated ./crates/generated
COPY crates/onlyswaps-client ./crates/onlyswaps-client
COPY crates/superalloy ./crates/superalloy
COPY crates/utils ./crates/utils

RUN cargo build --release -p onlyswaps-smoketest

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/onlyswaps-smoketest /usr/local/bin
ENTRYPOINT ["/usr/local/bin/onlyswaps-smoketest"]