# dcipher

## Building

### To run it with minimum effort
```bash
make run_dsigner
``
or
```bash
make run_onlyswaps-verifier ARGS="--port=8080"
```


Init submodules, fetch npm deps, and build forge contracts with:

```bash
make all
```

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
