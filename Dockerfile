# Stage 1: Build Rust Plugin
FROM rust:1-slim-bookworm as rust_builder
WORKDIR /build
COPY src/plugins/stl23mf /build
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse && cargo build --release

# Stage 2: Final Python Image
FROM python:3.11-slim-bookworm

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    mesa-utils \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY src /app

# Copy Rust optimizer from Stage 1
COPY --from=rust_builder /build/target/release/stl23mf /app/plugins/stl23mf

# Set permissions for entrypoint and plugin
RUN chmod +x /app/entrypoint.sh && chmod +x /app/plugins/stl23mf

# Expose Web UI port
EXPOSE 6767

# Volumes
VOLUME ["/input", "/output", "/staging", "/config"]

# Entrypoint to handle PUID/PGID
ENTRYPOINT ["./entrypoint.sh"]
