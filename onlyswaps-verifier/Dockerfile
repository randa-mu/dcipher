# Image building solidity contracts
ARG NODE_VERSION=22.3
FROM europe-west1-docker.pkg.dev/randamu-prod/candyland/node:${NODE_VERSION} AS sol_builder
WORKDIR /app

RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

FROM sol_builder AS sol_onlyswaps
WORKDIR /app/onlysubs-solidity
COPY ./onlysubs-solidity/package.json ./
COPY ./onlysubs-solidity/package-lock.json ./
RUN npm install

COPY ./onlysubs-solidity ./
RUN FOUNDRY_PROFILE=build forge install --no-git && FOUNDRY_PROFILE=build forge build

# Base image for rust
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
COPY --from=sol_onlyswaps /app/onlysubs-solidity/out ./onlysubs-solidity/out
RUN cargo build --release -p onlyswaps-verifier

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils
WORKDIR /app
COPY --from=builder /app/target/release/onlyswaps-verifier /usr/local/bin
ENTRYPOINT ["/usr/local/bin/onlyswaps-verifier"]
