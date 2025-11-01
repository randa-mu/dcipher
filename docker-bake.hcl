variable "DOCKER_REGISTRY" {
  default = "europe-west1-docker.pkg.dev/randamu-prod/candyland"
}

variable "IMAGE_TAG" {
  default = "latest"
}

variable "IMAGE_MAINTAINER" {
  default = "Randamu"
}

variable "IMAGE_VENDOR" {
  default = "Randamu"
}

# Base target that will be inherited by all binary targets
# Actual targets are defined in docker-bake.override.json
target "default" {
  context = "."
  dockerfile = "Dockerfile"
  labels = {
    "maintainer" = IMAGE_MAINTAINER
    "org.opencontainers.image.vendor" = IMAGE_VENDOR
  }
}