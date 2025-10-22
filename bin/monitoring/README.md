# monitoring

An agent for listening to dcipher-related metrics and shipping them via the prometheus exporter.
The healthcheck listen addr is hijacked for serving metrics at `/metrics.`

## Sample Config

```toml

[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 8081
log_level = "debug"
log_json = true

[metrics]
frequency = "3s"        # this is how often the agent calls the chain to check balances

[[networks]]
chain_id = 43114
rpc_url = "wss://banana.com"

[[networks.wallets]]
label = "big alice"
address = "0x000000aAEA9e152db83A846f4509d83053F21078"

[[networks.wallets]]
label = "wee bob"
address = "0x000000aAEA9e152db83A846f4509d83053F21078"

[[networks.tokens]]
address = "0x000000000A9e152db83A846f4509d83053F21078"
symbol = "RUSD"
decimals = 18

[[networks.tokens]]
address = "0x000000aAEA9e152db83A846f4509d83053F21078"
symbol = "USDT"
decimals = 6

```