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
- `docs/orchestration/` — references canonical WorkWeave Orchestration v0.5; this repository does not fork that model.
- `docs/templates/` — reusable Goal/Plan/Task/Decision/Question/Evaluation/Review authoring supports from the orchestration starter.
- `docs/STARTER-ADOPTION.md` — what was adopted, adapted, or deliberately held back from the starter.
- `goals/` — Goal packets and project bookkeeping.

## Current Goal

`G001 — WorkWeave Execution Architecture` establishes the architecture boundary before production Rust implementation.

The next proposed Goal is a narrow Rust execution-kernel spike that proves one local Agent run, one durable OWS Flow, Flow-to-Agent invocation, recovery, and ordered audit on the shared substrate.
