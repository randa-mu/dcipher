# Image must be built from the root

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin monitoring

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --bin monitoring

# Build application
COPY Cargo.toml Cargo.lock ./

COPY bin/monitoring/ ./bin/monitoring
COPY crates/agent-utils ./crates/agent-utils
COPY crates/config ./crates/config
COPY crates/generated ./crates/generated
COPY crates/utils ./crates/utils

RUN cargo build --release -p monitoring --bin monitoring

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/monitoring /usr/local/bin
ENTRYPOINT ["/usr/local/bin/monitoring"]
