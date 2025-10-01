# adkg-cli

CLI tool used to prepare and execute asynchronous distributed key generation ceremonies using the high-threshold ADKG
described
in [Practical Asynchronous High-threshold Distributed Key Generation and Distributed Polynomial Sampling](https://www.usenix.org/system/files/usenixsecurity23-das.pdf)
by Das et al.

A ceremony is a three steps process: creation of a scheme specification, long-term key generation, and the execution of
the ADKG.

## Networking

The ADKG relies on libp2p to exchange messages between the various nodes.
The current implementation requires all nodes to use tcp communication, the traffic is secured
using [Noise](https://docs.libp2p.io/concepts/secure-comm/noise/), and multiplexed
with [Yamux](https://docs.libp2p.io/concepts/multiplex/yamux/).
The node must allow incoming traffic on its libp2p tcp port, and tcp outbound traffic must be allowed to the rest of the
nodes.

## Specifying a scheme

A scheme is used to describe an instance of the ADKG.
It contains various parameters, such as the curve, the number of parties and the malicious threshold.
Here is an example of a scheme specification used in the context of `dcipher`:

```toml
app_name = "dcipher"
curve_id = "Bn254G1"
hash_id = "Keccak256"
adkg_version = "v0.1"
adkg_scheme_name = "DXKR23-Bn254G1-Keccak256"
generator_g = "qB3/U8RDVn4aF2tUTlmeDQbV0PvHJ8IB0QL/k1Z+5WI="
generator_h = "gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE="
```

The `new-scheme` command can be used to obtain a scheme specification as follows:

```bash
adkg-cli new-scheme --app-name dcipher --scheme-out scheme.toml
```

By default, we use the `DXKR23-Bn254G1-Keccak256` scheme.
This represents the aforementioned ADKG, using the bn254 curve on group G1.
In this configuration, the long-term public keys use a deterministic generator, `H`, obtained by hashing
`ADKG_GENERATOR_G` with the DST `ADKG-%adkg_version%-%app_name%_BN254G1_XMD:KECCAK-256_SVDW_RO_GENERATORS_`
using [rfc9380](https://datatracker.ietf.org/doc/html/rfc9380).
The public key after the ADKG, however, are output with respect to the standard generator of bn254, the point `(1, 2)`.

It is preferable that a single participant executes the `new-scheme` command, and sends the generated file to the rest
of the participants.

## Generating long-term keys

With the scheme in hand, the next step consists of generating long-term keys that can be used to execute multiple
instances of the ADKG.
We require the generation of two keypairs, an elliptic curve keypair `(sk, [sk] H)`, and an ed25519 libp2p keypair.

The former is used for the ADKG, while the latter to authenticate and encrypt network communications.

The `generate` command is used as follows and requires a `scheme.toml` file:

```bash
adkg-cli generate --scheme scheme.toml --priv-out longterm.priv --pub-out longterm.pub
```

The command writes the private key material to the `longterm.priv` file, and the public key material to `longterm.pub`.
Only the public key material must be forwarded to the rest of the participants.
As explained above, `longterm.pub` contains two different values:

```toml
adkg_pk = "6xBXHRXOlmPcfV2LqeEKEDAOGOoAH2pIYBnuG/h1w8s="
peer_id = "12D3KooWRLzVJSS2EYpc9Tm4BfV5HEdXc8DiKvQkFqWhEwXcJ8eP"
```

Once the public key material of each of the participant has been gather, a group configuration in the following format
must be built and sent to the participants:

```toml
n = 7
t = 2
t_reconstruction = 4
start_time = "2025-07-24T16:25:30Z"

[[nodes]]
id = 1
multiaddr = "/ip4/127.0.0.1/tcp/9991"
adkg_pk = "71zT8Vib8vHG3vX2cktMyFl5VngXFi8RLOkIw9a5ulI="
peer_id = "12D3KooWMSH17hbmMBSbEtCeyYBkFT7phd6PVaA2fdxakaTAuXMx"

[[nodes]]
id = 2
multiaddr = "/dns4/my.domain.com/tcp/1005"
adkg_pk = "354LSCLd/rCf8bX/vN+nWNfO2G2ZoLs/v054IAgiDFk="
peer_id = "12D3KooWMAADWEWNMFCNBadQfNFLeHHcYeZr9tNJBekY4opdCzDM"

[...]

[[nodes]]
id = 7
multiaddr = "/ip4/127.0.0.1/tcp/7777"
adkg_pk = "7dpgwvtWiLAw/TweCrzBeRuWahRbxqMwACwiiulYkfA="
peer_id = "12D3KooWGjQdQ6B3LazUw2EVbhakN3P5931e1UV76vJUNoV73Dd4"
```

This file contains the group configuration, which includes the number of parties (`n`), a threshold (`t`), a
reconstruction threshold, and an agreed upon starting time, alongside a list of nodes.
The reconstruction threshold represents the number of signatures required to obtain a group signature.

Note that we use the malicious threshold here, i.e., the maximum number of parties that may be malicious.
The reconstruction threshold, i.e., the number of partials required to obtain a threshold signature, is obtained by
adding one.

Each node is specified by its unique identifier, its public key material, and a libp2p multiaddress that can be used to
communicate with the node.

Once this group file has been obtained and save, we can proceed to the final step.

## Executing the ADKG

Finally, to execute the ADKG, we must gather various piece of information:

- the scheme configuration file (`scheme.toml`)
- the long-term private key file (`longterm.priv`)
- the node's identifier (`1`)
- the group configuration (`group.toml`)
- the libp2p listen address (`/ip4/0.0.0.0/tcp/7777`)

With those details, we can use the `run` command as follows:

```bash
adkg-cli run                                \
  --scheme ./scheme.toml                    \
  --group ./group.toml                      \
  --priv longterm.priv                          \
  --id 1                                    \
  --listen-address "/ip4/0.0.0.0/tcp/7777"  \
  --transcript-out adkg.transcript          \
  --priv-out keyshare.priv             \
  --pub-out keyshare.pub 
```

Notice that we also include two output files used to store the private and public output of the ADKG.

We also include a third file, with `--transcript-out`, which is used to store an encrypted transcript of messages sent
during the ADKG.
The transcript should be sent to the other participants should a node fail to run the `ADKG` ceremony.

### Additional options

**Logging**\
The log level can be changed using the `LOG_LEVEL` env variable.
The specified level should follow the [
`EnvFilter`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives)
syntax.

**Metrics**\
A prometheus metrics endpoint may be enabled by adding the `--features metrics` flag when compiling the cli, and by
adding the `--metrics` flag to the `run` command.
This exports metrics on the `127.0.0.1:8080/metrics` endpoint by default.

## Rescue ADKG

In the case where a participant misses the ceremony, or fails to join due to network issues, the `rescue` command may be
used to recover the secret output.
Currently, this command only works if the shares of the participant running the `rescue` command were not used by the
other nodes.
In other words, if the node fails during the execution of the `ADKG`, recovering the secrets may not be possible in the
current version of the tool.

Before running the `rescue` command, we need to obtain the transcripts from the participants that successfully executed
the `ADKG`.
This should be at least `n - t` nodes.

The `rescue` command is run as follows:

```bash
adkg-cli rescue                             \
  --scheme ./scheme.toml                    \
  --group ./group.toml                      \
  --priv longterm.priv                      \
  --id 1                                    \
  --priv-out keyshare.priv                  \
  --pub-out keyshare.pub                    \
  ./transcripts/*
```

It mostly contains the same arguments as `run`, but also contains a positional argument used to specify the path to the
transcripts.
Here, we stored the `n - t` transcripts in the `./transcripts` folder.

## Transmogrify

The `transmogrify` command provides a small utility for converting public keys produced by the ADKG ceremony into other
encodings suitable for use in different contexts (e.g. Solidity contracts).

---

### Command-line Arguments

```text
Usage: adkg-cli transmogrify [OPTIONS] -p <PUBLIC_KEY> -i <INPUT_FORMAT> -o <OUTPUT_FORMAT>

Options:
  -p <PUBLIC_KEY>    A base64-encoded public key output from the ADKG ceremony
  -i <INPUT_FORMAT>  The expected input format of the key
  -o <OUTPUT_FORMAT> The desired output format
  -h, --help         Show usage
```

#### Input formats

| Variant | Meaning                                         |
|---------|-------------------------------------------------|
| `G2`    | A BN254 G2 point, base64-encoded and compressed |

#### Output formats

| Variant    | Meaning                                                                                 |
|------------|-----------------------------------------------------------------------------------------|
| `Solidity` | A hex-encoded, uncompressed point suitable for inclusion in on-chain Solidity contracts by using `g2Unmarshal` from our [bls-solidity](https://github.com/randa-mu/bls-solidity) library |

Both input and output formats are parsed case-insensitively, so you can pass `g2` or `G2`.

---

## Example

Suppose you have the following base64 public key from an ADKG ceremony:

```text
486nH+6k3O56JiJs684/W/p8ryAipRluAplIG1ARaRkc5bjdvnnsLUni9VimypUsKm5gfW6aXSQGywYQ3Tu2zQ==
```

Run the CLI:

```bash
adkg-cli transmogrify \
  -p "486nH+6k3O56JiJs684/W/p8ryAipRluAplIG1ARaRkc5bjdvnnsLUni9VimypUsKm5gfW6aXSQGywYQ3Tu2zQ==" \
  -i g2 \
  -o solidity
```

Output:

```text
23cea71feea4dcee7a26226cebce3f5bfa7caf2022a5196e0299481b501169191ce5b8ddbe79ec2d49e2f558a6ca952c2a6e607d6e9a5d2406cb0610dd3bb6cd2638f452d5d029d83e3cd216ffa983b38011646cbf96d2b30956b92ae5836c3913ae657a3c88cbd18ebc7d4b1e577b4faedc71f9ace79db7823b8efa20bbf3f3
```

This string is the Solidity-compatible uncompressed representation of the same public key.
