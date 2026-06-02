# ADR 002: Media engine backend

## Status

Accepted (phased)

## Context

Industry-leading video needs always-on capture graph, shared preview, hardware plugins (NDI, DeckLink), and stream egress.

## Decision

1. **Phase 2–3:** `media-engine` exposes a stable Rust API and FSM; backend uses **FFmpeg CLI** behind a process manager (familiar from v2, fastest path to seamless record toggle).
2. **Phase 3+:** Introduce **GStreamer** pipeline behind the same trait when packaging on Windows/macOS is validated.

## Consequences

- FFmpeg CLI is not the end state but unblocks Phase 2 QA
- UI never calls FFmpeg directly — only `media-engine` commands
