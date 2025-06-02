# Image must be built from the root

# Image building solidity contracts
ARG NODE_VERSION=22.3
FROM europe-west1-docker.pkg.dev/randamu-prod/candyland/node:${NODE_VERSION} AS sol_builder
WORKDIR /app

RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

FROM sol_builder AS sol_blocklock
WORKDIR /app/blocklock-solidity
COPY ./blocklock-solidity/package.json ./
COPY ./blocklock-solidity/package-lock.json ./
RUN npm install

COPY ./blocklock-solidity ./
RUN FOUNDRY_PROFILE=build forge install --no-git && FOUNDRY_PROFILE=build forge build

FROM sol_builder AS sol_randomness
WORKDIR /app/randomness-solidity
COPY ./randomness-solidity/package.json ./
COPY ./randomness-solidity/package-lock.json ./
RUN npm install

COPY ./randomness-solidity ./
RUN FOUNDRY_PROFILE=build forge install --no-git && FOUNDRY_PROFILE=build forge build

# Base image for rust
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
COPY --from=sol_blocklock /app/blocklock-solidity/out ./blocklock-solidity/out
COPY --from=sol_randomness /app/randomness-solidity/out ./randomness-solidity/out
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . ../
RUN cargo build --release -p randomness-agent --example randomness

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/examples/randomness /usr/local/bin
ENTRYPOINT ["/usr/local/bin/randomness"]
