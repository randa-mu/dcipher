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
# forge needs git folder smh
COPY ./.git ../.git
# compile contracts
RUN npm run build:forge

FROM sol_builder AS sol_randomness
WORKDIR /app/randomness-solidity
COPY ./randomness-solidity/package.json ./
COPY ./randomness-solidity/package-lock.json ./
RUN npm install

COPY ./randomness-solidity ./
# forge needs git folder smh
COPY ./.git ../.git
# compile contracts
RUN npm run build:forge

# Base image for rust
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app/blocklock-agent

FROM chef AS planner
COPY . ../
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/blocklock-agent/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
COPY ./marshmallow-crypto ../marshmallow-crypto
COPY ./superalloy ../superalloy
COPY ./dcipher-agents ../dcipher-agents
COPY --from=sol_blocklock /app/blocklock-solidity/out ../blocklock-solidity/out
COPY --from=sol_randomness /app/randomness-solidity/out ../randomness-solidity/out
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY ./blocklock-agent ./
RUN cargo build --release --example blocklock

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/blocklock-agent/target/release/examples/blocklock /usr/local/bin
ENTRYPOINT ["/usr/local/bin/blocklock"]
