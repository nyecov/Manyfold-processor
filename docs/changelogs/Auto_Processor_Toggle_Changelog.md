# Auto-Processor Toggle: Changelog

## Summary
Fixed the Auto-Processor toggle in the Web UI, which was non-functional due to CSS/JS issues and a flawed toggle-based API design.

---

## Problems Identified

### 1. Toggle Not Clickable
- **Root Cause**: Toggle wrapped in `<div>` instead of `<label>`
- **Symptom**: Clicking the visual toggle didn't affect the hidden checkbox

### 2. State Constantly Reset
- **Root Cause**: `fetchStatus()` polled every 2s and overwrote `toggle.checked`
- **Symptom**: Any click was immediately reverted by the next poll

### 3. API Design Flaw (Toggle Endpoint)
- **Root Cause**: `POST /toggle-auto` flipped state server-side
- **Symptom**: Race conditions if clicked rapidly; UI/server could desync

---

## Changes Made

### Backend (`src/web.rs`)

| Before | After |
|--------|-------|
| `POST /api/config/toggle-auto` | `GET /api/config/auto-process` |
| Flips state internally | Returns `{ "enabled": bool }` |
| | `PUT /api/config/auto-process` |
| | Sets explicit `{ "enabled": bool }` |

- Default changed from `true` (ON) to `false` (OFF)

### Frontend (`static/index.html`)

1. Changed `<div>` wrapper to `<label>` for clickability
2. Added `togglePending` flag to prevent poll override during interaction
3. Changed JS from `POST toggle` to `PUT { enabled: value }`
4. Removed hardcoded `checked` attribute

---

## Verification

```powershell
# Set to OFF
Invoke-WebRequest -Method PUT -Uri "http://localhost:8080/api/config/auto-process" `
  -ContentType "application/json" -Body '{"enabled":false}'

# Get current state
Invoke-WebRequest -Uri "http://localhost:8080/api/config/auto-process"
```

---

## Files Modified
- `src/web.rs` – API refactor
- `static/index.html` – UI fixes
