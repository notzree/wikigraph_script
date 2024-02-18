FROM rust:1.75.0

WORKDIR /wikigraph

RUN cargo install diesel_cli --no-default-features --features postgres