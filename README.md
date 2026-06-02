# SlalomStream Platform

Industry-oriented tournament operations platform for waterski slalom — rebuilt from the ground up as a **venue node** (local authority) with optional cloud services.

**Version:** 3.0.0-alpha (phased rebuild)

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
```

Default listen: `127.0.0.1:3010` (configurable via `VENUE_PORT`).

Data directory: `%LOCALAPPDATA%/SlalomStream/venue` or `VENUE_DATA_DIR`.

## Development

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## License

MIT — see [LICENSE](LICENSE).
