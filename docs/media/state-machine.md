# Media engine state machine

```
Idle
  → PreviewReady   (device open, preview stream live)
  → Armed          (preview + waiting for pass)
  → Recording      (file writer active, same capture graph)
  → Finalizing     (flush container, write DB row)
  → Armed | PreviewReady
```

## Rules

1. **One capture session** per event day session — no stop/start preview on record toggle.
2. **One preview publisher** — all UI clients subscribe (HTTP/WebSocket), port not owned per page.
3. **Preflight** must pass before `Armed` is allowed.

## Preflight checks

- Data directory writable
- Free disk ≥ configurable threshold (default 10 GB)
- Device probe (platform-specific; may warn-only on dev machines)
