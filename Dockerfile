FROM rust:1.75.0

WORKDIR /wikigraph_script

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt-get update && \
    apt-get install -y postgresql-client && \
    rm -rf /var/lib/apt/lists/*