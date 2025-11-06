# Docker Buildx Bake file for building all bin binaries

variable "TAG" {
  default = "latest"
}

variable "REGISTRY" {
  default = ""
}

# Helper function to generate image name
function "image_name" {
  params = [name]
  result = REGISTRY != "" ? "${REGISTRY}/${name}:${TAG}" : "${name}:${TAG}"
}

# Target for metadata-action integration (if we use metadata-action in CI)
target "docker-metadata-action" {}

# Default group to build all targets
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

# Group for onlyswaps services only
group "onlyswaps" {
  targets = [
    "onlyswaps-smoketest",
    "onlyswaps-solver",
    "onlyswaps-state-api",
    "onlyswaps-verifier",
  ]
}

# Group for agent services
group "agents" {
  targets = [
    "blocklock-agent",
    "randomness-agent",
  ]
}

target "adkg-cli" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/adkg-cli/Dockerfile"
  tags       = [image_name("adkg-cli")]
  labels = {
    "org.opencontainers.image.title"       = "adkg-cli"
    "org.opencontainers.image.description" = "ADKG CLI tool"
  }
}

target "blocklock-agent" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/blocklock-agent/Dockerfile"
  tags       = [image_name("blocklock-agent")]
  labels = {
    "org.opencontainers.image.title"       = "blocklock-agent"
    "org.opencontainers.image.description" = "Blocklock Agent"
  }
}

target "monitoring" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/monitoring/Dockerfile"
  tags       = [image_name("monitoring")]
  labels = {
    "org.opencontainers.image.title"       = "monitoring"
    "org.opencontainers.image.description" = "Monitoring service"
  }
}

target "onlyswaps-smoketest" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/onlyswaps-smoketest/Dockerfile"
  tags       = [image_name("onlyswaps-smoketest")]
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-smoketest"
    "org.opencontainers.image.description" = "OnlySwaps Smoketest"
  }
}

target "onlyswaps-solver" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/onlyswaps-solver/Dockerfile"
  tags       = [image_name("onlyswaps-solver")]
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-solver"
    "org.opencontainers.image.description" = "OnlySwaps Solver"
  }
}

target "onlyswaps-state-api" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/onlyswaps-state-api/Dockerfile"
  tags       = [image_name("onlyswaps-state-api")]
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-state-api"
    "org.opencontainers.image.description" = "OnlySwaps State API"
  }
}

target "onlyswaps-verifier" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/onlyswaps-verifier/Dockerfile"
  tags       = [image_name("onlyswaps-verifier")]
  labels = {
    "org.opencontainers.image.title"       = "onlyswaps-verifier"
    "org.opencontainers.image.description" = "OnlySwaps Verifier"
  }
}

target "randomness-agent" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/randomness-agent/Dockerfile"
  tags       = [image_name("randomness-agent")]
  labels = {
    "org.opencontainers.image.title"       = "randomness-agent"
    "org.opencontainers.image.description" = "Randomness Agent"
  }
}

target "dsigner-legacy-http" {
  inherits   = ["docker-metadata-action"]
  context    = "."
  dockerfile = "bin/dsigner/examples/dsigner_legacy_http/Dockerfile"
  tags       = [image_name("dsigner-legacy-http")]
  labels = {
    "org.opencontainers.image.title"       = "dsigner-legacy-http"
    "org.opencontainers.image.description" = "DSigner Legacy HTTP"
  }
}

