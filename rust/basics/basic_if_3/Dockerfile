# Stage 1
FROM rust:1.93.1-slim-trixie AS builder
LABEL authors="jabbott-iii"

WORKDIR /app

# Compilation depenencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# Build dependency data
COPY Cargo.toml Cargo.lock ./
# cache cargo builds
RUN mkdir -p src && \
    printf "fn main() { println!(\"dep cache\"); }\n" > src/main.rs && \
    cargo build --release && \
    rm -rf src
# Copy source and build
COPY . .
RUN cargo build --release

# Stage 2
FROM debian:trixie-slim

WORKDIR /app

# Certs for HTTPS / Can remove if not needed
# RUN apt-get update && apt-get install -y --no-install-recommends \
#    ca-certificates \
# && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/basic_if_3 /usr/local/bin/basic_if_3

# EXPOSE 8080

CMD ["basic_if_3"]