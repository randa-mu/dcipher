# Blocklock Agent

A decentralized blocklock agent that fulfills on-chain blocklock requests through the [`blocklock-solidity`](https://github.com/randa-mu/blocklock-solidity) contracts

## Prerequisites
- Deployed [`blocklock-solidity`](https://github.com/randa-mu/blocklock-solidity) contracts,
- A committee configuration file, issued from a key generation ceremony.

## Configuration

The agent can be configured via command-line arguments, environment variables, or a TOML configuration file.

## Committee Configuration File

Create a `committee.toml` file with your committee settings:

```toml
# Your member ID within the committee
member_id = 1

# Your secret signing key (base64 encoded)
secret_key = "HBIjfH5BaOBP9yJKDiNaTSwYxPHNaz06wgHI58j1bOg="

# Total number of committee members
n = 4

# Minimum signatures required (threshold)
signing_threshold = 2

# Committee member details
[[members]]
member_id = 1
bls_pk = "5Bfq2cbTCQMB9iCB6zVU/BY0YShDCxtF4Imede78hFELCp8unobkd6XLf+Pno5dqJOVyEidv3XTKa53ubfcO2Q==" # compressed, base64 encoded
peer_id = "12D3KooWMSH17hbmMBSbEtCeyYBkFT7phd6PVaA2fdxakaTAuXMx"
address = "/ip4/127.0.0.1/tcp/15001"

[[members]]
member_id = 2
bls_pk = "qD1qMeZa/7dJ4ZoymmozhtYnTFn96KyqfWMunyGgfecgqHUDeAKXhul4UMiQTVRfl2CSUKpFulwUean1mVgIKg=="
peer_id = "12D3KooWMAADWEWNMFCNBadQfNFLeHHcYeZr9tNJBekY4opdCzDM"
address = "/ip4/127.0.0.1/tcp/15002"

[[members]]
member_id = 3
bls_pk = "lmnhNX86rLuqxNh+JxTX/GQnZPoP91g0ye8JB8dZAD4gKY00BgxhWtwdefMkShfWV7CNllRBJItcfemL0HBa/w=="
peer_id = "12D3KooWHpwkbFiQ9af1jm3JxH2gGj5s5dWjuzaTLkEvKXikvL1U"
address = "/ip4/127.0.0.1/tcp/15003"

[[members]]
member_id = 4
bls_pk = "pLNDWGmc1HflnVbJAzYVt/0Wymh8mGojIVeWXP53KvAM3uoUa8X4XTR0h7otlmlNoxqryKRTAAcl7aCtpV7dVg=="
peer_id = "12D3KooWFka9jyk6N9VtM36gsUgJPcmjw4NANxWgbx66VNgRAPcj"
address = "/ip4/127.0.0.1/tcp/15004"
```

### Creating the Committee File from ADKG Output

To create this file from the output of the ADKG tool in this repository, one must proceed as follows:
1. Take the public output of the ADKG, specifically `node_pks`, and fill the members list by renaming the various fields,
2. set the secret key from the ADKG's private output,
3. set `n` to the number of parties in the group file, and,
4. set `signing_threshold` to `t_reconstruction + 1` which is specified in the group file.

## Running the Agent

### Minimal Example

```bash
cargo run --example blocklock -- \
  --committee-config ./committee.toml \
  --rpc-url wss://wss.calibration.node.glif.io/apigw/lotus/rpc/v1 \
  --tx-private-key YOUR_TX_PRIVATE_KEY \
  --libp2p-key YOUR_LIBP2P_PRIVATE_KEY \
  --blocklock-sender-addr 0x1234567890123456789012345678901234567890 \
  --decryption-sender-addr 0x0987654321098765432109876543210987654321
```

## Configuration Options

### Required Arguments

| Argument                   | Environment Variable                           | Description                                |
|----------------------------|------------------------------------------------|--------------------------------------------|
| `--committee-config`       | `BLOCKLOCK_COMMITTEE_CONFIG`                   | Path to committee configuration TOML file  |
| `--rpc-url`                | `BLOCKLOCK_RPC_URL`                            | Blockchain WebSocket RPC URL               |
| `--tx-private-key`         | `BLOCKLOCK_TX_PRIVATE_KEY`                     | Private key for transaction signing        |
| `--libp2p-key`             | `BLOCKLOCK_LIBP2P_KEY`                         | Libp2p private key                         |
| `--blocklock-sender-addr`  | `BLOCKLOCK_SENDER_CONTRACT_ADDRESS`            | Deployed BlocklockSender contract address  |
| `--decryption-sender-addr` | `BLOCKLOCK_DECRYPTION_SENDER_CONTRACT_ADDRESS` | Deployed DecryptionSender contract address |

### Health Check

| Argument                    | Environment Variable                | Default   | Description                      |
|-----------------------------|-------------------------------------|-----------|----------------------------------|
| `--healthcheck-listen-addr` | `BLOCKLOCK_HEALTHCHECK_LISTEN_ADDR` | `0.0.0.0` | Health check server bind address |
| `--healthcheck-port`        | `BLOCKLOCK_HEALTHCHECK_PORT`        | `8080`    | Health check server port         |

### Blockchain Configuration

| Argument                       | Environment Variable                | Default     | Description                     |
|--------------------------------|-------------------------------------|-------------|---------------------------------|
| `--chain-id`                   | `BLOCKLOCK_CHAIN_ID`                | Auto-detect | Blockchain chain ID             |
| `--tx-fulfillment-disabled`    | `BLOCKLOCK_TX_FULFILLMENT_DISABLED` | `false`     | Disable transaction fulfillment |
| `--min-confirmations`          | `BLOCKLOCK_MIN_CONFIRMATIONS`       | `1`         | Required block confirmations    |
| `--confirmations-timeout-secs` | `BLOCKLOCK_CONFIRMATIONS_TIMEOUT`   | `60`        | Confirmation timeout (seconds)  |
| `--max-tx-per-tick`            | `BLOCKLOCK_MAX_TX_PER_TICK`         | unlimited   | Max transactions per tick       |
| `--tx-retry-strategy`          | `BLOCKLOCK_TX_RETRY_STRATEGY`       | `Never`     | Retry strategy                  |
| `--sync-batch-size`            | `BLOCKLOCK_SYNC_BATCH_SIZE`         | `20`        | Blockchain sync batch size      |

### Gas & Profitability

| Argument                     | Environment Variable                 | Default | Description                 |
|------------------------------|--------------------------------------|---------|-----------------------------|
| `--gas-price-buffer-percent` | `BLOCKLOCK_GAS_PRICE_BUFFER_PERCENT` | `20`    | Gas price bump percentage   |
| `--gas-buffer-percent`       | `BLOCKLOCK_GAS_BUFFER_PERCENT`       | `20`    | Gas limit buffer percentage |
| `--profit-threshold`         | `BLOCKLOCK_PROFIT_THRESHOLD_PERCENT` | `20`    | Minimum profit threshold    |

### Libp2p Networking

| Argument               | Environment Variable           | Default                 | Description                |
|------------------------|--------------------------------|-------------------------|----------------------------|
| `--libp2p-listen-addr` | `BLOCKLOCK_LIBP2P_LISTEN_ADDR` | `/ip4/0.0.0.0/tcp/9001` | Libp2p listen multiaddress |

### State & Logging

| Argument       | Environment Variable             | Default                  | Description                    |
|----------------|----------------------------------|--------------------------|--------------------------------|
| `--state-file` | `BLOCKLOCK_SAVED_STATE_FILENAME` | `./blocklock_state.json` | Persistent agent state file    |
| `--log-level`  | `BLOCKLOCK_LOG_LEVEL`            | `info`                   | Log verbosity                  |
| `--log-json`   | `BLOCKLOCK_LOG_JSON`             | `false`                  | Enable structured JSON logging |
