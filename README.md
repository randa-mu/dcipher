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

| Binary                | Purpose                                                                                                |
|-----------------------|--------------------------------------------------------------------------------------------------------|
| `adkg-cli`            | CLI for running and testing Asynchronous Distributed Key Generation ceremonies                         |
| `blocklock-agent`     | Agent for the Blocklock protocol (time-lock / conditional decryption)                                  |
| `dsigner`             | Threshold signing daemon, allowing operators to separate condition evaluation and signing for security |
| `gen-keys`            | Utility for key generation (testing / setup)                                                           |
| `onlyswaps-state-api` | API for caching and serviceexposing state related to ONLYSwaps                                         |
| `onlyswaps-verifier`  | A dcipher protocol implementation called ONLYSwaps for enabling cross-chain token swaps                |
| `randomness-agent`    | A dcipher protocol implementation for providing verifiable randomness on-chain                         |                                                                                                       |

---

### Crates (`crates/`)

| Crate            | Purpose                                                                                                                                                   |
|------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `adkg`           | Core Asynchronous Distributed Key Generation implementation                                                                                               |
| `agent-utils`    | Utilities shared by dcipher agents                                                                                                                        |
| `config`         | Shared configuration handling for agents that use networking, threshold signing, or load config files (TOML, env, etc.)                                   |
| `dcipher-agents` | Common agent framework code                                                                                                                               |
| `generated`      | Auto-generated Rust bindings from Solidity (kept in sync via `generate-bindings.sh`)                                                                      |
| `network`        | Networking primitives and libp2p integrations                                                                                                             |
| `omnievent`      | A library for streaming contract events sources into database sinks, and back into filterable streams for apps built on top of [alloy](https://alloy.rs/) |
| `signer`         | Signing logic (BLS / threshold compatible)                                                                                                                |
| `superalloy`     | A crate providing multiplexing logic for combining [alloy](https://alloy.rs/) providers                                                                   |
| `utils`          | General helper utilities                                                                                                                                  |

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
- Foundry (Forge)
- Node.js & npm
- `make`

### Quickstart

Build the repo (but not the tests):

```bash
cargo build
```

Build a specific bin (e.g. adkg-cli):

```bash
cargo build  --release -p adkg-cli
```

Build everything, including solidity and tests:

```bash
make all
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

## Contributing

1. Fork & clone the repo
2. Create a feature branch
3. Run `cargo test` and `forge test` before pushing
4. Open a PR and ensure CI passes

---

## License

Licensed under [MIT](LICENSE).
