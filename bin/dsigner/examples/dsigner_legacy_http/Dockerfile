# Image must be built from the root

# Base image for rust
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app

FROM chef AS planner
COPY . .
# chef prepare --bin sadly doesn't support binaries of kind "example"
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
# --bin sadly doesn't support binaries of kind "example"
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY Cargo.toml Cargo.lock ./

COPY bin/dsigner/ ./bin/dsigner
COPY crates/config ./crates/config
COPY crates/network ./crates/network
COPY crates/signer ./crates/signer
COPY crates/superalloy ./crates/superalloy
COPY crates/utils ./crates/utils
COPY modules/dcipher-proto ./modules/dcipher-proto

RUN cargo build --release --example dsigner_legacy_http --all-features

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/examples/dsigner_legacy_http /usr/local/bin
ENTRYPOINT ["/usr/local/bin/dsigner_legacy_http"]
