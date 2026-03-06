FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/pumpchain-validator /usr/local/bin/
COPY --from=builder /app/target/release/pumpchain-cli /usr/local/bin/

# Create data directory
RUN mkdir -p /root/.pumpchain

# Expose ports
EXPOSE 8899 8900 8001

# Default command
CMD ["pumpchain-validator", "--rpc-bind", "0.0.0.0:8899", "--p2p-listen", "/ip4/0.0.0.0/tcp/8001"]