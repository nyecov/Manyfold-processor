# Manyfold Processor

> **Status**: In Active Development (Rust Overhaul)

A standalone utility for processing 3D models for [Manyfold](https://manyfold.app), designed to optimize file handling on the Radxa Rock 5 ITX.

## 📚 Documentation

The detailed documentation for this project is maintained as **Agent Skills**. Please refer to the following resources:

### 🏗️ Architecture & Specs
*   [**Project Details**](.agent/skills/project_details/SKILL.md): Repository info and high-level status.
*   [**Architectural Guidelines**](.agent/skills/architectural_guidelines/SKILL.md): C4 model and design principles (Rust + Docker).
*   [**Developer Hardware Specs**](.agent/skills/developer_hardware_specs/SKILL.md): Dev machine specifications.

### 🚀 Integration & Deployment
*   [**Manyfold API Integration**](.agent/skills/manyfold_api_integration/SKILL.md): How to talk to Manyfold via Tus/JSON.
*   [**Deploy on Radxa Rock 5**](.agent/skills/deploy_on_radxa_rock5/SKILL.md): Hardware-specific deployment guide.

## 🛠️ Development

This project is being rewritten in **Rust**.
*   **Container**: Docker-first workflow.
*   **Interaction**: API-only communication with Manyfold.
