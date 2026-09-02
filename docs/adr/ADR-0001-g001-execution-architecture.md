# ADR-0001 — WorkWeave execution architecture baseline

- Status: accepted
- Goal: G001 — WorkWeave Execution Architecture
- Recorded: 2026-09-02
- Note: recorded retrospectively when the repository adopted an explicit ADR-per-Goal rule; it captures decisions already accepted during G001.

## Context

WorkWeave needs both a probabilistic Agent execution primitive and a deterministic durable workflow engine. Pi, its future Harness work, OWS, LangGraph, and the WorkWeave Orchestration model provide useful but non-identical reference architectures.

## Decision

1. Build WorkWeave Engine in Rust.
2. Implement WorkWeave Agent and WorkWeave Flow as sibling kernels on one shared operational runtime.
3. Do not force Agent and Flow through one internal state machine.
4. Keep WorkWeave Orchestration above both kernels; Goal/Task/Question/Decision/Evaluation/Review and epistemic/deontic/temporal semantics do not belong inside the Agent loop or Flow runtime.
5. Use Pi Agent as the primary Agent implementation reference and Pi Harness only as a durability/coordination reference.
6. Keep accepted OWS documents authoritative for the qualified Flow definition profile.
7. Use LangGraph for Flow runtime mechanics such as checkpoints, interrupts, resumability, streaming, and durable step execution; do not create a competing graph DSL.
8. Preserve an explicit A2A-shaped boundary for Flow-to-Agent invocation even when both execute in-process.
9. Treat durable audit records as product data distinct from exportable telemetry.
10. Expose Agent and Flow through first-class SDK, CLI, and TUI surfaces over time.

## Consequences

- Agent and Flow can share IDs, cancellation, persistence infrastructure, audit envelopes, policy primitives, artifacts, configuration, deployment machinery, and observability.
- Engine-specific domain/state types remain owned by their kernels.
- Local execution optimizations cannot bypass the logical external-execution seams that make remote deployment possible later.
- OWS source remains the workflow authority; compiled plans are disposable runtime artifacts.

## Rejected alternatives

- Port Pi package-for-package.
- Treat LangGraph's graph model as WorkWeave's canonical workflow language.
- Embed WorkWeave Orchestration semantics into Agent state.
- Build one universal state machine for jobs, Agents, and Flows.

## Evidence

- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`
- `docs/architecture/PI-REFERENCE-ARCHITECTURE.md`
- `docs/architecture/FLOW-REFERENCE-ARCHITECTURE.md`
- G001 verification and accepted user architecture review.
