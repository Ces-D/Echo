FROM rust:1.84 AS build

WORKDIR /app

COPY . .

## TODO: Fix the following error
# [server build 4/4] RUN cargo build --release:
# 0.483 error: failed to parse lock file at: /app/Cargo.lock
# 0.483
# 0.483 Caused by:
# 0.483   lock file version `4` was found, but this version of Cargo does not understand this lock file, perhaps Cargo needs to be updated?
RUN cargo build --release

FROM debian:bullseye-slim AS run

RUN apt-get update && apt-get autoclean && apt-get autoremove 

COPY --from=build /app/target/release/echo /app/server

CMD ["app/server"]

