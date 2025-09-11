# DSigner - Distributed Signer

A threshold signature service for distributed signing with BN254 signatures.

## Quick Start (Single Node Setup)

For a simple single-node setup with threshold = 1 and number of parties = 1:

### Prerequisites

- Rust toolchain installed
- A BLS private key (BN254 curve)
- A libp2p private key

### Basic Usage

```bash
# Run with minimal required arguments
cargo run --features=server --example dsigner_legacy_http -- \
  --bls-key "your_bls_private_key_here" \
  --libp2p-key "your_libp2p_private_key_here"
```

### Using Environment Variables

Create a `.env` file or export environment variables:

```bash
export DSIGNER_BLS_KEY="your_bls_private_key_here"
export DSIGNER_LIBP2P_KEY="your_libp2p_private_key_here"
export DSIGNER_NODE_ID="1"
export DSIGNER_N_PARTIES="1"
export DSIGNER_THRESHOLD="1"
export DSIGNER_APPLICATION_ARGUMENTS="evmnet"
export DSIGNER_LISTEN_ADDR="0.0.0.0"
export DSIGNER_PORT="8080"
export DSIGNER_LOG_LEVEL="info"
export DSIGNER_LRU_CACHE_SIZE="64"
export DSIGNER_SIG_COMPRESSION="true"
export DSIGNER_LIBP2P_LISTEN_ADDR="/ip4/0.0.0.0/tcp/9001"

# Then run
./dsigner
```

## Configuration Options

### Server Configuration

| Argument | Environment Variable | Default | Description |
|----------|---------------------|---------|-------------|
| `--listen-addr` | `DSIGNER_LISTEN_ADDR` | `0.0.0.0` | HTTP server listen address |
| `--port` | `DSIGNER_PORT` | `8080` | HTTP server port |
| `--log-level` | `DSIGNER_LOG_LEVEL` | `info` | Logging level (info, debug, error, trace) |
| `--lru-cache-size` | `DSIGNER_LRU_CACHE_SIZE` | `64` | LRU cache size for signatures |

### Key Configuration

| Argument                  | Environment Variable | Default      | Description |
|---------------------------|---------------------|--------------|-------------|
| `--bls-key`               | `DSIGNER_BLS_KEY` | **Required** | BLS private key for signing |
| `--node-id`               | `DSIGNER_NODE_ID` | `1`          | Node identifier |
| `-n`                      | `DSIGNER_N_PARTIES` | `1`          | Total number of parties |
| `-t`                      | `DSIGNER_THRESHOLD` | `1`          | Threshold required to sign |
| `--application-arguments` | `DSIGNER_APPLICATION_ARGUMENTS` | `evmnet`     | Application-specific arguments |
| `--sig-compression`       | `DSIGNER_SIG_COMPRESSION` | `false`      | Use signature compression |
| `--nodes-config`          | `DSIGNER_NODES_CONFIG` | Optional     | Path to nodes configuration file |

### libp2p Configuration

| Argument | Environment Variable | Default | Description |
|----------|---------------------|---------|-------------|
| `--libp2p-key` | `DSIGNER_LIBP2P_KEY` | **Required** | libp2p private key |
| `--libp2p-listen-addr` | `DSIGNER_LIBP2P_LISTEN_ADDR` | `/ip4/0.0.0.0/tcp/9001` | libp2p listen address |

## Application Arguments

The `application_arguments` field supports various signing applications:

### Blockchain-specific Applications
```bash
# Blocklock for specific chain
--application-arguments "blocklock:1"  # Ethereum mainnet
--application-arguments "blocklock:137"  # Polygon

# Randomness service
--application-arguments "randomness:1"

# OnlySwaps verifier
--application-arguments "onlyswaps_verifier:1"
```

### Generic Applications
```bash
# EVM network (default)
--application-arguments "evmnet"

# Any application with custom suffix
--application-arguments "any:custom_suffix"
```

## Multi-Node Setup

For signatures with n > 1, you'll need:

1. A `nodes_config.toml` file listing other nodes
2. Each node must have unique `node_id` values
3. All nodes must use the same `n` and `t` values

Example `nodes_config.toml`:
```toml
[[nodes]]
node_id = 1
bls_pk = "base64_encoded_public_key_of_node_1"
address = "/ip4/192.168.1.101/tcp/9001"
peer_id = "peer_id_of_node_1"

[[nodes]]
node_id = 2
bls_pk = "base64_encoded_public_key_of_node_2"
address = "/ip4/192.168.1.102/tcp/9001"
peer_id = "peer_id_of_node_1"

[[nodes]]
node_id = 3
bls_pk = "base64_encoded_public_key_of_node_3"  
address = "/ip4/192.168.1.103/tcp/9001"
peer_id = "peer_id_of_node_3"
```

Then run each node with:
```bash
./dsigner \
  --node-id 1 \
  -n 3 \
  -t 2 \
  --nodes-config nodes_config.toml \
  --bls-key "node_1_bls_key" \
  --libp2p-key "node_1_libp2p_key"
```


## HTTP API

The DSigner service exposes a REST API with the following endpoints:

### Health Check
```bash
# Check if the service is running
curl http://localhost:8080/healthcheck
# Response: "healthy"
```

### Get Public Key
```bash
# Retrieve the BLS public key for verification
curl http://localhost:8080/pk
# Response: "hex_encoded_public_key"
```

### Sign Message
```bash
# Sign a message using the threshold signature scheme
curl -X POST http://localhost:8080/sign \
-H "Content-Type: application/json" \
-d '{"m": "your_message_to_sign"}'

# Response:
# {
#   "signature": "base64_encoded_signature",
#   "dst": "domain_separation_tag"
# }
```
