# Builder stage
FROM --platform=$BUILDPLATFORM rust:1.85-slim-bookworm AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /usr/src/app

# 1. Create a dummy project to cache dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./
# Install external dependencies if any (e.g., openssl)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# 2. Build dependencies only
RUN mkdir tests && touch tests/cucumber_runner.rs
RUN cargo build --release
RUN rm src/*.rs && rm -rf tests

# 3. Copy actual source code
COPY . .

# 4. Touch main.rs to force rebuild of the app (but not deps)
RUN touch src/main.rs

# 5. Run lints (skipped when SKIP_LINTING=true for faster dev builds)
ARG SKIP_LINTING=false
RUN if [ "$SKIP_LINTING" = "false" ]; then \
    rustup component add clippy rustfmt && \
    cargo fmt --all -- --check && \
    cargo clippy --release --all-targets --all-features -- -D warnings; \
    fi

# 6. Build final binary
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
