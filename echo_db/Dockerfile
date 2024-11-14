FROM rust:alpine3.20 as build

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev && \
    cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]
