#!/usr/bin/env bash

# This script generates rust bindings into the `generated` crate using forge.
# If you add a new project to it, you should also:
# - update the 'check-rust-bindings' github action to build it
# - update the `make clean` target to remove it.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# you might expect that forge uses the one in the forge.toml file...
# you'd be wrong :|
SOLC_VERSION=0.8.28

# As we're generating modules, we can skip the cargo toml check..
# You _can_ generate *ALL* the bindings for each project, but it slows down the build
# and there are weird conflicts with the `Factory` CREATE2 deployer stuff that presently break with foundry
# see: https://github.com/foundry-rs/foundry/issues/11705
pushd $ROOT_DIR/randomness-solidity
forge bind --bindings-path $ROOT_DIR/generated/src/randomness \
--skip-cargo-toml \
--module \
--use $SOLC_VERSION \
--select RandomnessSender \
--select TypesLib \
--select SignatureSender \
--no-metadata
popd

pushd $ROOT_DIR/blocklock-solidity
forge bind --bindings-path $ROOT_DIR/generated/src/blocklock \
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

pushd $ROOT_DIR/onlyswaps-solidity
forge bind --bindings-path $ROOT_DIR/generated/src/onlyswaps \
--skip-cargo-toml \
--module \
--use $SOLC_VERSION \
--select Router \
--select IRouter \
--select ERC20FaucetToken \
--select SwapRequestParameters \
--select SwapRequestReceipt \
--no-metadata
