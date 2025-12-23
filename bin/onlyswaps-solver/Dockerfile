FROM lukemathwalker/cargo-chef:latest-rust-1.92 AS chef
RUN apt-get update \
 && apt-get install -y protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin onlyswaps-solver

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --bin onlyswaps-solver

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

RUN cargo build --release -p onlyswaps-solver --bin onlyswaps-solver --all-features

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
