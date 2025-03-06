FROM rust:1.84 AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim AS run

RUN apt-get update && apt-get autoclean && apt-get autoremove 

COPY --from=build /app/target/release/echo /app/server

CMD ["app/server"]

