#!/usr/bin/env bash

# This script generates rust bindings into the `generated` crate using forge.
# If you add a new project to it, you should also:
# - update the 'check-rust-bindings' github action to build it
# - update the `make clean_generated` target to remove it.
#
# We only generate bindings for a subset of the solidity files, so if some bindings are missing
# when you compile, you might need to add their solidity contract name with `--select` below

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# you might expect that forge uses the one in the forge.toml file...
# you'd be wrong :|
SOLC_VERSION=0.8.30

# As we're generating modules, we can skip the cargo toml check..
# You _can_ generate *ALL* the bindings for each project, but it slows down the build
# and there are weird conflicts with the `Factory` CREATE2 deployer stuff that presently break with foundry
# see: https://github.com/foundry-rs/foundry/issues/11705
pushd "$ROOT_DIR/modules/randomness-solidity"
forge bind --bindings-path "$ROOT_DIR/crates/generated/src/randomness" \
--skip-cargo-toml \
--module \
--use $SOLC_VERSION \
--select RandomnessSender \
--select TypesLib \
--select SignatureSender \
--no-metadata
popd

pushd "$ROOT_DIR/modules/blocklock-solidity"
forge bind --bindings-path "$ROOT_DIR/crates/generated/src/blocklock" \
--skip-cargo-toml \
--module \
--use $SOLC_VERSION \
--select BlocklockSender \
--select TypesLib \
--select DecryptionSender \
--select SignatureSender \
--select SignatureSchemeAddressProvider \
--select BlocklockSignatureScheme \
--select UUPSProxy \
--select MockBlocklockReceiver \
--no-metadata
popd

pushd "$ROOT_DIR/modules/onlyswaps-solidity"
forge bind --bindings-path "$ROOT_DIR/crates/generated/src/onlyswaps" \
--skip-cargo-toml \
--module \
--use $SOLC_VERSION \
--select SwapRequestParameters \
--select IRouter \
--select ERC20FaucetToken \
--select IERC20 \
--select ErrorsLib \
--select Permit2Relayer \
--no-metadata
