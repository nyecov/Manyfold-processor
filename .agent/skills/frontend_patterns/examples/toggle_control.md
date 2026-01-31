# Toggle Control: Code Examples

## HTML Structure
```html
<label class="toggle-wrapper">
    <input type="checkbox" id="my-toggle" class="sr-only">
    <div class="toggle-visual"><!-- CSS-styled --></div>
</label>
```

> Use `<label>`, not `<div>`. Labels propagate clicks to child inputs.

---

## JavaScript
```javascript
const toggle = document.getElementById('my-toggle');
let pendingRequest = false;

toggle.onchange = async () => {
    pendingRequest = true;
    const newValue = toggle.checked;
    try {
        const res = await fetch('/api/config/setting', {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ enabled: newValue })
        });
        toggle.checked = (await res.json()).enabled;
    } catch (e) {
        toggle.checked = !newValue; // Revert on failure
    } finally {
        pendingRequest = false;
    }
};

// Polling (skip during pending requests)
setInterval(async () => {
    if (!pendingRequest) {
        const res = await fetch('/api/config/setting');
        toggle.checked = (await res.json()).enabled;
    }
}, 2000);
```

---

## Rust/Axum Backend
```rust
#[derive(Deserialize)]
struct Payload { enabled: bool }

async fn get_setting(Extension(state): Extension<AppState>) -> Json<Value> {
    Json(json!({ "enabled": *state.setting.lock().unwrap() }))
}

async fn set_setting(
    Extension(state): Extension<AppState>,
    Json(p): Json<Payload>
) -> Json<Value> {
    *state.setting.lock().unwrap() = p.enabled;
    Json(json!({ "status": "success", "enabled": p.enabled }))
}
```

---

## Project Reference
- Backend: [web.rs](../../../../src/web.rs)
- Frontend: [index.html](../../../../static/index.html)
