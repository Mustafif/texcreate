FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest
FROM rust:latest 

RUN  && \
    apt-get update && \
    apt-get install -y libssl-dev:arm64