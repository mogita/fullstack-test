FROM rust:1.83-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
RUN USER=root cargo new --bin simple-fullstack-backend
WORKDIR /app/simple-fullstack-backend

# Copy over manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build dependencies - this is the caching Docker layer!
# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo 'fn main() { println!("Dummy build") }' > src/main.rs
RUN cargo build --release
RUN rm src/*.rs

# Copy actual source code
COPY ./src ./src

# Force a rebuild with the actual source code
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

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

# Expose the port
EXPOSE 3001

# Run the binary
CMD ["/app/simple-fullstack-backend"]