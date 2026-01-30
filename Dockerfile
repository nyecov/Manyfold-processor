# Builder stage
FROM --platform=$BUILDPLATFORM rust:1.84-slim-bookworm as builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /usr/src/app
COPY . .

# Install dependencies for compilation if needed
# Note: For strict cross-compilation without QEMU, we would need 'xx-rs' or similar.
# For now, we rely on Docker Buildx QEMU emulation for simplicity.

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary
COPY --from=builder /usr/src/app/target/release/manyfold-processor /usr/local/bin/manyfold-processor


# External Dependencies (if any are needed at runtime, e.g., ca-certificates for API calls)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create volume mount points as per architectural guidelines
VOLUME ["/input", "/output", "/config", "/app/temp"]

# Set Environment Variables
ENV RUST_LOG=info
ENV WORKING_DIR=/app/temp

CMD ["manyfold-processor"]
