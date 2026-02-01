---
name: Frontend Patterns
description: Repository for basic frontend elements and atomic UI components.
---

# Frontend Patterns

A library of atomic UI components and basic frontend elements. Use these as building blocks when implementing new features.

## Available Patterns

### 1. Toggle Control Pattern
A persistent on/off toggle with REST API synchronization.

**Key Principles:**
- Use `<label>` wrapper (not `<div>`) for click propagation
- Use explicit `PUT { enabled: bool }` (not toggle/flip endpoints)
- Debounce polling during pending requests
- Default state in backend, not hardcoded in HTML

**API Contract:**
| Method | Endpoint | Body | Response |
|--------|----------|------|----------|
| `GET` | `/api/config/{setting}` | — | `{ "enabled": bool }` |
| `PUT` | `/api/config/{setting}` | `{ "enabled": bool }` | `{ "status": "success", "enabled": bool }` |

See [examples/toggle_control.md](examples/toggle_control.md) for implementation code.

---

## Checklist (All Patterns)

- [ ] API uses explicit values, not toggle/flip
- [ ] Frontend syncs with server response
- [ ] Polling respects pending request state
- [ ] Default state lives in backend only

---

## See Also
- [Backend Patterns](../backend_patterns/SKILL.md) – Companion API patterns
- [Deployment Operations](../deployment_operations/SKILL.md) – Container orchestration
