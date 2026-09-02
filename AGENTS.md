# AGENTS.md

## Purpose

- Build WorkWeave Engine as one Rust execution platform with independent Agent and Flow kernels.
- Treat WorkWeave Agent as a bounded probabilistic worker and WorkWeave Flow as a deterministic durable workflow worker.
- Keep WorkWeave Orchestration semantic state above the engine rather than embedding Goal/Task/Evaluation semantics into the Agent loop.
- Preserve source-grounded architecture evidence and distinguish observed reference behavior from proposed WorkWeave design.

## Operating rules

- Verify before mutation.
- Use the simplest working approach that preserves coherent verified work.
- Work directly in `/mnt/data/ww-engine` when a local checkout is available.
- Keep the Agent and Flow state machines independent even when they share runtime infrastructure.
- Keep accepted OWS documents authoritative for qualified Flow-definition semantics.
- Do not create a second canonical WorkWeave workflow graph DSL beneath OWS.
- Treat ordinary model calls and tool calls as execution audit/observability, not orchestration semantic records.
- Treat Flow completion, Agent completion, Task completion, and Goal achievement as different concepts.
- Use immutable upstream source pins for material reference-architecture claims.
- Separate source-observed, derived, and proposed claims.
- Capture durable direction in `DECISIONS.md`, uncertainty in `QUESTIONS.md`, active state in `PROJECT_STATE.md`, and Goal-owned work under `goals/`.
- Keep reusable WorkWeave templates and skills independent from product-specific implementation details.

## Architecture authority

- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md` is the current proposed engine architecture.
- Canonical WorkWeave Orchestration Domain/Flow semantics live in `misawsneto/ww-orchestration`; references here are non-authoritative pins.
- Pi, LangGraph, and OWS are reference evidence; none is authoritative for WorkWeave Engine design.

## Now / next

- Now — G002 — implement the shared runtime walking skeleton only: IDs, lifecycle, SQLite transactions, durable audit/events, cancellation, artifacts, SDK inspection/event streaming, and CLI.
- Next — G003 — implement the thin probabilistic Agent kernel on that substrate.
- Then — G004/G005 — implement the deterministic OWS Flow kernel and immediately prove restart-safe Flow-to-Agent integration.
- Later — broaden OWS coverage, providers, policy/sandboxing, remote execution, server/TUI depth, and plugins without weakening the sibling-kernel boundary.
