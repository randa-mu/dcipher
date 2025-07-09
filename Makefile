.PHONY= git solidity-deps build-forge-all build-forge-all-concurrent clean-node-modules clean-solidity-out $(addprefix run_,$(DIRS))
DCIPHER_MODULE_DIRS := onlyswaps-verifier dsigner
SOLIDITY_DIRS := $(wildcard *-solidity/)

git:
	@git submodule update --init --recursive --verbose --progress -j 8

install_solidity_node_deps:
	@npm install

build_forge_all:
	@npm run build:forge --ws

build_forge_all_concurrent:
	@for dir in $(SOLIDITY_DIRS); do \
		(cd $$dir && npm run build:forge) & \
	done; \
	wait

build_cargo: deps
	cargo build

# If you want to pass args run
# make run_onlyswaps-verifier ARGS="-v"
run_%: deps
	cargo run --bin $* -- $(ARGS)

clean_node_modules:
	@find . -name 'node_modules' -type d -prune -exec rm -rf '{}' +

clean_solidity_out:
	@for dir in $(SOLIDITY_DIRS); do \
		(cd $$dir && find . -name 'out' -type d -prune -exec rm -rf '{}' +) \
	done;

clean: clean_node_modules clean_solidity_out
deps: install_solidity_node_deps build_forge_all_concurrent
all: git install_solidity_node_deps build_forge_all_concurrent