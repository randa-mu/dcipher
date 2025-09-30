# onlyswaps-verifier

onlyswaps-verifier is an agent for verifying crosschain swap fulfilments by solvers in the dcipher network.
It listens across a configurable set of EVM blockchains, verifies solvers' reported fulfillments on destination chains
against swap requests on source chains, and cooperatively creates threshold signatures to unlock reimbursements and
earned fees for honest solvers.

## Start the Verifier

You can start the verifier by running:

```bash
onlyswaps-verifier start --config-file <CONFIG_FILE>
```

### CLI flags

- **--config-file <path-to-some-file>**
  A TOML or JSON file containing the rest of the configuration parameters required to connect to EVM blockchains and
  dcipher nodes. See [Configuration](#configuration).

### Configuration

An annotated, sample TOML configuration can be found below.  
You can generate a basic configuration using the [generate configuration](#generate-a-configuration-file) CLI command.

```toml
       # `agent` is used for general configuration and monitoring params
[agent]
healthcheck_listen_addr = "0.0.0.0"
healthcheck_port = 9999
log_level = "debug"     # debug, info, trace, error
log_json = true         # whether the logs should be structured as JSON or plaintext

# `networks` details all the configuration relating to connecting to blockchains. Each can be configured independently.
# Presently all networks must be supported, and skipping verifications for one route (chain -> chain) may cause errors.
[[networks]]
chain_id = 31337
rpc_url = "ws://localhost:31337"                                                   # presently only websockets and websockets secure are supported
router_address = "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd"                      # the address for the router contract (/or proxy) for the given chain
private_key = "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2" # an Ethereum private key for writing verification signatures back to the chain. Can be 0x0000000000000000000000000000000000000000000000000000000000000000 if `should_write` is false.
should_write = false                                                               # controls whether this node actually writes signatures back to the chain to verify swaps.

[[networks]]
chain_id = 1338
rpc_url = "ws://localhost:1338"
router_address = "0x1293f79c4fa7fa83610fa5ef8064ef64929ee2fd"
private_key = "0x868c3482353618000889b0e733022108e174bb821e1fdb43bb56dc8115e218d2"
should_write = false

# `lib2p` contains everything related the (libp2p) networking layer between dcipher nodes. It's used to gossip partial signatures over floodsub
# You should follow the operator guide to creating a secret key to get all the details here
[libp2p]
secret_key = "Q0FFU1FOZU5VaVN0MjZNVlVlcTBtRjF6ZVpZZWgybVRVc0NMVjJrZUpGMEVkNStIVkxlQlBXTahsR9dVaUJacVh2eFVfOFpWbk1CVnlDenFtaUFtRzVBRW5Mcz0" # secret_key should be base-64 encoded in the protobuf format specified in the [libp2p spec](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md#keys)
multiaddr = "/dns/example.org/tcp/8080"                                                                                                      # dns and other multiaddr protocols are supported as you'd expect

# `committee` contains everything related to the BLS BN254 keyshare and associated group of dcipher operators. 
# You ought to have run a DKG protocol (or shamir-secret-sharing) to retrieve the `secret_key` here.
[committee]
member_id = 1                                                                     # your index in the `committee.nodes` vector.
secret_key = "0x2800cafe7d54bcc5cc21d37a2e4e67a49654fc7ddf16bf616e15091962426f8d" # your BLS BN254 secret key share encoded as `0x` prefixed hex
t = 2                                                                             # the honest threshold required for reconstruction. This is *different* to the malicious threshold output by the [ADKG CLI](../bin/adkg-cli)
n = 3                                                                             # the total count of members in the committee, including yourself.

# `committee.members` should contain as many entries as `n` above, and should contain one for your own node.
# `address` and `peer_id` aren't bound to the public key, so different dcipher nodes can connect via different routes.
# `member_id`s should be monotonically increasing from 1-n and unique, though can be out of order in this list.
[[committee.members]]
member_id = 1
bls_pk = "yFCy1kJ6Goeq0jFuVVTPICNh/1fNhf5PaIRs4847Z58uN00sxx87rMNHXae2RreBNkzrhP/3yJ+6vrNASPmHRg==" # BLS BN254 public key in standard base64 encoding
address = "/dns/example.org/tcp/8080"                                                               # a libp2p multiaddr 
peer_id = "12D3KooWJ4kJ5e9uY6aH9c8o8gQfupVx41Yx9QxQ9yPZy2m6Yt8b"                                    # a Peer ID as per the [libp2p spec](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md#peer-ids)

[[committee.members]]
member_id = 2
bls_pk = "yFCy1kJ6Goeq0jFuVVTPICNh/2fNhf5PaIRs4847Z58uN00sxx87rMNHXae2RreBNkzrhP/3yJ+6vrNASPmHRg=="
address = "/ip4/253.1.25.1/tcp/8081"
peer_id = "12D3KooWJ4kJ5e9uY6aH9c8o8gQfupVx41Yx9QxQ9yPZy2m6Yt8c"

[[committee.members]]
member_id = 3
bls_pk = "yFCy1kJ6Goeq0jFuVVTPICNh/3fNhf5PaIRs4847Z58uN00sxx87rMNHXae2RreBNkzrhP/3yJ+6vrNASPmHRg=="
address = "/ip4/127.0.0.1/tcp/8082"
peer_id = "12D3KooWJ4kJ5e9uY6aH9c8o8gQfupVx41Yx9QxQ9yPZy2m6Yt8d"
```

## Generate a Configuration File

If you have a run a successful DKG using the [ADKG CLI](../adkg-cli/README.md), you can generate a corresponding config
for the onlyswaps-verifier using this command.  
Running configuration generation can be done as follows:

```bash
onlyswaps-verifier generate-config \
  --private /path/to/longterm/private/key \       
  --group /path/to/group.toml \
  --public-share /path/to/public/keyshare \
  --private-share /path/to/private/keyshare \
  --multiaddr "/ip4/0.0.0.0/tcp/1234" \
  --member-id 1 \
  --contract-addr "0x4cB630aAEA9e152db83A846f4509d83053F21078" \
  --environment "mainnet"
```

### CLI flags

| Flag                                              | Description                                                                                                                                                            |
|---------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `--private <FILE>`                                | Path to the node’s long-term **private key**. This key uniquely identifies your verifier across sessions and should be kept secret.                                    |
| `--group <FILE>`                                  | Path to the **group definition file** (`.toml`) produced by the ADKG ceremony. This file contains the committee membership and public parameters.                      |
| `--public-share <FILE>`                           | Path to the **public keyshare** for this member, typically distributed after the DKG/ADKG run. This is safe to share with other participants.                          |
| `--private-share <FILE>`                          | Path to the **private keyshare** corresponding to your role in the committee. This must be kept secure.                                                                |
| `--multiaddr <ADDR>`                              | The [libp2p multiaddress](https://docs.libp2p.io/concepts/addressing/) the verifier should bind to for peer-to-peer communication. Example: `"/ip4/0.0.0.0/tcp/1234"`. |
| `--member-id <N>`                                 | The numeric identifier of this member in the committee (as defined in the `group.toml`).                                                                               | 
| (Optional) `--contract-addr <0x prefixed string>` | The 0x prefixed contract address for the ONLYSwaps [Router contract](https://github.com/randa-mu/onlyswaps-solidity/blob/main/src/Router.sol).                         | 
| (Optional) `--environment <testnet\|mainnet>`     | Whether to generate a config for testnet networks (e.g. Base Sepolia, Avalanche Fuji) or their mainnet counterparts.                                                   |
