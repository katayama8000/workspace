FROM rust:1.76.0-bookworm

RUN apt-get update

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-update