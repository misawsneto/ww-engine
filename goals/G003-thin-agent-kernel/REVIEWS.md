# G003 Reviews

## Implementation progress review — 2026-09-03

State: active. No terminal G003 review has occurred.

Verified slices:

- T002 provider-neutral protocol and pure stream assembler: provider boundary, clippy and workspace tests passed; 15 assembler/conformance tests passed.
- T003 durable Agent entries/operational records and pure recovery reducer: clippy and workspace tests passed; 11 recovery/corruption tests passed.
- T004 Agent-owned SQLite persistence/reconstruction: clippy and workspace tests passed, including rollback/reopen/version-conflict and real OS-process restart reconstruction.

Current checkpoint:

- T006 RecordedProvider conformance is complete and verified. The full `main` gate passed on Rust 1.98.0: rustfmt, five architecture-boundary checks, clippy with `--locked -D warnings`, and 58/58 tests.
- T005 remains scoped only to atomic common-execution + Agent-run creation/linkage and rollback on injected half-write failure. Terminal repair remains owned by T009.
- T007 tool contract, schema validation, policy, and replay fixtures is the next open implementation slice.

## D018 experiment disposition — 2026-09-03

D018 introduced a durability-hygiene cleanup gate after T006. The implementation produced useful evidence, but review found that inserting and renumbering Tasks changed the accepted G003 structure without an existing acceptance criterion or Stop Condition requiring the interruption. D019 therefore superseded D018, and the D018-era implementation was ordinarily reverted without rewriting history. The canonical G003 sequence remains T001–T012, with T007 next. Technical findings are mapped either to an existing G003 Task or proposed G010; the full evidence and disposition remain in `docs/memories/recall/D018-DURABILITY-HYGIENE-RETROSPECTIVE-2026-09-03.md`.

## D021 specification and plan refinement — 2026-09-04

D021 authorized a reference-grounded precision pass before T007 without changing the Goal boundary, accepted ADR-0003, completed T001–T006 semantics, or used Task identities. The lock was published at `999170f895b4a4ecc72615e48b6cd4efba87473e`; the initial candidate packet was published at `403fd72b30ec8afe11112803d5ee06e17217e6d4`; later review/grounding amendments culminated in candidate head `63c8f5cd7e9223b0614e4b9dbce39bc884c831fd`.

The review corrections preserve the existing T003 meaning of `ToolAttemptStarted`, `turn_count`, and `tool_attempt_count`. They add prospective T007/T010 concepts—an explicit `ToolEffectStarted` ambiguity marker, a completed-model-turn counter, and a logical-tool-call budget counter—rather than retroactively reinterpreting completed durable-state semantics.

Method and evidence basis:

- `addyosmani/agent-skills@1c760d643497e9da289300e5eb2f5aca861503f7`: `spec-driven-development` blob `f3f5877c5d6be8f74408c308393bfb45cbcf53c4` and `planning-and-task-breakdown` blob `296249b64334bcfd1aeaefd27b9e3e5494e38ec0`;
- pinned Pi revision `6c87d9a026677b601e8278030dcf1ad97fe0bd86`: production provider/tool-loop seams and future-Harness entry/record/reducer durability evidence;
- accepted ADR-0003 plus the WorkWeave Engine architecture dossier and current Rust contracts;
- current implementation basis through T006 on `main`.

Refined records:

- `SPEC.md` v2 defines normative architecture, ownership, ordering, failure behavior, durable boundaries, exact limit semantics, F1–F8 recovery states, testing strategy, and requirement traceability for T007–T012;
- `PLAN.md` v2 preserves T007→T012 and adds dependency-ordered work units, likely files, checkpoints, risks, and escalation rules;
- `TASKS.md` preserves completed rows and identities while making each open Task executable through descriptions, work units, acceptance criteria, focused commands, likely files, and scope guidance;
- `VERIFICATION.md` adds stable Task-scoped checks for tool safety, kernel execution, lifecycle/cancellation, limits, recovery, and final review while retaining T002–T006 evidence;
- `EVALUATIONS.md` defines exact deterministic procedures, expected results, and EvaluationRun evidence fields.

Deliberately deferred rather than added to G003: generic durable-format migration infrastructure, cross-adapter storage hardening, approval-bearing policy, idempotency-key replay, parallel tools, concrete providers, filesystem/process/network capabilities, SDK/CLI/TUI/server surfaces, and Flow/Orchestration semantics.

### Requester approval — 2026-09-04

The requester explicitly approved the complete refined packet and all four final boundary recommendations:

1. linked G002 `ExecutionRecord.deadline` is authoritative; Agent deadline is a matching snapshot only;
2. configured token limits require provider/model normalized usage capability, and missing promised finalized usage fails closed before another request;
3. `max_tool_calls` counts logical model-requested calls and admits a finalized multi-call batch all-or-none before any call executes;
4. an ordinary returned `ToolExecutionError` becomes one durable model-visible error result, while cancellation and panic/impossible invariant failure remain distinct paths.

These clarifications are incorporated into SPEC/PLAN/TASKS/VERIFICATION/EVALUATIONS v2. They do not change the Goal boundary or Task identities.

A current Goal-owned implementation orientation is now `goals/G003-thin-agent-kernel/HANDOFF.md`. Historical handoffs under `docs/memories/recall/` remain evidence only.

The candidate head `63c8f5cd7e9223b0614e4b9dbce39bc884c831fd` already passed hosted CI run `33879580352`. The approval-reconciliation commit must also pass the complete permanent gate on its exact head before `REPLAN_LOCK` is removed. Until that verification completes, implementation remains blocked.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite transaction ownership without Agent DTO leakage into shared store contracts;
- tool argument validation, policy, replay classification, and logical-result uniqueness;
- crash/restart behavior at every model/tool ambiguity boundary;
- cancellation and durable budget accounting;
- absence of concrete transport, filesystem, CLI/product surface, Flow/OWS, or WorkWeave Orchestration semantics in the kernel.
