# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G003 — Durable Agent Kernel`.
- G001 architecture baseline: achieved and accepted.
- G002 shared runtime: achieved after independent owner review acceptance on 2026-09-02; governed by `ADR-0002`.
- G003 is active under accepted `ADR-0003` and remains narrowed to the durable provider-neutral kernel/recovery proof.
- G003 T002 through T006 are complete and verified.
- T005 proved atomic common-execution + Agent-run creation/linkage and rollback on injected mid-write failure without adding Agent DTOs to the shared `ww-store` semantic API.
- D021 refinement is requester-approved: `SPEC v2`, `PLAN v2`, open T007–T012 detail, Verification, and Evaluations are now the accepted planning basis.
- The four final D021 boundary clarifications are accepted: common deadline authority, usage-observable token limits, all-or-none logical tool-call batch admission, and distinct ordinary-tool-error/cancellation/invariant semantics.
- G003 remains temporarily under `REPLAN_LOCK` only until the exact approval-reconciliation head passes the complete permanent D017 gate. The Goal state remains active and implementation is still blocked while the lock exists.
- T007 remains the next engineering slice after unlock: tool contract, schema validation, policy, and replay fixtures; it converges with the completed provider work at T008, the functional model → tool → model kernel.
- Current implementation orientation: `goals/G003-thin-agent-kernel/HANDOFF.md`.
- Proposed following Goal: `G004 — Agent Provider and Surface`; OpenAI protocol, bounded `fs.read`, Agent SDK/CLI; governed by proposed `ADR-0004`.
- Deterministic OWS Flow kernel follows as G005; restart-safe Flow → Agent integration follows as G006.
- `G010 — Durable Storage Evolution and Recovery Hardening` is proposed as a separate home for non-blocking persistence evolution findings. It is not a prerequisite for G004 unless later evidence activates it for a specific reason.

## Goal ADR rule

Every Goal must reference at least one ADR before activation. G001/G002 ADRs were backfilled when this rule was introduced; future Goals must create their ADRs while still proposed.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.
- Refinement method: approved `docs/skills/ww-refine-goal/SKILL.md`, introduced by D020 and invoked for G003 by D021.

## G002 achieved boundary

Final reviewed implementation code head `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` is covered by CI run `33646651848` through evidence commit `bb2cb831fe42342afcfc93cf7e8757a9206c1947`. The project owner independently reviewed and approved G002 on 2026-09-02.

## G003 execution status

```text
T001 activation/bookkeeping              complete
T002 provider protocol/assembler          complete / verified
T003 Agent history + recovery reducer     complete / verified
T004 Agent SQLite persistence             complete / verified
T005 common/Agent transaction seam        complete / verified
T006 recorded provider                    complete / verified
T007 tool policy/replay fixtures          open / REPLAN_LOCK pending exact-head gate
T008 functional Agent kernel              open / REPLAN_LOCK pending exact-head gate
T009 lifecycle + cancellation             open / REPLAN_LOCK pending exact-head gate
T010 durable limits                       open / REPLAN_LOCK pending exact-head gate
T011 crash/restart matrix                 open / REPLAN_LOCK pending exact-head gate
T012 evaluations + terminal review        open / REPLAN_LOCK pending exact-head gate
```

The Task IDs, meanings, and dependencies remain the original G003 sequence. The lock changes only whether implementation may proceed while the approved planning basis completes its final verification gate.

## Current G003 planning basis

```text
ADR-0003 accepted
GOAL unchanged
SPEC v2              approved under D021
PLAN v2              approved, based on SPEC v2
TASKS T007-T012      refined, IDs unchanged, approved
VERIFICATION         requirement-traceable v2 approved
EVALUATIONS          executable v2 approved
HANDOFF.md            current builder orientation
requester approval   complete 2026-09-04
REPLAN_LOCK           active only pending exact-head D017 verification
```

## Planned implementation sequence

```text
G002 Shared Runtime                 achieved
  ↓
G003 Durable Agent Kernel           ACTIVE / temporary REPLAN_LOCK
  ↓ after exact-head gate and unlock
G004 Agent Provider and Surface
  ↓
G005 Deterministic OWS Flow Kernel
  ↓
G006 Flow → Agent integration
  ↓
G007 Full frozen OWS profile
  ↓
G008 Local product experience
  ↓
G009 Coordinated deployment          reserved
```

`G009 — Coordinated Deployment` is reserved by the original architecture roadmap; D015 shifted it by one when the Agent Provider and Surface Goal was inserted.

G010 is proposed outside this sequence. Its activation and execution order will be decided after G003 from concrete requirements rather than inferred from review findings alone.
