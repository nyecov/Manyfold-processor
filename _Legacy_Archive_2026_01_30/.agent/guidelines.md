# Project Guidelines

This document serves as the source of truth for the AI agent regarding project standards, style, and architecture.

## 1. Project Overview
- **Name**: Manyfold Processor
- **Purpose**: A geometry processing pipeline for Manyfold.
- **Language**: Python and Rust
- **Key Tools**: `stl23mf` (Internal Rust tool for efficient STL processing, replacing Trimesh).

## 2. Code Style
- Follow **PEP 8** standards for Python code.
- Use type hints for function arguments and return values.
- Docstrings should be present for all public modules, classes, and functions (Google style preferred).

## 3. Testing
- **Framework**: `pytest` and `behave` (BDD).
- Tests are located in the `tests/` directory.
- Only run the whole testing suite if explicitly requested.
- You may run individual tests if deemed necessary.

## 4. Documentation
- Update `README.md` if new features are added.
- Keep `Project_Context.md` updated with high-level architectural changes.

## 5. Agent Instructions
- Always check this file before starting major refactoring or new feature implementation.
- Prioritize current project conventions over general best practices if there is a conflict, unless otherwise instructed.

## 6. Development Workflow
- Repository is located at `https://github.com/nyecov/Manyfold-processor`.
- Use BDD and feature tests to design new features, validate existing features, and ensure that the codebase remains stable.
- Strive for a clean and maintainable codebase.
- Use version control (Git) to manage changes to the codebase.
- Keep the cucumber test steps clean, free from bloat, aim to reuse steps if it is logically possible.
- During refinement start from high level logic and then go to the details.
- Only develop code if the logic is clear, feature is well defined and refined with the user.

## 7. Project Technical Goals
- fast build time
- fast runtime
- minimal memory usage
- minimal disk space usage
- minimal dependencies
- minimal complexity
- minimal cpu usage
- Must be optimized to run inside a single docker container
- Must be able to run on a Radxa Rock 5 ITX (16gb ram)
- Project must be able to efficiently process and manipulate the following file types: 3mf, stl, obj, jpeg, jpg, png, webp, txt, ly (lychee slicer project file), zip, rar, 7zip, ctb, cbddlp, fdg, json.
