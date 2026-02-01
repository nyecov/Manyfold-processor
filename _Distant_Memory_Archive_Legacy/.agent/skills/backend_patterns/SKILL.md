---
name: Backend Patterns
description: Repository for basic Rust/Axum backend patterns and API endpoint designs.
---

# Backend Patterns

A library of backend endpoint patterns and Rust/Axum API designs. Use these as building blocks when implementing new features.

## Available Patterns

### 1. Config Endpoint Pattern
A GET/PUT pattern for boolean configuration values.

**Key Principles:**
- Use explicit `PUT { value }` (not toggle/flip endpoints)
- Return the new value in the response for frontend sync
- Keep state in `Arc<Mutex<T>>` for thread safety

**API Contract:**
| Method | Endpoint | Body | Response |
|--------|----------|------|----------|
| `GET` | `/api/config/{setting}` | — | `{ "enabled": bool }` |
| `PUT` | `/api/config/{setting}` | `{ "enabled": bool }` | `{ "status": "success", "enabled": bool }` |

See [examples/config_endpoint.md](examples/config_endpoint.md) for implementation code.

---

## Checklist (All Patterns)

- [ ] Use explicit values, not toggle/flip
- [ ] Return new state in response
- [ ] Use proper locking for shared state
- [ ] Log state changes for observability

---

## See Also
- [Frontend Patterns](../frontend_patterns/SKILL.md) – Companion UI components
- [Deployment Operations](../deployment_operations/SKILL.md) – Container orchestration
