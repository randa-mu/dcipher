# onlyswaps-verifier

onlyswaps-verifier is an agent for verifying crosschain swap fulfilments by solvers in the dcipher network.
It listens across a configurable set of EVM blockchains, verifies solvers' reported fulfillments on destination chains against swap requests on source chains, and cooperatively creates threshold signatures to unlock reimbursements and earned fees for honest solvers.

## Quickstart
Run the verifier by building it and running
`/path/to/onlyswaps-verifier start --config-file /path/to/config/file`

## CLI flags
- **--config-file <path-to-some-file>**
A TOML or JSON file containing the rest of the configuration parameters required to connect to EVM blockchains and dcipher nodes. See [Configuration](#configuration).
 
## Configuration
An annotated, sample TOML configuration can be found below.
```toml
longterm_secret_path = "/path/to/longterm/secret/key/file" # you generated this before running the ADKG
group_path = "/path/to/group/file/pre/adkg"                # you received this before the adkg to know who to connect to
adkg_public_path = "/path/to/pub/adkg/output"              # this was created during the adkg
adkg_secret_path = "/path/to/priv/adkg/output"             # this was created during the adkg
eth_private_key = "/path/to/private/key"                   # or you can put it in directly like "0x12345678959726c7020bca2612345678959851c13c1561b399ad8dde5207b57c"
member_id = 1                                              # your ID in the adkg_public file
listen_addr = "/ip4/0.0.0.0/tcp/9898"                      # this is the address you bind locally, not necessarily the multiaddr others connect to you with

# `agent` is used for general configuration and monitoring params
[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 9999                     # make sure not to bind  the same as the listen_addr!
log_level = "debug"                         # debug, info, trace, error
log_json = true                             # whether the logs should be structured as JSON or plaintext

# `networks` details all the configuration relating to connecting to blockchains. Each can be configured independently.
# Presently all networks must be supported, and skipping verifications for one route (chain -> chain) may cause errors.
[[networks]]
chain_id = 43114
rpc_url = "wss://avalanche-c-chain-rpc.publicnode.com"                                          # presently only websockets and websockets secure are supported 
router_address = "0x3dD1a497846d060Dce130B67b22E1F9DeE18c051"                      # the address for the router contract (/or proxy) for the given chain
should_write = false                                                               # controls whether this node actually writes signatures back to the chain to verify swaps.

[[networks]]
chain_id = 8453
rpc_url = "wss://base-rpc.publicnode.com"
router_address = "0x3dD1a497846d060Dce130B67b22E1F9DeE18c051"
should_write = false
```
