# WorkWeave Engine

WorkWeave Engine is a Rust execution platform with two independent execution kernels on one shared operational substrate:

- **WorkWeave Agent** — a bounded probabilistic model/tool worker;
- **WorkWeave Flow** — a deterministic durable OWS workflow worker.

WorkWeave Orchestration remains a layer above the engine. It owns Goal-, Task-, Question-, Decision-, Evaluation-, Review-, epistemic-, deontic-, and temporal-work semantics. The engine executes bounded Agent and Flow runs; it does not become a second orchestration domain model.

## Architecture

The architecture is deliberately split:

```text
WorkWeave Orchestration
        ↓ assignments / execution requests

WorkWeave Engine
  ├─ shared runtime substrate
  ├─ Agent kernel
  └─ Flow kernel
```

The shared runtime owns execution identity, lifecycle, cancellation, audit events, artifacts, storage ports, observability, SDK projections and CLI foundations. It does not own either engine's semantic state machine.

The Agent kernel owns provider-neutral model streaming, messages, tools, attempts, usage, limits, recovery and terminal Agent results.

The Flow kernel owns OWS flow-definition loading, bindings, guards, actions, tokens, joins, retries, waits, signals, timers, checkpoints and deterministic resume.

## Repository map

- `AGENTS.md` — repository operating rules and current refinement locks.
- `CLAUDE.md` — Git symlink to `AGENTS.md`.
- `DECISIONS.md` — accepted and superseded durable direction.
- `QUESTIONS.md` — unresolved project questions.
- `PROJECT_STATE.md` — canonical current state and next work.
- `WARNINGS.md` — anti-drift guardrails.
- `LEARNINGS.md` — accumulated architectural/project learning.
- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md` — proposed engine architecture baseline.
- `docs/architecture/RUST-CONTRACTS.md` — proposed Rust crate and interface contracts.
- `docs/architecture/SOURCE-REGISTER.md` — immutable evidence pins.
- `docs/adr/` — Goal-linked architecture decisions; every Goal must have an ADR before activation.
- `docs/orchestration/` — references canonical WorkWeave Orchestration v0.5; this repository does not fork that model.
- `docs/skills/` — repository-local authoring/refinement procedures.
- `docs/templates/` — reusable Goal/Plan/Task/Decision/Question/Evaluation/Review/ADR authoring supports.
- `goals/` — Goal packets and project bookkeeping.

## Current Goal

`G003 — Durable Agent Kernel` is active under accepted ADR-0003. G002 is achieved after independent owner review.

G003 is intentionally limited to the durable provider-neutral kernel/recovery proof using recorded provider/tool fixtures. T002 provider protocol/assembler, T003 Agent history/recovery reducer, T004 Agent SQLite persistence, T005 atomic common/Agent SQLite transaction coordination, and T006 RecordedProvider conformance are verified.

D021 currently places G003 under `REPLAN_LOCK` while candidate Spec/Plan v2 and explicit T007–T012 acceptance/verification are reviewed. T007 tool contract, schema validation, policy, and replay fixtures remains the next implementation slice, but it must not begin until requester approval and lock removal.

`G004 — Agent Provider and Surface` remains proposed. It adds the first OpenAI-protocol adapter, bounded `fs.read`, and Rust SDK/CLI surface only after G003 is accepted. Deterministic OWS Flow work begins at G005.

`G010 — Durable Storage Evolution and Recovery Hardening` is proposed as a separate, non-blocking home for persistence evolution findings. It is not automatically a prerequisite for G004.
