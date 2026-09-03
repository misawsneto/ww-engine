# G002 — Shared Runtime Walking Skeleton

## Statement

Prove that WorkWeave Engine has a durable, inspectable Rust execution substrate that both future Agent and Flow kernels can use without importing either kernel's semantics.

## State

- achieved

## Architecture Decision Records

- `docs/adr/ADR-0002-g002-shared-runtime.md` — accepted.

## Boundaries

- Implement shared execution infrastructure only.
- Do not implement the LLM/tool Agent loop.
- Do not implement OWS interpretation or Flow tokens.
- Do not introduce Agent- or Flow-owned state into shared runtime aggregates.
- Use SQLite as the first physical store while preserving a store contract that can admit PostgreSQL later.
- Keep audit history durable and distinct from transient telemetry.

## Success Criteria

- A Rust workspace builds with separate types, runtime, SQLite store, SDK, and CLI boundaries.
- A synthetic execution can be created, started, cancelled, and terminalized through runtime APIs.
- Every lifecycle mutation atomically changes current execution state and appends one ordered durable execution event.
- Execution state and event history survive process restart and reconstruct to the same projection.
- Optimistic version checks reject conflicting writers.
- A durable cancel request is recorded and an in-process cancellation token is signalled when the execution is locally registered.
- Content-addressed artifacts can be stored on the local filesystem and inspected through a stable `ArtifactRef`.
- SDK inspection and cursor-based event streaming work without direct database access by clients.
- The `ww` CLI can create/start/cancel/settle/inspect executions and print committed events in machine-readable form.
- Shared runtime crates contain no Agent message/provider/tool-loop types and no Flow/OWS/token/interpreter types.

## Requirements

- Pin the Rust toolchain used by CI.
- Verify with executable tests, including reopen/restart behavior and lifecycle reduction.
- Keep database migrations in source control.
- Make invalid lifecycle transitions fail without appending an event.
- Keep external side effects out of G002 except local artifact writes.

## Dependencies

- G001 — WorkWeave Execution Architecture.
