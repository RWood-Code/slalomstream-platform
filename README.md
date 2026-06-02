# SlalomStream Platform

Industry-oriented tournament operations platform for waterski slalom — rebuilt from the ground up as a **venue node** (local authority) with optional cloud services.

**Repository:** https://github.com/RWood-Code/slalomstream-platform  
**Version:** 3.0.0-alpha (phased rebuild — see [docs/BUILD_PLAN.md](docs/BUILD_PLAN.md) and [docs/qa/](docs/qa/))

## Architecture

| Component | Crate / App | Role |
|-----------|-------------|------|
| Event Core | `slalom-core` | Tournament data, scoring, SQLite, event log |
| Scoring rules | `slalom-scoring` | IWWF-valid scores, panel logic |
| Real-time bus | `slalom-bus` | WebSocket pub/sub for venue clients |
| HTTP API | `slalom-api` | REST + WS mount (OpenAPI-aligned subset) |
| Media Engine | `media-engine` | Capture, record, preview state machine |
| Officials registry | `slalom-registry` | Versioned import/sync |
| Venue binary | `venue-node` | Runs Event Core + API on the lake |
| Operator UI | `clients/operator-ui` | React operator / judge / live surfaces |
| Cloud hub | `cloud/hub-api` | Registry sync, VOD stubs (Phase 6) |

See [docs/BUILD_PLAN.md](docs/BUILD_PLAN.md) for phased delivery and QA gates.

## Quick start

```bash
# Requires Rust stable (https://rustup.rs)
cargo run -p venue-node

# Health
curl http://127.0.0.1:3010/healthz

# Operator UI (separate terminal)
cd clients/operator-ui && npm install && npm run dev
# → http://localhost:3020

# Cloud hub stub (optional)
cd cloud/hub-api && npm install && npm start
# → http://127.0.0.1:3021/healthz
```

Default listen: `127.0.0.1:3010` (configurable via `VENUE_PORT`).

Data directory: `%LOCALAPPDATA%/SlalomStream/venue` or `VENUE_DATA_DIR`.

## Phase status (QA)

| Phase | Scope | QA doc |
|-------|--------|--------|
| 0 | Docs, ADRs, CI | [PHASE-0.md](docs/qa/PHASE-0.md) |
| 1 | Event Core + REST + WS | [PHASE-1.md](docs/qa/PHASE-1.md) |
| 2 | Media FSM + recordings | [PHASE-2.md](docs/qa/PHASE-2.md) |
| 3 | Encoder / pre-roll stubs | [PHASE-3.md](docs/qa/PHASE-3.md) |
| 4 | Registry CSV import | [PHASE-4.md](docs/qa/PHASE-4.md) |
| 5 | Operator React UI | [PHASE-5.md](docs/qa/PHASE-5.md) |
| 6 | Cloud hub stub | [PHASE-6.md](docs/qa/PHASE-6.md) |

**Next major work:** Wire FFmpeg/GStreamer into `media-engine` (Phase 3b), port remaining v2 routes (judging, EMS, SurePath), Tauri installer.

## Development

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## License

MIT — see [LICENSE](LICENSE).
