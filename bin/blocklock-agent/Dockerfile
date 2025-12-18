FROM lukemathwalker/cargo-chef:latest-rust-1.92 AS chef
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM chef AS planner
WORKDIR /app

COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin blocklock-agent

FROM chef AS builder
WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json -p blocklock-agent --examples

# Copy minimum dependencies to benefit from caching
COPY Cargo.toml Cargo.lock ./
COPY bin/blocklock-agent/ ./bin/blocklock-agent
COPY crates/dcipher-agents ./crates/dcipher-agents
COPY crates/generated ./crates/generated
COPY crates/utils ./crates/utils
COPY crates/config ./crates/config
COPY crates/network ./crates/network
COPY crates/signer ./crates/signer
COPY crates/superalloy ./crates/superalloy
COPY crates/omnievent ./crates/omnievent
COPY modules/dcipher-proto ./modules/dcipher-proto

# Build application
RUN cargo build --release -p blocklock-agent --example blocklock

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/examples/blocklock /usr/local/bin
ENTRYPOINT ["/usr/local/bin/blocklock"]
