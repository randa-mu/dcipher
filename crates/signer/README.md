# dcipher-signer

This library abstracts over threshold signature schemes.
It allows other crates to be generic over the choice of algorithm, group and hash function, as well as swapping groups in the case of BLS.
It also implements a generic signing microservice used by agents.

Currently, only BLS is supported.
The crate is most useful if at least one of `bn254` or `bls12_381`, and at least one of `sha2` or `sha3` are enabled.
Note that `sha3` enable the Ethereum variant of keccak256, using the `sha3` crate.
The `rayon` feature enables basic parallelism when signing multiple messages in the microservice.
