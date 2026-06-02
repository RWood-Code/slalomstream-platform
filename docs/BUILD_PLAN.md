# SlalomStream Platform — Build Plan

**Product:** Operating system for a slalom tournament day — capture, score, display, review, distribute.

**Baseline:** SlalomStream v2.0.0 (Tauri + React + Express + FFmpeg sidecars).

**Target:** Venue Node (Rust) + thin clients + optional cloud hub.

---

## Phase gates

Each phase ends with **QA** documented in `docs/qa/PHASE-N.md`. The next phase does not start until QA passes.

| Phase | Goal | QA criteria (summary) |
|-------|------|------------------------|
| **0** | Foundation | Docs, ADRs, workspace compiles, CI green |
| **1** | Event Core | CRUD tournaments/passes/scores; WS events; offline SQLite |
| **2** | Media v1 | Media FSM; recordings↔pass; preflight API; tests |
| **3** | Media v2 | Encoder profiles; pre-roll stub; egress config API |
| **4** | Registry | CSV import; versioned snapshots; update check |
| **5** | Operator UI | React app talks to venue-node; core routes |
| **6** | Cloud stubs | Hub API skeleton; GitHub release workflow |

---

## Phase 0 — Foundation

- Monorepo layout, MIT license, README
- ADRs: Rust venue node, media backend (FFmpeg → GStreamer path)
- Domain spec: pass lifecycle
- Media spec: FSM states
- CI: `cargo test`, `clippy`

## Phase 1 — Event Core

- `slalom-core`: SQLite schema, migrations, repositories
- `slalom-scoring`: IWWF scores, panel roles (port from v2 `utils.ts`)
- `slalom-bus`: WebSocket broadcast
- `slalom-api`: Axum routes aligned with v2 OpenAPI subset
- SurePath client hook (config + stub)

## Phase 2 — Media Engine v1

- `media-engine`: `Idle → PreviewReady → Armed → Recording → Finalizing`
- `recordings` table linked to `pass_id`
- Preflight: disk, device placeholder, write test
- Single preview port owner (no :9877 collision)

## Phase 3 — Media Engine v2

- Encoder presets (Standard / High / Broadcast)
- Pre-roll buffer interface
- SRT/RTMP egress configuration (stub until hardware lab)

## Phase 4 — Global registry

- `slalom-registry`: snapshot version, CSV import
- Remove hardcoded NZ-only seed from runtime path

## Phase 5 — Operator UI

- Vite + React client: Home, Recording shell, Judging, Scoreboard, Live
- WebSocket + REST to venue-node

## Phase 6 — Cloud hub

- `cloud/hub-api`: registry publish, health (stubs)
- GitHub Actions release

---

## Success metrics (MVP = Phase 1+2)

| Metric | Target |
|--------|--------|
| WS score → client | < 500 ms |
| Pass without DB link to recording | 0 when auto-record armed |
| `cargo test --workspace` | pass |
| Fresh install officials | import-driven, not baked binary |

---

## Migration from v2

1. Export PGlite tournament JSON (tool TBD)
2. Import into venue SQLite
3. Parallel pilot event before retiring v2 installer
