FROM --platform=linux/amd64 ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes \
    libssl-dev:arm64