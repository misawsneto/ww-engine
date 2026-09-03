# Plan

## Title

- Validate the durable kernel through one real protocol and one bounded local effect

## State

- draft

## Slicing Strategy

Vertical + contract-first.

```text
G003 normalized Provider contract ─→ OpenAI adapter + local mock server ─┐
                                                                        ├→ SDK projection → CLI E2E
G003 Tool contract ────────────────→ bounded fs.read ───────────────────┘
```

## Strategy

1. Accept G003 review and ADR-0004 before activation.
2. Implement OpenAI protocol translation against deterministic HTTP/SSE fixtures before any live credentialed call.
3. Implement `fs.read` as a deliberately narrow read-only capability with canonical workspace containment and output bounds.
4. Add a stable Agent projector/SDK façade over committed G003 state.
5. Add CLI only after provider/tool/SDK contracts pass independently.
6. Prove the complete CLI path against a local mock OpenAI-compatible server in separate OS processes.
7. Run optional live smoke only after deterministic acceptance; live provider availability is never an acceptance dependency.
8. Perform architecture/security review before Goal completion.

## Stop Conditions

- Stop if OpenAI transport types leak into G003 kernel contracts.
- Stop if credentials/raw authorization headers enter durable audit or CLI output.
- Stop if `fs.read` cannot prove canonical workspace containment before opening content.
- Stop if CLI requires direct store/SQLite access.
- Stop if G004 changes G003 replay/recovery semantics instead of adapting to them.
- Stop if a second provider/tool category is added to "finish the abstraction".

## Rollback

Remove G004 adapters/surfaces while preserving the accepted G003 kernel. Retain deterministic provider/fs/CLI fixtures and review findings documenting any rejected adapter design.
