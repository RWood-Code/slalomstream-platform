# Pass lifecycle

## States

| Status | Meaning |
|--------|---------|
| `pending` | Active run; judges may score |
| `scored` | All required panel scores received |
| `complete` | Chief collation done / finalized |
| `cancelled` | Voided pass |

## Transitions

```
created → pending
pending → scored (auto when panel complete)
scored → complete (chief action or auto policy)
pending → cancelled (operator)
```

## SurePath / WaterskiConnect

External `pass_start` events create `pending` passes on the active tournament when SurePath is enabled.

## Media coupling (Phase 2+)

When media is **Armed**:

- `pending` entered → start recording (with pre-roll when available)
- `pending` cleared → finalize recording, link `recordings.pass_id`
