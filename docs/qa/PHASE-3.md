# Phase 3 QA — Media Engine v2 (partial alpha)

**Date:** 2026-06-03

## Delivered in alpha

- Encoder profile field on `MediaStatus` (Standard / High / Broadcast mapping in API)
- Pre-roll seconds configurable on engine (`set_pre_roll_secs`)
- Egress: documented stub; RTMP/SRT config endpoint deferred

## Checklist

| # | Criterion | Result |
|---|-----------|--------|
| 1 | Encoder profile API accepts profile | ☑ stub |
| 2 | Pre-roll interface exists | ☑ |
| 3 | FFmpeg integration | ☐ Phase 3b |

**Gate:** Phase 4 — registry (no blocker).
