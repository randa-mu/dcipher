# Docker Build Strategy

This directory contains the Docker build configuration for the dcipher project, implementing a multi-stage caching strategy optimized for CI/CD pipelines.

## Overview

The Docker setup is designed around two key principles:

1. **Split Dockerfiles for CI/CD**: Separate build stages that can be cached independently
2. **Full Dockerfile for Standalone Builds**: A complete Dockerfile that can build from scratch while benefiting from existing cached layers

## Architecture

### Core Components

#### 1. `Dockerfile.rust-deps-builder`
- **Purpose**: Builds and caches Rust dependencies using `cargo-chef`
- **Key Feature**: Creates a cached layer containing all compiled dependencies
- **Usage**: Run first in CI to establish the dependency cache layer
- **Output**: Intermediate image with compiled Rust dependencies

```dockerfile
# Builds dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
```

#### 2. `Dockerfile.sol-builder`
- **Purpose**: Builds Solidity contracts using Foundry
- **Key Feature**: Compiles smart contracts and caches the build artifacts
- **Parameters**: 
  - `SOLIDITY_PROJECT_DIR`: Which Solidity project to build (e.g., "blocklock-solidity")
- **Output**: Intermediate image with compiled Solidity contracts

#### 3. `Dockerfile.rust-app`
- **Purpose**: Complete standalone Dockerfile for building any Rust application
- **Key Feature**: Can build independently OR reuse cached layers from previous stages
- **Parameters**:
  - `APPLICATION`: Name of the Rust application to build
  - `SOLIDITY_PROJECT_DIR`: Which Solidity contracts to include
- **Strategy**: Duplicates the dependency building steps so it can work standalone, but benefits from cached layers when available

### Build Strategy Benefits

#### For CI/CD Pipelines:
1. **Dependency Caching**: Rust dependencies are built once and reused across all applications
2. **Solidity Caching**: Smart contracts are compiled once per project and reused
3. **Parallel Builds**: Multiple applications can build concurrently using shared cached layers
4. **Incremental Updates**: Only rebuild what actually changed

#### For Local Development:
1. **Standalone Capability**: Each application can be built independently
2. **Layer Reuse**: Automatically benefits from any existing cached layers
3. **Fast Rebuilds**: Dependency changes only require rebuilding the dependency layer

## Docker Bake Configuration

The `docker-bake.hcl` file orchestrates the build process:

### Base Targets
- `_base-config`: Common configuration (context, etc.)
- `_solidity-builder`: Base for Solidity builds
- `rust-deps-builder`: Rust dependency cache builder
- `rust-app`: Generic Rust application builder

### Application Targets
- `blocklock-weft`: Blocklock application with blocklock-solidity contracts
- `randomness-weft`: Randomness application with randomness-solidity contracts
- Additional applications can be added following the same pattern

### Build Groups
- `base-configs`: Core infrastructure images
- `builders`: Solidity contract builders
- `dcipher-apps`: All application images
- `blocklock`: Complete blocklock stack

## Usage Examples

### CI/CD Pipeline (Optimal Caching)
```bash
# Step 1: Build shared dependency cache
docker buildx bake rust-deps-builder

# Step 2: Build Solidity contracts in parallel
docker buildx bake builders

# Step 3: Build all applications (reusing cached layers)
docker buildx bake dcipher-apps
```

### Local Development (Standalone)
```bash
# Build a single application (will reuse any existing cached layers)
docker buildx bake blocklock-weft

# Build everything from scratch
docker buildx bake
```

### Custom Application Build
```bash
# Build with custom parameters
docker buildx bake rust-app --set rust-app.args.APPLICATION=my-app --set rust-app.args.SOLIDITY_PROJECT_DIR=my-contracts
```

## Cache Strategy

### Layer Hierarchy
1. **Base OS + Tools**: Node.js, Rust, Foundry (rarely changes)
2. **Package Dependencies**: npm packages, Cargo dependencies (changes occasionally)
3. **Contract Compilation**: Solidity builds (changes with contract updates)
4. **Application Code**: Rust application builds (changes frequently)

### Cache Invalidation
- **Rust Dependencies**: Invalidated when `Cargo.toml` or `Cargo.lock` changes
- **Solidity Contracts**: Invalidated when contract source files change
- **Application Code**: Invalidated when application source changes

## Adding New Applications

To add a new application:

1. **Create application Dockerfile** in the crate directory (following existing patterns)
2. **Add target to docker-bake.hcl**:
   ```hcl
   target "my-new-app" {
     inherits = ["rust-app"]
     tags = [
       "${DOCKER_REGISTRY}/my-new-app:${IMAGE_TAG}"
     ]
     args = {
       APPLICATION = "my-new-app"
       SOLIDITY_PROJECT_DIR = "relevant-solidity-project"
     }
   }
   ```
3. **Update build groups** as needed

## Performance Considerations

- **Cold Build**: ~10-15 minutes (compiling all dependencies)
- **Warm Build**: ~2-3 minutes (reusing cached layers)
- **Code-only Changes**: ~30-60 seconds (only rebuilding application layer)
- **Parallel Builds**: Multiple applications build concurrently after shared stages complete

## Troubleshooting

### Cache Issues
- Use `docker buildx prune` to clear build cache if needed
- Check layer reuse with `docker buildx bake --progress=plain`

### Build Failures
- Ensure you're building from the repository root
- Verify all required build arguments are provided
- Check that Solidity contracts compile successfully

## Future Improvements

- **Matrix Builds**: The bakefile is being enhanced to use matrix builds for automatic target generation
- **Multi-arch Support**: Planned support for ARM64 and AMD64 architectures
- **Registry Optimization**: Enhanced layer sharing across different applications

