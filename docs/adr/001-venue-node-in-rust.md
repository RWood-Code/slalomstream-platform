# ADR 001: Venue Node in Rust

## Status

Accepted

## Context

SlalomStream v2 runs Express + PGlite as a Node sidecar inside Tauri, with FFmpeg as separate child processes orchestrated from a 3,500-line React page. Event-day reliability and global scale need a single local authority for data and media.

## Decision

Implement the **Venue Node** as a Rust workspace:

- `slalom-core` — SQLite, domain, event log
- `media-engine` — capture/record lifecycle
- `venue-node` — binary binding HTTP + WebSocket

UI layers (React) are thin clients only.

## Consequences

- Requires Rust toolchain for developers and CI
- Node/Express removed from event-critical path
- Easier to embed GStreamer/FFmpeg as libraries later
