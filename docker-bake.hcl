# Docker Buildx Bake file for building all bin binaries

variable "TAG" {
  default = "latest"
}

variable "SHA" {
  default = ""
}

variable "REGISTRY" {
  default = ""
}

# Sanitize TAG by replacing / with - (branch names like "docker/bake" become "docker-bake")
sanitized_tag = replace(TAG, "/", "-")

# Helper function to generate image tags
function "image_tags" {
  params = [name]
  result = REGISTRY != "" ? (
    SHA != "" ? [
      "${REGISTRY}/${name}:${sanitized_tag}",
      "${REGISTRY}/${name}:${substr(SHA, 0, 7)}"
    ] : [
      "${REGISTRY}/${name}:${sanitized_tag}"
    ]
  ) : (
    SHA != "" ? [
      "${name}:${sanitized_tag}",
      "${name}:${SHA}",
      "${name}:${substr(SHA, 0, 7)}"
    ] : [
      "${name}:${sanitized_tag}"
    ]
  )
}

target "docker-metadata-action" {}

# --- CI CACHE TARGETS ---

target "rust-base-internal" {
  context    = "."
  dockerfile = "bin/Dockerfile.base"
  target     = "rust-base-internal"
}

# --- COMMON CONFIGURATION ---

target "rust-binary-common" {
  inherits = ["docker-metadata-action"]
  context  = "."
  # This creates a proper dependency: binary targets wait for rust-base-internal to build first
  contexts = {
    rust-base-internal = "target:rust-base-internal"
  }
  args = {
    base_stage_alias = "rust-base-internal"
  }
}

# --- BINARY TARGETS ---

target "adkg-cli" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/adkg-cli/Dockerfile"
  tags       = image_tags("adkg-cli")
  labels = {
    "org.opencontainers.image.title"       = "adkg-cli"
    "org.opencontainers.image.description" = "ADKG CLI tool"
  }
}

target "blocklock-agent" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/blocklock-agent/Dockerfile"
  tags       = image_tags("blocklock-agent")
  labels = {
    "org.opencontainers.image.title"       = "blocklock-agent"
    "org.opencontainers.image.description" = "Blocklock Agent"
  }
}

target "monitoring" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/monitoring/Dockerfile"
  tags       = image_tags("monitoring")
  labels = {
    "org.opencontainers.image.title"       = "monitoring"
    "org.opencontainers.image.description" = "Monitoring service"
  }
}

target "onlyswaps-smoketest" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/onlyswaps-smoketest/Dockerfile"
  tags       = image_tags("onlyswaps-smoketest")
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-smoketest"
    "org.opencontainers.image.description" = "OnlySwaps Smoketest"
  }
}

target "onlyswaps-solver" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/onlyswaps-solver/Dockerfile"
  tags       = image_tags("onlyswaps-solver")
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-solver"
    "org.opencontainers.image.description" = "OnlySwaps Solver"
  }
}

target "onlyswaps-state-api" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/onlyswaps-state-api/Dockerfile"
  tags       = image_tags("onlyswaps-state-api")
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-state-api"
    "org.opencontainers.image.description" = "OnlySwaps State API"
  }
}

target "onlyswaps-verifier" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/onlyswaps-verifier/Dockerfile"
  tags       = image_tags("onlyswaps-verifier")
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-verifier"
    "org.opencontainers.image.description" = "OnlySwaps Verifier"
  }
}

target "randomness-agent" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/randomness-agent/Dockerfile"
  tags       = image_tags("randomness-agent")
  labels = {
    "org.opencontainers.image.title"       = "randomness-agent"
    "org.opencontainers.image.description" = "Randomness Agent"
  }
}

target "dsigner-legacy-http" {
  inherits   = ["rust-binary-common"]
  dockerfile = "bin/dsigner/examples/dsigner_legacy_http/Dockerfile"
  tags       = image_tags("dsigner-legacy-http")
  labels = {
    "org.opencontainers.image.title"       = "dsigner-legacy-http"
    "org.opencontainers.image.description" = "DSigner Legacy HTTP"
  }
}

group "default" {
  targets = [
    "adkg-cli",
    "blocklock-agent",
    "monitoring",
    "onlyswaps-smoketest",
    "onlyswaps-solver",
    "onlyswaps-state-api",
    "onlyswaps-verifier",
    "randomness-agent",
    "dsigner-legacy-http",
  ]
}