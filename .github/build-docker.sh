#!/bin/bash
set -e

# Required environment variables:
# - GITHUB_REF: Git reference (e.g., refs/heads/main, refs/tags/v1.0.0)
# - GITHUB_SHA: Git commit SHA
# - PR_NUMBER: Pull request number (if applicable)
# - TEST_MODE: Whether test mode is enabled
# - DOCKER_REGISTRY: Docker registry URL
# - IMAGE_MAINTAINER: Image maintainer label
# - IMAGE_VENDOR: Image vendor label

CONFIG_FILE=".github/docker-config.json"

# Load config if exists
if [ -f "$CONFIG_FILE" ]; then
  CONFIG=$(cat "$CONFIG_FILE")
else
  CONFIG='{}'
fi

# Generate image list from binaries.json
IMAGES=$(jq -sc --argjson config "$CONFIG" 'map({
  binary_name: .name,
  binary_path: .path,
  image_name: ($config[.name].image_name // .name),
  description: ($config[.name].description // "Dcipher service")
})' binaries.json)

echo "Building images for the following binaries:"
echo "$IMAGES" | jq .

# Generate version tag based on git ref
if [[ "$GITHUB_REF" == "refs/heads/main" ]]; then
  VERSION_TAG="main-latest"
elif [[ "$GITHUB_REF" == refs/tags/* ]]; then
  VERSION_TAG="${GITHUB_REF#refs/tags/}"
elif [[ "$GITHUB_REF" == refs/pull/* ]]; then
  VERSION_TAG="pr-${PR_NUMBER}"
else
  VERSION_TAG="${GITHUB_REF#refs/heads/}"
fi

# Determine if we should push
SHOULD_PUSH="false"
if [[ "$GITHUB_REF" == "refs/heads/main" ]] || [[ "$GITHUB_REF" == refs/tags/* ]] || [[ "$TEST_MODE" == "true" ]]; then
  SHOULD_PUSH="true"
fi

echo "Version tag: $VERSION_TAG"
echo "Should push: $SHOULD_PUSH"

# Loop through each image and build
echo "$IMAGES" | jq -c '.[]' | while read -r image; do
  BINARY_NAME=$(echo "$image" | jq -r '.binary_name')
  BINARY_PATH=$(echo "$image" | jq -r '.binary_path')
  IMAGE_NAME=$(echo "$image" | jq -r '.image_name')
  DESCRIPTION=$(echo "$image" | jq -r '.description')

  echo ""
  echo "========================================"
  echo "Building image: $IMAGE_NAME"
  echo "Binary: $BINARY_NAME ($BINARY_PATH)"
  echo "Description: $DESCRIPTION"
  echo "========================================"

  FULL_IMAGE_NAME="${DOCKER_REGISTRY}/$IMAGE_NAME:$VERSION_TAG"
  CACHE_REF="${DOCKER_REGISTRY}/$IMAGE_NAME-cache:$VERSION_TAG"
  CACHE_REF_MAIN="${DOCKER_REGISTRY}/$IMAGE_NAME-cache:main"

  # Build arguments (using array to properly handle spaces)
  BUILD_ARGS=()
  BUILD_ARGS+=(--build-arg 'BINARY_PATH='"$BINARY_PATH")
  BUILD_ARGS+=(--build-arg 'BINARY_NAME='"$BINARY_NAME")
  BUILD_ARGS+=(--label 'maintainer='"$IMAGE_MAINTAINER")
  BUILD_ARGS+=(--label 'org.opencontainers.image.vendor='"$IMAGE_VENDOR")
  BUILD_ARGS+=(--label 'org.opencontainers.image.title='"$IMAGE_NAME")
  BUILD_ARGS+=(--label 'org.opencontainers.image.description='"$DESCRIPTION")
  BUILD_ARGS+=(--label "org.opencontainers.image.version=$VERSION_TAG")
  BUILD_ARGS+=(--label "org.opencontainers.image.revision=$GITHUB_SHA")
  BUILD_ARGS+=(--cache-from "type=registry,ref=$CACHE_REF")
  BUILD_ARGS+=(--cache-from "type=registry,ref=$CACHE_REF_MAIN")
  BUILD_ARGS+=(--cache-to "type=registry,ref=$CACHE_REF,mode=max")
  BUILD_ARGS+=(-t "$FULL_IMAGE_NAME")
  BUILD_ARGS+=(-f ./Dockerfile)

  if [[ "$SHOULD_PUSH" == "true" ]]; then
    BUILD_ARGS+=(--push)
  else
    BUILD_ARGS+=(--load)
  fi

  set -x
  # Build and optionally push
  echo "Running: docker buildx build ${BUILD_ARGS[@]} ."
  docker buildx build "${BUILD_ARGS[@]}" .

  echo "✓ Successfully built $IMAGE_NAME"
done

echo ""
echo "========================================"
echo "All images built successfully!"
echo "========================================"