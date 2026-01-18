# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/sui-invariant-monitor /usr/local/bin/sui-invariant-monitor

# Make binary executable
RUN chmod +x /usr/local/bin/sui-invariant-monitor

# Create non-root user
RUN useradd -m -u 1000 monitor
USER monitor

WORKDIR /home/monitor

EXPOSE 8080

# Use full path and add error logging
CMD ["/usr/local/bin/sui-invariant-monitor"]
