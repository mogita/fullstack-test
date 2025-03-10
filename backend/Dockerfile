FROM rust:1.70-slim as builder

WORKDIR /app

# Create a new empty shell project
RUN USER=root cargo new --bin simple-fullstack-backend
WORKDIR /app/simple-fullstack-backend

# Copy over manifests
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release
RUN rm src/*.rs

# Copy actual source code
COPY ./src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install OpenSSL - required for many Rust applications
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/simple-fullstack-backend/target/release/simple-fullstack-backend /app/

# Create a non-root user to run the application
RUN useradd -m appuser
USER appuser

# Set environment variables
ENV SERVER_PORT=3001
ENV SERVER_HOST=0.0.0.0

# Expose the port
EXPOSE 3001

# Run the binary
CMD ["/app/simple-fullstack-backend"]