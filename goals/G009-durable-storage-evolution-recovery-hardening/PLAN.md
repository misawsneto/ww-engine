# Plan

## Title

- Harden durable evolution without changing execution semantics

## State

- draft

## Slicing Strategy

Contract-first + compatibility-first.

The proposed Goal separates durable-format and backend evolution from Agent and Flow behavior. Activation must start with an accepted ADR and an evidence-based scheduling decision after G003.

## Strategy

1. Decide activation order after G003 and accept a governing ADR without making G009 an automatic prerequisite for G004.
2. Inventory component-owned durable schemas and payload envelopes, then define their version and ownership contracts.
3. Prove migrations from committed known-old fixtures and fail-closed handling of future versions.
4. Define idempotent create/ensure behavior for committed-but-unacknowledged outcomes.
5. Define recovery-oriented storage failure classes independent of SQLite-specific error details.
6. Extract only physical SQLite mechanics that can be reused without centralizing component schemas or durable DTOs.
7. Run cross-adapter migration, retry, reopen, concurrency, and ambiguity conformance tests.
8. Perform a storage-boundary and compatibility review before completion.

## Stop Conditions

- Stop if reuse requires Agent or Flow DTOs to enter shared store contracts.
- Stop if a migration can partially mutate durable state before rejecting an unsupported version.
- Stop if create/ensure idempotency depends on process-local memory.
- Stop if error classification changes Agent, Flow, or Orchestration semantics.
- Stop if G009 expands into provider, tool, workflow-interpreter, SDK, CLI, TUI, filesystem, or network work.
- Stop if activation is justified only by the existence of review findings rather than a concrete scheduling decision.

## Rollback

Revert G009 implementation while preserving its ADR, fixtures, compatibility evidence, and review findings. Previously accepted Agent and Flow behavior remains authoritative.
