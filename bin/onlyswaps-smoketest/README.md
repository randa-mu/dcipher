# only swaps smoketest

A continuous monitoring and testing tool for only swaps. 
This service periodically executes configured swap tests and exposes metrics for monitoring swap success rates, fulfillment, and verification status.

## Metrics

The service exposes the following Prometheus metrics on the healthcheck endpoint (`/metrics`):

- `smoketest_swap_requested{label}`: Total number of swaps requested per test label
- `smoketest_swap_fulfilled{label}`: Total number of swaps successfully fulfilled per test label
- `smoketest_swap_verified{label}`: Total number of swaps verified on-chain per test label
- `smoketest_swap_failed{label,reason}`: Total number of failed swaps per test label and failure reason

## Configuration

The service is configured via a TOML file. By default, it looks for the config at `~/.config/onlyswaps/smoketest/config.toml`, but this can be overridden.

### Configuration File Structure

```toml
# Private key for signing transactions (32 bytes as hex)
eth_private_key = "deadbeef"

[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 8080
log_level = "onlyswaps_smoketest=info,onlyswaps_client=info,warn"
log_json = true

# Network configurations
[[networks]]
chain_id = 43113
rpc_url = "wss://avalanche-fuji-c-chain-rpc.publicnode.com"
router_address = "0xC69DD549B037215BA1Ea9866FFa59603862bf986"

[[networks]]
chain_id = 84532
rpc_url = "wss://base-sepolia-rpc.publicnode.com"
router_address = "0xC69DD549B037215BA1Ea9866FFa59603862bf986"

# Swap test configurations
[[swaps]]
amount = "1_000_000_000_000_000_000" # 1 RUSD (18 decimals)
src_token = "RUSD"
dst_token = "RUSD"
src_chain_id = 84532
dst_chain_id = 43113
label = "rusd_base_sepolia_to_ava_fuji"
interval = "10m"       # Run test every 10 minutes
timeout = "60s"        # Fail if not completed in 60 seconds
recipient = "0x..."    # Optional: specify recipient address, otherwise uses signer's address
```

### Configuration Parameters

#### Agent Section
- `healthcheck_listen_addr`: IP address to bind the healthcheck server (default: "0.0.0.0")
- `healthcheck_port`: Port for healthcheck and metrics endpoint (default: 8080)
- `log_level`: Logging configuration using Rust tracing filter syntax
- `log_json`: Enable JSON-formatted logs for structured logging

#### Networks Section
- `chain_id`: EVM chain ID
- `rpc_url`: RPC endpoint URL (supports both HTTP and WebSocket)
- `router_address`: only swaps router contract address

#### Swaps Section
- `amount`: Token amount to swap (in smallest unit, e.g., wei for 18 decimal tokens)
- `src_token`: Source token tag (e.g., "RUSD", "USDT")
- `dst_token`: Destination token tag
- `src_chain_id`: Source chain ID
- `dst_chain_id`: Destination chain ID
- `label`: Unique identifier for this test (used in metrics)
- `interval`: Time between test executions (e.g., "60s", "5m", "1h")
- `timeout`: Maximum time to wait for swap completion (e.g., "60s", "2m")
- `recipient`: (Optional) Recipient address. If not specified, uses the signer's address
