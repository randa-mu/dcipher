# Common base dockerfile
# e.g. from root rune
# docker build --build-arg BINARY_PATH=target/release/monitoring --build-arg BINARY_NAME=monitoring -t monitoring .

ARG BINARY_PATH
ARG BINARY_NAME

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 dnsutils && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ${BINARY_PATH} /usr/local/bin/${BINARY_NAME}

ENTRYPOINT ["/bin/sh", "-c", "/usr/local/bin/${BINARY_NAME} \"$@\"", "--"]
