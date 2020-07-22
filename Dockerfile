ARG BASE_IMAGE=rust:latest

# Declare the build environment
FROM ${BASE_IMAGE} AS builder

WORKDIR /usr/src/twitter-bot

COPY . .

RUN cargo build --release

RUN cargo install --path .

# CMD ["/usr/local/cargo/bin/twitter-bot"]
CMD cargo run
