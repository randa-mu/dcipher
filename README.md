# dcipher

Currently, there is no automatic dependency management. To compile `blocklock-agent`, you first have to pull the submodules with
```bash
git submodule update --init --recursive
```

Then, go to `blocklock-solidity` to compile the contracts with
```
cd blocklock-solidity
npm install
npm run build:forge
```

This should be enough to compile the `blocklock-agent` successfully.