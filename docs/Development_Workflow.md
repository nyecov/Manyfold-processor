# Development Workflow Guide

This guide explains how to use Docker Compose profiles for efficient local development.

## Quick Reference

| Workflow | Command | Build Time | Features |
|----------|---------|------------|----------|
| **Dev** | `docker compose --profile dev up --build` | ~2-3 min | No linting, live static files |
| **Prod** | `docker compose up --build` | ~5-8 min | Full linting, release build |

## Development Profile (`--profile dev`)

The dev profile is optimized for rapid iteration:

```bash
# Start dev container (first time or after Rust changes)
docker compose --profile dev up --build

# Rebuild after Rust code changes
docker compose --profile dev up --build --force-recreate
```

### Features

1. **Skips Linting** – No `clippy` or `fmt --check` during build
2. **Volume-Mounted Static Files** – Frontend changes (HTML/CSS/JS) are instant, no rebuild needed
3. **Debug Logging** – `RUST_LOG=debug` for detailed output

### Frontend-Only Changes

After starting the dev container, edits to files in `./static/` are reflected immediately—just refresh your browser. No container rebuild required.

## Production Build

For final testing or deployment, use the default production service:

```bash
# Full build with linting
docker compose up --build

# Force clean rebuild
docker compose build --no-cache && docker compose up
```

### What Production Adds

- `cargo fmt --check` – Code formatting validation
- `cargo clippy` – Lint warnings as errors
- `RUST_LOG=info` – Standard logging level

## Switching Between Profiles

The dev and prod services use different container names, so they don't conflict. However, they share the same port (`8080`). Stop one before starting the other:

```bash
# Stop dev, start prod
docker compose --profile dev down
docker compose up --build

# Stop prod, start dev
docker compose down
docker compose --profile dev up --build
```

## Configuration Reference

### Dockerfile Build Arg

```dockerfile
ARG SKIP_LINTING=false  # Set to "true" to skip linting
```

### Compose Profile

The `processor-dev` service activates only when explicitly requested with `--profile dev`.

---

**See Also**: [Deployment Operations](../.agent/skills/deployment_operations/SKILL.md)
