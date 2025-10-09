ARG NODE_VERSION=22.3
FROM europe-west1-docker.pkg.dev/randamu-prod/candyland/node:${NODE_VERSION} AS sol_builder
WORKDIR /app

RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

FROM sol_builder AS solidity-builder
WORKDIR /app/onlyswaps-solidity
COPY ./onlyswaps-solidity/package.json ./
COPY ./onlyswaps-solidity/package-lock.json ./
RUN npm install

COPY ./onlyswaps-solidity ./
RUN npm run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY --from=solidity-builder /app/onlyswaps-solidity/out ./onlyswaps-solidity/out

# Build application
COPY . .
RUN cargo build --release

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
COPY config-default.toml /app/config.toml
ENTRYPOINT ["/usr/local/bin/onlyswaps-solver", "--config", "/app/config.toml"]
