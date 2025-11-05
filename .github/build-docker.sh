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
# - $PARALLEL_BUILDS: true|false defaults to true

CONFIG_FILE=".github/docker-config.json"
PARALLEL_BUILDS=${PARALLEL_BUILDS:-"true"}

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
echo "Parallel builds: $PARALLEL_BUILDS"

# Function to build a single image
build_image() {
  local image="$1"
  local prefix="$2"

  BINARY_NAME=$(echo "$image" | jq -r '.binary_name')
  BINARY_PATH=$(echo "$image" | jq -r '.binary_path')
  IMAGE_NAME=$(echo "$image" | jq -r '.image_name')
  DESCRIPTION=$(echo "$image" | jq -r '.description')

  echo ""
  echo "[$prefix] ========================================"
  echo "[$prefix] Building image: $IMAGE_NAME"
  echo "[$prefix] Binary: $BINARY_NAME ($BINARY_PATH)"
  echo "[$prefix] Description: $DESCRIPTION"
  echo "[$prefix] ========================================"

  FULL_IMAGE_NAME="${DOCKER_REGISTRY}/$IMAGE_NAME:$VERSION_TAG"
  CACHE_REF="${DOCKER_REGISTRY}/$IMAGE_NAME-cache:$VERSION_TAG"
  CACHE_REF_MAIN="${DOCKER_REGISTRY}/$IMAGE_NAME-cache:main"

  # Build arguments (using array to properly handle spaces)
  BUILD_ARGS=()
  BUILD_ARGS+=(--build-arg "BINARY_PATH=$BINARY_PATH")
  BUILD_ARGS+=(--build-arg "BINARY_NAME=$BINARY_NAME")
  BUILD_ARGS+=(--label "maintainer=$IMAGE_MAINTAINER")
  BUILD_ARGS+=(--label "org.opencontainers.image.vendor=$IMAGE_VENDOR")
  BUILD_ARGS+=(--label "org.opencontainers.image.title=$IMAGE_NAME")
  BUILD_ARGS+=(--label "org.opencontainers.image.description=$DESCRIPTION")
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

  # Build and optionally push (prefix all output)
  echo "[$prefix] Running: docker buildx build ${BUILD_ARGS[@]} ."
  if docker buildx build "${BUILD_ARGS[@]}" "${DOCKER_CONTEXT}" 2>&1 | sed -E "s#^#[$prefix/$BINARY_NAME] #"; then
    echo "[$prefix] ✓ Successfully built $IMAGE_NAME"
    return 0
  else
    echo "[$prefix] ✗ Failed to build $IMAGE_NAME" >&2
    return 1
  fi
}

# Convert images to array
readarray -t IMAGE_ARRAY < <(echo "$IMAGES" | jq -c '.[]')
IMAGE_COUNT=${#IMAGE_ARRAY[@]}

if [[ $IMAGE_COUNT -eq 0 ]]; then
  echo "No images to build"
  exit 0
fi

# Build first image sequentially to warm cache
echo ""
echo "Building first image to warm cache..."
build_image "${IMAGE_ARRAY[0]}" "1/$IMAGE_COUNT"
FIRST_BUILD_STATUS=$?

if [[ $FIRST_BUILD_STATUS -ne 0 ]]; then
  echo "Failed to build first image, aborting"
  exit 1
fi

# Build remaining images
if [[ $IMAGE_COUNT -le  1 ]]; then
  echo ""
  echo "========================================"
  echo "All images built successfully!"
  echo "========================================"
  exit 0;
fi

if [[ "$PARALLEL_BUILDS" == "true" ]]; then
  echo ""
  echo "Building remaining $((IMAGE_COUNT - 1)) images in parallel..."

  PIDS=()
  FAILED_BUILDS=()

  for i in $(seq 1 $((IMAGE_COUNT - 1))); do
    (
      build_image "${IMAGE_ARRAY[$i]}" "$((i + 1))/$IMAGE_COUNT"
      exit $?
    ) &
    PIDS+=($!)
  done

  # Wait for all background jobs and collect failures
  for i in "${!PIDS[@]}"; do
    pid=${PIDS[$i]}
    if ! wait $pid; then
      image_num=$((i + 2))
      FAILED_BUILDS+=("Image $image_num/$IMAGE_COUNT")
    fi
  done

  if [[ ${#FAILED_BUILDS[@]} -gt 0 ]]; then
    echo ""
    echo "✗ Some builds failed:"
    printf '  - %s\n' "${FAILED_BUILDS[@]}"
    exit 1
  fi
else
  echo ""
  echo "Building remaining $((IMAGE_COUNT - 1)) images sequentially..."

  for i in $(seq 1 $((IMAGE_COUNT - 1))); do
    build_image "${IMAGE_ARRAY[$i]}" "$((i + 1))/$IMAGE_COUNT"
    if [[ $? -ne 0 ]]; then
      echo "Failed to build image $((i + 1))/$IMAGE_COUNT, aborting"
      exit 1
    fi
  done
fi

echo ""
echo "========================================"
echo "All images built successfully!"
echo "========================================"