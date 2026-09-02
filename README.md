# WorkWeave Engine

WorkWeave Engine is a Rust execution platform with two sibling kernels:

- **WorkWeave Agent** — a probabilistic LLM/tool execution engine inspired by Pi.
- **WorkWeave Flow** — a deterministic durable workflow engine that executes the qualified Open Workflow Specification (OWS) profile.

Both engines share a runtime substrate for identity, lifecycle, cancellation, persistence, policy, audit, observability, artifacts, configuration, deployment, SDKs, CLI, and TUI surfaces. They do **not** share one execution state machine.

WorkWeave Orchestration remains a layer above the engine. It owns Goal-, Task-, Question-, Decision-, Evaluation-, Review-, epistemic-, deontic-, and temporal semantics. It dispatches Agent or Flow as execution primitives.

## Architecture

```text
                    WorkWeave Orchestration
                  governed work coordination
                             |
                             v
                  +-----------------------+
                  |   WorkWeave Engine    |
                  |      Rust layer       |
                  +-----------+-----------+
                              |
              +---------------+---------------+
              |                               |
              v                               v
      WorkWeave Flow                   WorkWeave Agent
      deterministic                    probabilistic
      OWS execution                    LLM <-> tools loop
              |                               |
              +---------------+---------------+
                              |
                    shared runtime substrate
```

## Reference mapping

| Reference | WorkWeave target |
| --- | --- |
| Pi Agent | WorkWeave Agent |
| Pi future Harness | execution/runtime patterns relevant to WorkWeave Orchestration and durable runs |
| OWS | WorkWeave Flow definition authority |
| LangGraph | WorkWeave Flow runtime reference for durability, checkpoints, interrupts, streaming, and execution mechanics |

## Repository orientation

- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md` — integrated architecture and implementation blueprint.
- `docs/architecture/PI-REFERENCE-ARCHITECTURE.md` — source-grounded Pi analysis.
- `docs/architecture/FLOW-REFERENCE-ARCHITECTURE.md` — OWS and LangGraph runtime analysis.
- `docs/architecture/RUST-CONTRACTS.md` — proposed Rust crate and interface contracts.
- `docs/architecture/SOURCE-REGISTER.md` — immutable evidence pins.
- `docs/adr/` — Goal-linked architecture decisions; every Goal must have an ADR before activation.
- `docs/orchestration/` — references canonical WorkWeave Orchestration v0.5; this repository does not fork that model.
- `docs/templates/` — reusable Goal/Plan/Task/Decision/Question/Evaluation/Review/ADR authoring supports.
- `goals/` — Goal packets and project bookkeeping.

## Current Goal

`G002 — Shared Runtime Walking Skeleton` is engineering-complete and awaiting independent review T010. Its runtime/SQLite/cancellation/audit acceptance suite is green.

`G003 — Thin Agent Kernel` is fully drafted but remains **proposed**. It will activate only after G002 review acceptance and ADR-0003 acceptance. G003 proves one bounded provider → tool → provider Agent Run before WorkWeave Flow is implemented.
