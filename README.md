# dcipher

## Prerequisites
- foundry 1.3.5-stable
- rust 1.89.0+
 
## Building

### To run it with minimum effort
```bash
make git run_dsigner
```
or
```bash
make git run_onlyswaps-verifier ARGS="--port=8080"
```

## To just build without running
Init submodules, fetch npm deps, and build forge contracts with:
```bash
make all
```
You'll get binaries produced in ./target/debug/


## Less automagic process
If you prefer a granular process, you can run:
```bash
make clean                      # Remove node_modules
make git
make install_solidity_node_dep
# Either:
make build_forge_all_parallel   # Parallel build
# OR
make build_forge_all            # Linear build
make build_cargo
```

### Cleaning
Remove all node_modules and forge outputs with:
```bash
make clean
```

just node_modules:
```bash
make clean_node_modules
```

just forge outputs:
```bash
make clean_forge
```

## Bindings
If any of the solidity code is updated, you must generate new Rust bindings before committing. 
This can be done by running `./generate-bindings.sh`.
Any mismatch in the generated bindings and those in the repo will cause a build failure.