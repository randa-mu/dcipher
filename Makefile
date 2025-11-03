.PHONY= git solidity-deps build-forge-all build-forge-all-concurrent clean-node-modules clean-solidity-out clean_dcipher $(addprefix run_,$(DIRS))
DCIPHER_MODULE_DIRS := onlyswaps-verifier dsigner
SOLIDITY_DIRS := $(wildcard modules/*-solidity/)

git:
	@git submodule update --init --recursive --verbose --progress -j 8

install_solidity_node_deps:
	@npm install

generate_rust_bindings:
	./generate-bindings.sh;

build_forge_all:
	@echo $(SOLIDITY_DIRS)
	@for dir in $(SOLIDITY_DIRS); do \
		(cd $$dir && npm run build:forge) \
	done; \
	wait
build_forge_all_concurrent:
	@for dir in $(SOLIDITY_DIRS); do \
		(cd $$dir && npm run build:forge) & \
	done; \
	wait

build_cargo: deps
	cargo build

build_binaries: deps
	cargo build --release --workspace --bins --examples

# Helper to determine binary path for Docker builds
# Usage: make build_docker_randomness-agent
build_docker_%: build_binaries
	@if [ -f "target/release/$*" ]; then \
		BINARY_PATH="target/release/$*"; \
		BINARY_NAME="$*"; \
	elif [ -f "target/release/examples/$*" ]; then \
		BINARY_PATH="target/release/examples/$*"; \
		BINARY_NAME="$*"; \
	else \
		echo "Error: Binary not found for $*"; \
		exit 1; \
	fi; \
	docker build \
		--build-arg BINARY_PATH=$$BINARY_PATH \
		--build-arg BINARY_NAME=$$BINARY_NAME \
		-t dcipher-$* \
		-f Dockerfile .

# If you want to pass args run
# make run_onlyswaps-verifier ARGS="-v"
run_%: deps
	cargo run --bin $* -- $(ARGS)

clean_dcipher:
	@rm -rf ./target

clean_generated:
	@rm -rf ./crates/generated/src/blocklock;
	@rm -rf ./crates/generated/src/randomness;
	@rm -rf ./crates/generated/src/onlyswaps;

clean_solidity_out:
	@for dir in $(SOLIDITY_DIRS); do \
		(cd $$dir && find . -name 'out' -type d -prune -exec rm -rf '{}' +) \
	done;

clean_node_modules:
	@find . -name node_modules -maxdepth 2 -exec rm -rf {} \;

clean: clean_node_modules clean_solidity_out clean_dcipher clean_generated
deps: install_solidity_node_deps build_forge_all generate_rust_bindings
all: git deps build_cargo
