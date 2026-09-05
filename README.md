# WorkWeave Engine

WorkWeave Engine is a Rust execution platform with two independent execution kernels on one shared operational substrate:

- **WorkWeave Agent** — a bounded probabilistic model/tool worker;
- **WorkWeave Flow** — a deterministic durable OWS workflow worker.

WorkWeave Orchestration remains above the engine and owns Goal/Task/Question/Decision/Evaluation/Review plus epistemic/deontic/temporal work semantics. The engine executes bounded Agent and Flow runs; it does not become a second orchestration domain model.

## Architecture

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
- `DECISIONS.md` — accepted/superseded durable direction.
- `QUESTIONS.md` — project questions and resolutions.
- `PROJECT_STATE.md` — canonical current state and next work.
- `WARNINGS.md` — anti-drift guardrails.
- `LEARNINGS.md` — accumulated learning.
- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md` — architecture baseline.
- `docs/architecture/RUST-CONTRACTS.md` — Rust contract baseline.
- `docs/architecture/SOURCE-REGISTER.md` — immutable evidence pins.
- `docs/adr/` — Goal-linked architecture decisions.
- `docs/orchestration/` — references canonical WorkWeave Orchestration; this repository does not fork that model.
- `docs/skills/` — repository-local procedures.
- `docs/templates/` — reusable authoring supports.
- `goals/` — Goal packets and bookkeeping.

## Current Goal

`G003 — Durable Agent Kernel` is active under accepted ADR-0003. G002 is achieved after independent owner review.

T002 provider protocol/assembler, T003 Agent history/recovery reducer, T004 Agent SQLite persistence, T005 atomic common/Agent transaction coordination, and T006 RecordedProvider conformance are complete and verified.

D021 established the approved v2 basis. D022 was later resumed after review found the first dry-run hardening pass had placed material architecture requirements only in lower-authority `TASKS.md` and unlocked before reconciling SPEC/PLAN/V&V.

The **G003 D022 `REPLAN_LOCK` is active**. Implementation is blocked until the corrected packet is approved and the lock is removed.

The candidate corrected basis is:

- `SPEC v3-candidate`;
- `PLAN v3-candidate`;
- reconciled open Tasks;
- `VERIFICATION v3-candidate` with stable D022 checks;
- `EVALUATIONS v3-candidate`;
- updated `HANDOFF.md`.

The corrected boundary keeps T007 focused on tool preparation + durable tool grammar/reducer, while T008 owns the real production commit-before-effect execution proof. Tool cancellation is machine-distinguishable from ordinary tool error, Draft 2020-12 offline validation covers `$ref` and `$dynamicRef`, and run configured tool-pin order—not registry insertion order—is authoritative.

After requester approval and unlock, T007 is the next implementation Task.

`G004 — Agent Provider and Surface` remains proposed. It adds the first concrete provider, bounded `fs.read`, and Rust SDK/CLI surface only after G003 is accepted. Deterministic OWS Flow work begins at G005.

`G010 — Durable Storage Evolution and Recovery Hardening` remains a proposed non-blocking home for persistence evolution findings.
