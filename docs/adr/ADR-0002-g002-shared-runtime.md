# ADR-0002 — Shared runtime walking skeleton

- Status: accepted
- Goal: G002 — Shared Runtime Walking Skeleton
- Recorded: 2026-09-02
- Note: recorded retrospectively when the ADR-per-Goal rule was introduced; it captures the architecture actually implemented and verified in G002.

## Context

Before adding either execution kernel, WorkWeave needed evidence that a common Rust substrate could be durable and inspectable without importing Agent or Flow semantics.

## Decision

1. The shared runtime owns only operational concepts: execution identity/lifecycle, ordered durable events, cancellation, artifact references, storage ports, SDK inspection, and CLI projection.
2. Use SQLite first in embedded mode, with a storage port that can admit PostgreSQL later.
3. Commit current execution state and its corresponding durable event atomically with optimistic version checks.
4. Keep event history reducible and compare the reconstructed projection with the current row during inspection.
5. Use content-addressed local artifacts with SHA-256 identity and durable metadata.
6. Cancellation is two-phase: `request_cancel` durably records intent/reason and signals local cancellation; `settle_cancelled` may terminalize only after that durable request exists.
7. CLI callers use `ww-sdk`; CLI code does not mutate SQLite directly.
8. Agent/provider/tool-loop concepts and Flow/OWS/token/interpreter concepts are forbidden from common runtime crates.
9. Pin Rust 1.98.0 and enforce format, architecture-boundary checks, clippy with warnings denied, and workspace tests in CI.

## Consequences

- G003 can bind an Agent Run to a common `ExecutionRecord` without making the runtime understand messages, models, providers, or tools.
- G005 can bind Flow execution to the same substrate without reusing Agent state.
- A shared physical SQLite database is permitted, but logical ownership and engine-specific repositories remain separate.
- Engine-specific persistence introduced later must coordinate with shared execution/audit state without pushing engine-owned types into `ww-store`.

## Rejected alternatives

- JSONL as the canonical runtime store.
- OpenTelemetry as the only execution record.
- Immediate PostgreSQL/distributed-worker implementation.
- A generic runtime aggregate containing Agent and Flow state.

## Evidence

- Permanent CI run `33646651848`.
- G002 process-boundary CLI lifecycle test.
- G002 optimistic-version conflict integration test.
- G002 cancellation/audit review finding and corrected two-phase lifecycle.
