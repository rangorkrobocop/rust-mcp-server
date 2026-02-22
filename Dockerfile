# Build stage
FROM rust:slim-bookworm AS builder

# Install build dependencies for OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
RUN USER=root cargo new --bin rust-mcp-server
WORKDIR /rust-mcp-server

# Copy over our manifests
COPY ./Cargo.toml ./Cargo.lock ./

# This build step will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy our actual source code
COPY ./src ./src

# Build for release.
# We remove the cached dummy binary so cargo rebuilt it
RUN rm ./target/release/deps/todo_mcp* || true
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim
# Install ca-certificates since the server makes HTTPS requests to JSONPlaceholder
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /rust-mcp-server/target/release/todo_mcp /usr/local/bin/todo_mcp

# Set the start command
ENTRYPOINT ["todo_mcp"]
