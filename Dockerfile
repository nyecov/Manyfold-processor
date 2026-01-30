# Builder stage
FROM --platform=$BUILDPLATFORM rust:1.84-slim-bookworm as builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /usr/src/app
COPY . .

# Install dependencies for compilation
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup component add clippy rustfmt

# Run lints
RUN cargo fmt --all -- --check
RUN cargo clippy --release --all-targets --all-features -- -D warnings

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary and static assets
COPY --from=builder /usr/src/app/target/release/manyfold-processor /usr/local/bin/manyfold-processor
COPY --from=builder /usr/src/app/static /app/static


# External Dependencies (if any are needed at runtime, e.g., ca-certificates for API calls)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create volume mount points as per architectural guidelines
VOLUME ["/input", "/output", "/config", "/app/temp"]

# Set Environment Variables
ENV RUST_LOG=info
ENV WORKING_DIR=/app/temp

CMD ["manyfold-processor"]
