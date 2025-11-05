# dcipher

[![Build](https://img.shields.io/github/actions/workflow/status/randa-mu/dcipher/rust-build-and-tests.yml?branch=main)](https://github.com/randa-mu/dcipher/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Stars](https://img.shields.io/github/stars/randa-mu/dcipher?style=social)](https://github.com/randa-mu/dcipher)

---

## Overview

**dcipher** is a modular protocol for [threshold cryptography](https://en.wikipedia.org/wiki/Threshold_cryptosystem).  
It combines smart contracts and off-chain node operators to enable:

- Asynchronous Distributed Key Generation
- Threshold signing
- Identity-based encryption
- Chain abstraction

The repository is organized into:

- **Binaries (`bin/`)** – runnable agents, CLIs, and services.
- **Crates (`crates/`)** – Rust libraries used across the project.
- **Modules (`modules/`)** – **Git submodule dependencies** that pull in protocol-specific logic or external components.

---

## Repository Structure

### Binaries (`bin/`)

| Binary                | Purpose                                                                                                | Documentation                                 |
|-----------------------|--------------------------------------------------------------------------------------------------------|-----------------------------------------------|
| `adkg-cli`            | CLI for running and testing Asynchronous Distributed Key Generation ceremonies                         | [README](./bin/adkg-cli/README.md)            |
| `blocklock-agent`     | Agent for the Blocklock protocol (time-lock / conditional decryption)                                  |                                               | 
| `dsigner`             | Threshold signing daemon, allowing operators to separate condition evaluation and signing for security | [README](./bin/dsigner/README.md)             |
| `gen-keys`            | Utility for key generation (testing / setup)                                                           | [README](./bin/gen-keys/README.md)            |
| `monitoring`          | An agent that listens/polls dcipher-related information and ships it to prometheus                     | [README](./bin/monitoring/README.md)          |
| `onlyswaps-smoketest` | A continuous monitoring and testing tool for only swaps.                                               | [README](./bin/onlyswaps-smoketest/README.md) |
| `onlyswaps-solver`    | An agent for providing token liquidity on chains that support onlyswaps to make yield.                 | [README](./bin/onlyswaps-solver/README.md)    |
| `onlyswaps-state-api` | API for caching and serving state related to ONLYSwaps                                                 |                                               |
| `onlyswaps-verifier`  | A dcipher protocol implementation called ONLYSwaps for enabling cross-chain token swaps                | [README](./bin/onlyswaps-verifier/README.md)  |
| `randomness-agent`    | A dcipher protocol implementation for providing request-based verifiable randomness on-chain           |                                               |

---

### Crates (`crates/`)

| Crate              | Purpose                                                                                                                                                   |
|--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `adkg`             | Core Asynchronous Distributed Key Generation implementation                                                                                               |
| `agent-utils`      | Utilities shared by dcipher agents                                                                                                                        |
| `config`           | Shared configuration handling for agents that use networking, threshold signing, or load config files (TOML, env, etc.)                                   |
| `dcipher-agents`   | Common agent framework code                                                                                                                               |
| `generated`        | Auto-generated Rust bindings from Solidity (kept in sync via `generate-bindings.sh`)                                                                      |
| `network`          | Networking primitives and libp2p integrations                                                                                                             |
| `onlyswaps-client` | A client to interact with onlyswaps                                                                                                                       |
| `omnievent`        | A library for streaming contract events sources into database sinks, and back into filterable streams for apps built on top of [alloy](https://alloy.rs/) |
| `signer`           | Signing logic (BLS / threshold compatible)                                                                                                                |
| `superalloy`       | A crate providing multiplexing logic for combining [alloy](https://alloy.rs/) providers                                                                   |
| `utils`            | General helper utilities                                                                                                                                  |

---

### Modules (`modules/`)

These are **Git submodule dependencies**, not local crates.  
They typically include protocol-specific or external components maintained in separate repositories.  
Update them with:

```bash
git submodule update --init --recursive
```

---

## Getting Started

### Prerequisites

- Rust `1.89.0+`
- Node.js & npm
- `make`
- Foundry (if running the tests)

### Quickstart

Build everything, including solidity and tests:

```bash
make all
```

Build the repo (but not the tests):

```bash
cargo build
```

Build a specific binary (e.g. adkg-cli):

```bash
cargo build --release -p adkg-cli
```

Build Docker images (requires binaries to be built first):

```bash
make build_binaries
make build_docker_randomness-agent
make build_docker_onlyswaps-verifier
```

Clean artifacts:

```bash
make clean
make clean_node_modules
make clean_forge
```

---

## Bindings

If Solidity contracts change, regenerate Rust bindings:

```bash
./generate-bindings.sh
```

CI will fail if bindings are out of sync.

---

## Testing

- Rust crates:

```bash
cargo test --workspace
```

---
## Cross compiling
### Using cross (recommended)
If you're on a non linux machine and want to build against linux dylibs, you can use `cross`.
Note that cross requires docker to be available, see the cross documentation for complex cases.

1. First ensure you've installed dev-dependencies or have otherwise installed `cross`

2. Then you can invoke `cross` like you would cargo, by default this will build you GLIBC compatible
build:
```bash
cross build --release
```

If your build succeeds you will find a directory corresponding to the architecture in the `target` directory:
```bash
$ find target -type f -name adkg-cli -exec file {} \;
target/x86_64-unknown-linux-gnu/release/adkg-cli: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=9893b95e8c056757dcf5651a3237a1e84b9f72dc, not stripped
target/debug/adkg-cli: Mach-O 64-bit executable arm64
```

3. If you wish to target MUSL you need to specify the target as follows
```bash
cross build --release --target x86_64-unknown-linux-musl
```

If you require another target, simply add it to the `Cross.toml` file

4. If you're running rosetta with docker, ensure you specify the architecture when running docker commands, e.g.
```bash
$ docker run --rm \
  -v $(pwd)/target/x86_64-unknown-linux-gnu/release/adkg-cli:/usr/local/bin/onlyswaps-solver \
  --platform linux/amd64 \
  debian:bookworm-slim \
  /usr/local/bin/onlyswaps-solver
Unable to find image 'debian:bookworm-slim' locally
bookworm-slim: Pulling from library/debian
1adabd6b0d6b: Pull complete
Digest: sha256:936abff852736f951dab72d91a1b6337cf04217b2a77a5eaadc7c0f2f1ec1758
Status: Downloaded newer image for debian:bookworm-slim
CLI for key generation and distributed key generation.

Usage: onlyswaps-solver [OPTIONS] <COMMAND>
```

---
## Contributing

1. Fork & clone the repo
2. Create a feature branch
3. Run `cargo test` and `forge test` before pushing
4. Open a PR and ensure CI passes

---

## License

Licensed under [MIT](LICENSE).
