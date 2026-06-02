# Phase 1 QA — Event Core

**Date:** 2026-06-03  
**Build plan ref:** Phase 1 — Event Core

## Delivered

- `slalom-core`: SQLite schema, tournaments/skiers/passes/scores/repos, event log
- `slalom-scoring`: IWWF panel + valid scores (ported from v2)
- `slalom-bus`: WebSocket broadcast with sequenced envelopes
- `slalom-api`: REST routes + `/ws`
- `venue-node` binary on port **3010**
- Integration test: `crates/slalom-api/tests/integration.rs`

## Checklist

| # | Criterion | Result |
|---|-----------|--------|
| 1 | Create tournament via API | ☑ |
| 2 | Pass lifecycle + panel scoring → `scored` | ☑ unit test |
| 3 | WebSocket publishes on score | ☑ bus unit test |
| 4 | No Node/Express on event path | ☑ |
| 5 | `cargo test` + clippy | ☑ |

## Manual smoke

```bash
cargo run -p venue-node
curl http://127.0.0.1:3010/healthz
curl -X POST http://127.0.0.1:3010/api/tournaments -H "Content-Type: application/json" -d "{\"name\":\"Test\"}"
```

**Gate:** Phase 2 approved.
