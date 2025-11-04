# Common base dockerfile
# e.g. from root rune
# docker build --build-arg BINARY_PATH=target/release/monitoring --build-arg BINARY_NAME=monitoring -t monitoring .

ARG BINARY_PATH
ARG BINARY_NAME

FROM debian:bookworm-slim AS runtime

ARG BINARY_PATH
ARG BINARY_NAME

RUN apt-get update && apt-get install -y --no-install-recommends libssl3 dnsutils && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --chmod=0755 ${BINARY_PATH} /usr/local/bin/${BINARY_NAME}

ENTRYPOINT ["/bin/sh", "-c", "/usr/local/bin/${BINARY_NAME} \"$@\"", "--"]
