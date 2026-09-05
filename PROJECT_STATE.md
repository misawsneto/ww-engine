# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G003 — Durable Agent Kernel`.
- G001 architecture baseline: achieved and accepted.
- G002 shared runtime: achieved and accepted on 2026-09-02 under `ADR-0002`.
- G003 remains active under accepted `ADR-0003` and narrowed to the durable provider-neutral kernel/recovery proof.
- T002–T006 are complete and verified; their semantics/evidence are frozen.
- D021 established the approved v2 planning basis and four accepted boundary clarifications: common deadline authority, usage-observable token limits, whole logical-tool-batch admission, and distinct ordinary-error/cancellation/invariant paths.
- D022 was resumed after review found the first D022 pass had placed material architecture requirements only in `TASKS.md`, contrary to the repository authority hierarchy and `ww-refine-goal` completion rules.
- The G003 `REPLAN_LOCK` is active under D022. Implementation is blocked until the corrected packet is approved and the lock is removed.
- Candidate planning basis: `SPEC v3-candidate`, `PLAN v3-candidate`, reconciled T007/T008 Tasks, `VERIFICATION v3-candidate`, `EVALUATIONS v3-candidate`, and the updated Goal handoff.
- The corrected boundary keeps T007 responsible for pure tool preparation + durable tool grammar/reducer and T008 responsible for the real commit-before-effect execution proof.
- Tool execution cancellation is now a machine-distinguishable control outcome, not an ordinary `ToolExecutionError`; T009 owns durable cancellation intent and replay-sensitive final settlement.
- JSON Schema Draft 2020-12 offline rules cover both `$ref` and `$dynamicRef`; `$id` alone is not treated as an external retrieval request.
- Run configured tool pin order is authoritative; registry registration order has no model-visible authority.
- Q008 remains resolved for A004-builder: Policy stage belongs in `ToolCallPrepared::NoEffect`; `ToolAttemptDenied` gains no duplicate stage field.
- T007 remains the next implementation Task **after requester approval and unlock**.
- Proposed following Goal: `G004 — Agent Provider and Surface`; first concrete provider, bounded `fs.read`, SDK/CLI; proposed `ADR-0004`.
- G005 is deterministic OWS Flow kernel; G006 is restart-safe Flow → Agent integration.
- `G010 — Durable Storage Evolution and Recovery Hardening` remains proposed and non-blocking unless later evidence activates it.

## Goal ADR rule

Every Goal must reference at least one ADR before activation. Material architecture changes during an active Goal require the governing ADR to be amended/superseded before reliance. D022 does not change ADR-0003; it reconciles contracts already inside it.

## Current evidence pins

- Pi reference: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.
- Refinement method: `docs/skills/ww-refine-goal/SKILL.md`, introduced by D020 and used by D021/D022.

## G002 achieved boundary

Final reviewed G002 implementation code head `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` is covered by CI run `33646651848` through evidence commit `bb2cb831fe42342afcfc93cf7e8757a9206c1947`.

## G003 execution status

```text
T001 activation/bookkeeping              complete
T002 provider protocol/assembler          complete / verified
T003 Agent history + recovery reducer     complete / verified
T004 Agent SQLite persistence             complete / verified
T005 common/Agent transaction seam        complete / verified
T006 recorded provider                    complete / verified
T007 tool preparation + durable grammar   open / NEXT AFTER D022 APPROVAL
T008 functional Agent kernel              open
T009 lifecycle + cancellation             open
T010 durable limits                       open
T011 crash/restart matrix                 open
T012 evaluations + terminal review        open
```

Task IDs and dependency order remain unchanged.

## Current G003 planning basis

```text
ADR-0003              accepted / unchanged
GOAL                   unchanged
SPEC v3-candidate      pending D022 requester approval
PLAN v3-candidate      pending D022 requester approval
TASKS                  reconciled to v3 candidate; IDs unchanged
VERIFICATION v3        candidate with stable D022 checks
EVALUATIONS v3         candidate with preparation/effect-boundary split
HANDOFF                 aligned to candidate authority
Q008                    resolved
REPLAN_LOCK             ACTIVE — D022
next implementation     T007 after approval/unlock
```

## Planned implementation sequence

```text
G002 Shared Runtime                 achieved
  ↓
G003 Durable Agent Kernel           ACTIVE — D022 replanning lock
  ↓
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

G010 remains proposed outside this sequence.
