# Image must be built from the root

# Base image for rust
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin onlyswaps-verifier

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --bin onlyswaps-verifier

# Build application
COPY Cargo.toml Cargo.lock ./

COPY bin/onlyswaps-verifier/ ./bin/onlyswaps-verifier
COPY crates/agent-utils ./crates/agent-utils
COPY crates/config ./crates/config
COPY crates/generated ./crates/generated
COPY crates/network ./crates/network
COPY crates/omnievent ./crates/omnievent
COPY crates/signer ./crates/signer
COPY crates/superalloy ./crates/superalloy
COPY crates/utils ./crates/utils
COPY modules/dcipher-proto ./modules/dcipher-proto

RUN cargo build --release -p onlyswaps-verifier --bin onlyswaps-verifier

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/onlyswaps-verifier /usr/local/bin
ENTRYPOINT ["/usr/local/bin/onlyswaps-verifier", "start"]
