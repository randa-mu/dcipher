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
# In addition to listening to events, poll the chain state every 10s instead of the default 30s
poll_interval = "10s"
```

It is also possible to configure the solver with an external omnievent endpoint by adding the following to the config:
```toml
[omnievent]
endpoint = "https://omnievent:3284"
```

## Initial setup
The current version of the solver relies on [Uniswap's permit2 contract](https://docs.uniswap.org/contracts/permit2/overview) for EIP-712 (i.e., gasless) token approvals for any ERC20 tokens.
This requires a one-time setup which consists of giving an unlimited token approval to the permit2 contract for each configured tokens.
Given that the contracts have been audited (see [here](https://github.com/Uniswap/permit2/tree/main/audits)), and have undergone public scrutiny for several years, the risk of an unlimited approval to permit2 is very low.

The initial setup can be done with the following command:
```bash
> cargo run -p onlyswaps-solver -- --config ./path/to/my/config.toml --private-key $PRIV setup
Loading app config from ./path/to/my/config.toml
The following allowances are required:
Chain      Token Address                                Permit2
------------------------------------------------------------------------------------------
56         0x55d398326f99059fF775485246999027B3197955   Default
314        0x80B98d3aa09ffff255c3ba4A241111Ff1262F045   Custom (0x1Ea2dBcB20263a969A017022E8B1C1dc13BD2470)
43114      0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76   Default

Proceed? [y/n]: y
Sending txs...
> [Chain 56] sending approve for 0x55d398326f99059fF775485246999027B3197955...
> [Chain 314] sending approve for 0x80B98d3aa09ffff255c3ba4A241111Ff1262F045...
> [Chain 43114] sending approve for 0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76...
> [Chain 56] approval for 0x55d398326f99059fF775485246999027B3197955 sent successfully
> [Chain 43114] approval for 0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76 sent successfully
> [Chain 314] approval for 0x80B98d3aa09ffff255c3ba4A241111Ff1262F045 sent successfully

Transaction results:
Chain      Token Address                                Result
------------------------------------------------------------------------------------------
56         0x55d398326f99059fF775485246999027B3197955   mined in 0xc1fc22d51f3130574da2c2501ed10aca986ab5e3a388b9f9d57717d630374d1c
314        0x80B98d3aa09ffff255c3ba4A241111Ff1262F045   mined in 0xf2f206ed3439b80a43c08bfd042da99596f689cdc155243bea187e47f5cbeb5f
43114      0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76   mined in 0x4044eadc976bdd4546649e41e573eca30786593969b1ca4582d713892a1e4134
```

After executing the command, the binary first checks which chain / token pairs require an approval, and ask for confirmation.
Upon acceptance, a transaction per chain / token pair is sent, and the result is displayed.
