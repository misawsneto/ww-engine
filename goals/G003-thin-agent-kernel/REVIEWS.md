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

### Requester approval and unlock — 2026-09-04

The requester explicitly approved the complete refined packet and all four final boundary recommendations:

1. linked G002 `ExecutionRecord.deadline` is authoritative; Agent deadline is a matching snapshot only;
2. configured token limits require provider/model normalized usage capability, and missing promised finalized usage fails closed before another request;
3. `max_tool_calls` counts logical model-requested calls and admits a finalized multi-call batch all-or-none before any call executes;
4. an ordinary returned `ToolExecutionError` becomes one durable model-visible error result, while cancellation and panic/impossible invariant failure remain distinct paths.

The approval-reconciliation commit `2cde0a9ceb7abf448bed57cd363065dce5494a22` incorporates those clarifications into SPEC/PLAN/TASKS/VERIFICATION/EVALUATIONS v2 and adds the current Goal-owned implementation orientation at `goals/G003-thin-agent-kernel/HANDOFF.md`. Historical handoffs under `docs/memories/recall/` remain evidence only.

Hosted CI run `33881904717` succeeded on exact approval-reconciliation head `2cde0a9ceb7abf448bed57cd363065dce5494a22`: Format, architecture boundaries, locked Clippy, and full workspace tests all passed. That satisfies the D021 unlock condition. The G003 `REPLAN_LOCK` is removed in the subsequent bookkeeping change; G003 remains active and T007 is implementation-ready.

## D022 T007 dry-run hardening — 2026-09-04

The requester asked for a deeper review of `artifacts/A004-builder-T007-claude-opus-5-dryrun-01.md` against the accepted architecture, especially Pi production and Pi Harness, and authorized a `ww-refine-goal` pass only if the resulting hardening required no domain-model change.

### Domain-model assessment

No domain-model change is required.

The accepted model already contains the relevant semantics: stable tool identity/version, exact parsed arguments, `EffectDescriptor`, `ReplayPolicy`, `PolicyDecision`, `ToolPreparationDisposition`, `ToolPreparationStage`, Agent-owned logical-call/attempt/result identities, `ToolCallPrepared`, `ToolEffectStarted`, effect completion, rejected/denied/completed/interrupted/intervention attempt outcomes, and one model-visible result per logical call. D022 introduces no new entity, state, relationship, lifecycle, authority, or durable record variant.

The hardening is an implementation-architecture clarification inside ADR-0003:

- make Pi's distinct preparation seam explicit as one production `ww-agent-tools::prepare_tool_call` boundary rather than leaving T008 to recompose registry/schema/effect/replay/policy logic;
- keep Agent IDs and durable wrapping in `ww-agent-core`, so the seam does not acquire Agent-domain ownership;
- require an effect/replay-aware policy conformance fixture so the existing policy input is behaviorally proved rather than merely present in types;
- harden canonicalization proof against the current `serde_json::Map` ordering false-green by asserting deterministic nested canonical bytes, not digest equality alone;
- resolve A004's remaining `failed_at` question without adding a new field or record.

This follows Pi's production `prepareToolCall` separation and Harness's durable source/result correlation while retaining WorkWeave's stronger commit-before-effect and replay-intervention semantics. It does not import Pi queues, hooks, parallel execution, session façade, or Harness lanes.

### Q008 resolution

`Q008` records the dry-run question on behalf of `A004-builder`. For policy denial, `failed_at: Policy` lives in `ToolCallPrepared::NoEffect` with `PolicyDecision::Deny`; the existing `ToolAttemptDenied { attempt_id, result_entry_id }` shape remains unchanged. Resolve/Validate/Classify failures continue to use `ToolAttemptRejected.failed_at`.

The actor-identifier question from the dry run was already resolved separately as `A004-builder` in commit `c0c684d580d1e24bb746b7c46b1c7aaa4119639e`.

D022 changes only the open T007 implementation contract and question/evidence bookkeeping. `SPEC v2`, `PLAN v2`, ADR-0003, the G003 Goal boundary, completed T001–T006 semantics, and Task identifiers remain unchanged.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite transaction ownership without Agent DTO leakage into shared store contracts;
- tool argument validation, policy, replay classification, and logical-result uniqueness;
- crash/restart behavior at every model/tool ambiguity boundary;
- cancellation and durable budget accounting;
- absence of concrete transport, filesystem, CLI/product surface, Flow/OWS, or WorkWeave Orchestration semantics in the kernel.
