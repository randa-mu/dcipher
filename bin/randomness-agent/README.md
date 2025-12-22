
# Randomness Agent

A decentralized randomness agent that fulfills randomness requests on blockchain networks through the [`randomness-solidity`](https://github.com/randa-mu/randomness-solidity) contracts.

## Prerequisites
- Deployed [`randomness-solidity`](https://github.com/randa-mu/randomness-solidity) contracts,
- A committee configuration file, issued from a key generation ceremony.

## Configuration

The agent can be configured via command-line arguments, environment variables, or a TOML configuration file.

### Committee Configuration File

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

To create this file from the output of the ADKG tool in this repository, one must proceed as follows:
1. Take the public output of the ADKG, specifically `node_pks`, and fill the members list by renaming the various fields,
2. set the secret key from the ADKG's private output,
3. set `n` to the number of parties in the group file, and,
4. set `signing_threshold` to `t_reconstruction + 1` which is specified in the group file.

## Running the Agent

### Minimal Example

```bash
cargo run --example randomness -- \
  --committee-config ./committee.toml \
  --rpc-url wss://eth.drpc.org \
  --tx-private-key YOUR_PRIVATE_KEY \
  --libp2p-key YOUR_LIBP2P_PRIVATE_KEY \
  --signature-sender-addr 0x1234567890123456789012345678901234567890 \
  --randomness-sender-addr 0x0987654321098765432109876543210987654321
```

Note that parameters may also be set through environment variables, see the table below.

## Configuration Options

### Required Arguments

| Argument                   | Environment Variable                           | Description                                       |
|----------------------------|------------------------------------------------|---------------------------------------------------|
| `--committee-config`       | `RANDOMNESS_COMMITTEE_CONFIG`                  | Path to committee configuration TOML file         |
| `--rpc-url`                | `RANDOMNESS_RPC_URL`                           | Blockchain websockets RPC URL                     |
| `--tx-private-key`         | `RANDOMNESS_TX_PRIVATE_KEY`                    | Private key for signing transactions (hex format) |
| `--libp2p-key`             | `RANDOMNESS_LIBP2P_KEY`                        | Private key for libp2p networking                 |
| `--signature-sender-addr`  | `RANDOMNESS_SIGNATURE_SENDER_CONTRACT_ADDRESS` | Address of deployed SignatureSender contract      |
| `--randomness-sender-addr` | `RANDOMNESS_SENDER_CONTRACT_ADDRESS`           | Address of deployed RandomnessSender contract     |

### Optional Arguments

#### Health Check

| Argument                    | Environment Variable                 | Default   | Description                        |
|-----------------------------|--------------------------------------|-----------|------------------------------------|
| `--healthcheck-listen-addr` | `RANDOMNESS_HEALTHCHECK_LISTEN_ADDR` | `0.0.0.0` | IP address for health check server |
| `--healthcheck-port`        | `RANDOMNESS_HEALTHCHECK_PORT`        | `8080`    | Port for health check server       |

#### Blockchain Configuration

| Argument                        | Environment Variable                 | Default     | Description                                                            |
|---------------------------------|--------------------------------------|-------------|------------------------------------------------------------------------|
| `--chain-id`                    | `RANDOMNESS_CHAIN_ID`                | Auto-detect | Blockchain network chain ID                                            |
| `--tx-fulfillment-disabled`     | `RANDOMNESS_TX_FULFILLMENT_DISABLED` | `false`     | Disable transaction fulfillment (monitoring only)                      |
| `--min-confirmations`           | `RANDOMNESS_MIN_CONFIRMATIONS`       | `1`         | Block confirmations required                                           |
| `--confirmations-timeout-secs`  | `RANDOMNESS_CONFIRMATIONS_TIMEOUT`   | `60`        | Timeout in seconds for confirmations                                   |
| `--max-tx-per-tick`             | `RANDOMNESS_MAX_TX_PER_TICK`         | unlimited   | Maximum transactions per processing cycle                              |
| `--tx-retry-strategy`           | `RANDOMNESS_TX_RETRY_STRATEGY`       | `Never`     | Retry strategy for failed transactions                                 |
| `--sync-batch-size`             | `RANDOMNESS_SYNC_BATCH_SIZE`         | `20`        | Batch size for blockchain sync                                         |
| `--sig-compression`             | `RANDOMNESS_SIG_COMPRESSION`         | `false`     | Enable on-chain signature compression                                  |
| `--contract-sync-interval-secs` | `RANDOMNESS_CONTRACT_SYNC_INTERVAL`  | `30`        | How often to sync the current state against the chain.                 |
| `--fulfillment-interval-secs`   | `RANDOMNESS_FULFILLMENT_INTERVAL`    | `60`        | How often to retry sending transactions / fulfilling pending requests. |

#### Gas & Profitability

| Argument                     | Environment Variable                  | Default | Description                         |
|------------------------------|---------------------------------------|---------|-------------------------------------|
| `--gas-price-buffer-percent` | `RANDOMNESS_GAS_PRICE_BUFFER_PERCENT` | `20`    | Gas price buffer percentage         |
| `--gas-buffer-percent`       | `RANDOMNESS_GAS_BUFFER_PERCENT`       | `20`    | Gas limit buffer percentage         |
| `--profit-threshold`         | `RANDOMNESS_PROFIT_THRESHOLD_PERCENT` | `20`    | Minimum profit threshold percentage |

#### Libp2p Networking

| Argument               | Environment Variable            | Default                 | Description                |
|------------------------|---------------------------------|-------------------------|----------------------------|
| `--libp2p-listen-addr` | `RANDOMNESS_LIBP2P_LISTEN_ADDR` | `/ip4/0.0.0.0/tcp/9001` | Libp2p listen multiaddress |

#### Logging

| Argument      | Environment Variable   | Default | Description                                 |
|---------------|------------------------|---------|---------------------------------------------|
| `--log-level` | `RANDOMNESS_LOG_LEVEL` | `info`  | Log level (trace, debug, info, warn, error) |
| `--log-json`  | `RANDOMNESS_LOG_JSON`  | `false` | Output logs in JSON format                  |
