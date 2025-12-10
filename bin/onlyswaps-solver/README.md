# onlyswaps-solver

## Build
`cargo build`

## Test

`cargo test`

## Docker build

`docker build .`

## Configuration

| Environment Variable | Mandatory? | Description                                         | Example                                  | Default                                  |
|----------------------|------------|-----------------------------------------------------|------------------------------------------|------------------------------------------|
| `SOLVER_PRIVATE_KEY` | Yes        | A hex-encoded private key, with or without the `0x` | `0xdeadbeefdeadbeefdeadbeefdeadbeefdead` | —                                        |
| `SOLVER_CONFIG_PATH` | No         | Path to your solver configuration TOML              | `/data/config.toml`                      | `~/.config/onlyswaps/solver/config.toml` |

## Sample Config File

```toml
[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 8081
log_level = "debug"
log_json = true

[[networks]]
chain_id = 43113
rpc_url = "wss://avalanche-fuji-c-chain-rpc.publicnode.com"
tokens = ["0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"]
router_address = "0x83b2dFc83E41a2398e28e31C352E1053805e4C16"
permit2_relayer_address = "0x862acc167842c72B6f5B6b4091573dDE91A5AcfB"

[[networks]]
chain_id = 84532
rpc_url = "wss://base-sepolia-rpc.publicnode.com"
tokens = ["0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"]
router_address = "0x83b2dFc83E41a2398e28e31C352E1053805e4C16"
permit2_relayer_address = "0x862acc167842c72B6f5B6b4091573dDE91A5AcfB"
```

It is also possible to configure the solver with an external omnievent endpoint by adding the following to the config:
```toml
[omnievent]
endpoint = "https://omnievent:3284"
```
