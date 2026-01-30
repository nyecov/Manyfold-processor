# Manyfold Processor

> **Automated 3D Assets Pipeline for Manyfold**

## 🎯 Project Aim
A tool to accept raw 3D files and archives, processing them into a standardized format ready for insertion into a self-hosted [Manyfold](https://github.com/manyfold3d/manyfold) instance.

## 🔄 Pipeline
*   **Inputs**: `.stl`, `.3mf`, Compressed Folders
*   **Outputs**:
    *   `model.3mf` (Standardized)
    *   `datapackage.json` (Metadata)
    *   `preview.webp` (Rendered Image)

## 🛠️ Usage

### Linux (Radxa Rock5 / Ubuntu)
Use the `manage.sh` script to control the service.
```bash
./manage.sh start   # Start service
./manage.sh logs    # View logs
```

### Windows
Use the `manage.ps1` PowerShell script.
```powershell
.\manage.ps1 start   # Start service
.\manage.ps1 logs    # View logs
```

## 🤖 Attribution
This project structure and documentation are assisted by **Antigravity with Gemini 3 Pro (High)**.