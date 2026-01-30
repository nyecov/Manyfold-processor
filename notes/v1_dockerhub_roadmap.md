# v1.0 Roadmap: DockerHub Publication

**Target Milestone**: v1.0
**Goal**: Publish the Manyfold Processor Docker image to DockerHub for public/private distribution.

---

## Technical Preparations Required

### 1. Image Naming & Tagging
*   **Repository Name**: `nyecov/manyfold-processor` (or preferred namespace)
*   **Tag Strategy**:
    *   `latest` - Always points to most recent stable
    *   `vX.Y.Z` - Semantic versioned releases
    *   `arm64` / `amd64` - Architecture-specific tags (if not using manifest)

### 2. Multi-Architecture Build
*   Use `docker buildx` for multi-platform images
*   Target architectures:
    *   `linux/arm64` (Radxa Rock 5 ITX - primary)
    *   `linux/amd64` (Development/Generic servers)
*   Command: `docker buildx build --platform linux/arm64,linux/amd64 --push -t nyecov/manyfold-processor:latest .`

### 3. Dockerfile Optimization
*   [ ] Ensure minimal final image size (use `slim` base images)
*   [ ] Remove build-time-only dependencies from final stage
*   [ ] Add LABEL metadata (maintainer, version, description)
*   [ ] Add HEALTHCHECK instruction for container orchestrators

### 4. CI/CD Integration
*   [ ] GitHub Actions workflow for automated builds on tag push
*   [ ] DockerHub automated builds linked to GitHub repo
*   [ ] Secret management for `DOCKERHUB_USERNAME` and `DOCKERHUB_TOKEN`

### 5. Documentation
*   [ ] README.md with pull/run instructions
*   [ ] Environment variable documentation
*   [ ] Volume mount requirements
*   [ ] Example `docker-compose.yml` for end users

---

## Pre-Publication Checklist
- [ ] All tests passing
- [ ] Security audit clean (`cargo audit`)
- [ ] Image tested on both arm64 and amd64
- [ ] CHANGELOG.md updated
- [ ] GitHub release created
- [ ] DockerHub repository description updated
