# === Builder Stage: Build a static binary using musl ===
# We use the official musl builder image which compiles your application
# into a fully static binary that can run on a scratch image.
FROM rust:1.85-slim-bullseye AS builder

# Set working directory inside the container
WORKDIR /usr/src/app


# Install build dependencies
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

# Copy the Cargo files first to leverage caching
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Will build and cache the binary and dependent crates in release mode
RUN cargo build --release && mv ./target/release/echo_server ./echo_server

# === Final Stage: Minimal runtime image ===
# We use the scratch image (i.e. an empty image) for the smallest possible binary.
FROM debian:bullseye-slim AS runner

# Install runtime dependencies 
RUN apt-get update && apt-get install -y libpq-dev 

# Set Environment Variables
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ARG SPOTIFY_CLIENT_ID
ENV SPOTIFY_CLIENT_ID=${SPOTIFY_CLIENT_ID}
ARG SPOTIFY_CLIENT_SECRET
ENV SPOTIFY_CLIENT_SECRET=${SPOTIFY_CLIENT_SECRET}
ARG SPOTIFY_REDIRECT_URI
ENV SPOTIFY_REDIRECT_URI=${SPOTIFY_REDIRECT_URI}
ARG HASH_KEY
ENV HASH_KEY=${HASH_KEY}
ARG BIND_ADDRESS
ENV BIND_ADDRESS=${BIND_ADDRESS}
ARG SESSION_COOKIE_KEY
ENV SESSION_COOKIE_KEY=${SESSION_COOKIE_KEY}
ARG CLIENT_URL
ENV CLIENT_URL=${CLIENT_URL}
ARG ENVIRONMENT
ENV ENVIRONMENT=${ENVIRONMENT}
ARG DOMAIN
ENV DOMAIN=${DOMAIN}

# Expose the port your application listens on (e.g. 8080)
EXPOSE 8080

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/echo_server /app/echo_server

# Run the app
CMD [ "./echo_server", "/app/" ]
