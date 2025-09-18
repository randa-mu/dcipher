# dcipher test keys generator
A simple tool to generate dcipher test keys without running a DKG ceremony.

## Usage
The tool takes a number of parties (`-n`), a threshold (`-t`), as well as a supported scheme (`--scheme`).

Keys may be generated as follows:
```bash
> cargo run -p gen-keys -- -n 1 -t 1 --scheme Bn254
Generating bn254 keys:

node 1: bls private key    = 2f77ae6e2baba20bd0f384130b1026ea10c5f547e8d1f8dce60ccaee0a27ec74
node 1: bls public key g1  = 0y8dnQaEMw1S6xsYmUa/2mfQgOw0u9aw8mDxIoOgqic=
node 1: bls public key g2  = 0/u/Zf2gl3Cm4CXrPL7LXGzt73LFexS8gus6biXWP6wEvQxTX4P4D4FSfkn6DI2bvlIE1/Zn1hmqF0yC815pgw==
node 1: libp2p private key = CAESQAmtSWNPrb6QAX54jpqtIzneXZcjMNWfst2Gdo7Kttw/o3QUt7RyrSWD/4xnEHgkmbi2+ydGnFPo4fco3IiheG4=
node 1: libp2p peer id     = 12D3KooWLpRHYiWZi3hiNmF8usdFFjVFByLmAQm9jfHNckcLoYaR

group bls public key g1    = 0y8dnQaEMw1S6xsYmUa/2mfQgOw0u9aw8mDxIoOgqic=
group bls public key g2    = 0/u/Zf2gl3Cm4CXrPL7LXGzt73LFexS8gus6biXWP6wEvQxTX4P4D4FSfkn6DI2bvlIE1/Zn1hmqF0yC815pgw==
```

Both scheme-specific and bls keys are output.
