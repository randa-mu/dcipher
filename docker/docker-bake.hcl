variable "DOCKER_REGISTRY" {
  default = "europe-west1-docker.pkg.dev/randamu-prod/candyland"
}

variable "NODE_VERSION" {
  default = "22.3"
}

variable "IMAGE_TAG" {
  default = "latest"
}

target "_base-config" {
  context = "../"
}

target "_solidity-builder" {
  inherits = ["_base-config"]
  dockerfile = "./docker/Dockerfile.sol-builder"
}


target "rust-deps-builder" {
  inherits = ["_base-config", ]
  dockerfile = "./docker/Dockerfile.rust-deps-builder"
}

target "rust-app" {
  inherits = ["_base-config"]
  dockerfile = "./docker/Dockerfile.rust-app"
}

target "blocklock-solidity-builder" {
  inherits = ["_solidity-builder"]
  tags = [
    "${DOCKER_REGISTRY}/blocklock-solidity-builder:${IMAGE_TAG}"
  ]
  args = {
    SOLIDITY_PROJECT_DIR = "blocklock-solidity"
  }
}

target "blocklock-weft" {
  inherits = ["rust-app"]
  tags = [
    "${DOCKER_REGISTRY}/blocklock-weft:${IMAGE_TAG}"
  ]
  args = {
    APPLICATION = "blocklock-weft"
    SOLIDITY_PROJECT_DIR = "blocklock-solidity"
  }
}

target "randomness-solidity-builder" {
  inherits = ["_solidity-builder"]
  tags = [
    "${DOCKER_REGISTRY}/randomness-solidity-builder:${IMAGE_TAG}"
  ]
  args = {
    SOLIDITY_PROJECT_DIR = "randomness-solidity"
  }
}


target "randomness-weft" {
  inherits = ["rust-app"]
  tags = [
    "${DOCKER_REGISTRY}/randomness-weft:${IMAGE_TAG}"
  ]
  args = {
    APPLICATION = "randomness-weft"
    SOLIDITY_PROJECT_DIR = "randomness-solidity"
  }
}


group "base-configs" {
  targets = ["_base-config", "_solidity-builder", "rust-deps-builder", "rust-app"]
}

group "builders" {
  targets = ["blocklock-solidity-builder", "randomness-solidity-builder"]
}

group "dcipher-apps" {
  targets = ["blocklock-weft", "randomness-weft"]
}

group "blocklock" {
  targets = ["rust-deps-builder", "blocklock-solidity-builder", "blocklock-weft"]
}