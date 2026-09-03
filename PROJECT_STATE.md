# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G003 — Durable Agent Kernel`.
- G001 architecture baseline: achieved and accepted.
- G002 shared runtime: achieved after independent owner review acceptance on 2026-09-02; governed by `ADR-0002`.
- G003 is active under accepted `ADR-0003` and is narrowed to the durable provider-neutral kernel/recovery proof.
- G003 T002, T003, T004, T005, and T006 are complete and verified.
- T005 proved atomic common-execution + Agent-run creation/linkage and rollback on injected mid-write failure without adding Agent DTOs to the shared `ww-store` semantic API.
- T006 added the deterministic RecordedProvider conformance harness; the full `main` gate passed at 58/58 tests.
- D018 is accepted: before tool-contract/replay work continues, G003 Plan v2 executes a bounded durability/hygiene cleanup gate T007–T011.
- Next engineering slice: T007 durable schema and migration evolution.
- Proposed following Goal: `G004 — Agent Provider and Surface`; OpenAI protocol, bounded `fs.read`, Agent SDK/CLI; governed by proposed `ADR-0004`.
- Deterministic OWS Flow kernel follows as G005; restart-safe Flow → Agent integration follows as G006.

## Goal ADR rule

Every Goal must reference at least one ADR before activation. G001/G002 ADRs were backfilled when this rule was introduced; future Goals must create their ADRs while still proposed.

D018 does not reopen ADR-0003. The Plan v2 cleanup gate tightens the existing durability, recovery, and ownership guarantees before additional persistent tool semantics are introduced.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.

## G002 achieved boundary

Final reviewed implementation code head `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` is covered by CI run `33646651848` through evidence commit `bb2cb831fe42342afcfc93cf7e8757a9206c1947`. The project owner independently reviewed and approved G002 on 2026-09-02.

## G003 execution status — Plan v2

```text
T001 activation/bookkeeping                         complete
T002 provider protocol/assembler                    complete / verified
T003 Agent history + recovery reducer               complete / verified
T004 Agent SQLite persistence                       complete / verified
T005 common/Agent transaction seam                  complete / verified
T006 recorded provider                              complete / verified
T007 durable schema + migration evolution           open / NEXT
T008 idempotent coordination + typed store errors   open
T009 durable ownership + canonical tool arguments   open
T010 provider stream finalization path              open
T011 physical-backend/repository hygiene            open
T012 tool policy/replay fixtures                    open
T013 functional Agent kernel                        open
T014 lifecycle + cancellation                       open
T015 durable limits                                 open
T016 crash/restart matrix                           open
T017 evaluations + terminal review                  open
```

Historical G003 records written before Plan v2 keep their original task numbering. Plan v2 preserves completed T001–T006 and renumbers only the former open T007–T012 sequence.

## Planned implementation sequence

```text
G002 Shared Runtime                 achieved
  ↓
G003 Durable Agent Kernel           ACTIVE
  ├─ T007–T011 durability/hygiene gate
  ├─ T012 tool/replay contract
  ├─ T013 functional kernel
  └─ T014–T017 lifecycle/recovery/acceptance
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
```
