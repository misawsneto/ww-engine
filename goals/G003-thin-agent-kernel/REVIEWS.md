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

## D021 specification and plan refinement candidate — 2026-09-04

State: candidate Goal packet published; requester approval pending. G003 remains under `REPLAN_LOCK`, so implementation work is still forbidden.

D021 authorized a reference-grounded precision pass before T007 without changing the Goal boundary, accepted ADR-0003, completed T001–T006 semantics, or used Task identities. The lock was published at `999170f895b4a4ecc72615e48b6cd4efba87473e`; the initial candidate packet was published at `403fd72b30ec8afe11112803d5ee06e17217e6d4`; the reviewed candidate basis is `4f49c15c45af544bc8df14a8cbb690c33ec8d38a`.

The review correction at `4f49c15c45af544bc8df14a8cbb690c33ec8d38a` preserves the existing T003 meaning of `ToolAttemptStarted` and `turn_count`. It introduces an explicit prospective `ToolEffectStarted` ambiguity marker for T007/T011 and a separate completed-model-turn budget counter for T010, avoiding retroactive reinterpretation of completed durable-state semantics.

Method and evidence basis:

- `addyosmani/agent-skills@1c760d643497e9da289300e5eb2f5aca861503f7`: `spec-driven-development` blob `f3f5877c5d6be8f74408c308393bfb45cbcf53c4` and `planning-and-task-breakdown` blob `296249b64334bcfd1aeaefd27b9e3e5494e38ec0`;
- pinned Pi revision `6c87d9a026677b601e8278030dcf1ad97fe0bd86`: production provider/tool-loop seams and future-Harness entry/record/reducer durability evidence;
- accepted ADR-0003 plus the WorkWeave Engine architecture dossier and current Rust contracts;
- current implementation basis through T006 on `main`.

Refined records:

- `SPEC.md` candidate v2 defines normative architecture, ownership, ordering, failure behavior, durable boundaries, exact limit semantics, F1–F8 recovery states, testing strategy, and requirement traceability for T007–T012;
- `PLAN.md` candidate v2 preserves T007→T012 and adds dependency-ordered work units, likely files, checkpoints, risks, and escalation rules;
- `TASKS.md` preserves completed rows and identities while making each open Task executable through descriptions, work units, acceptance criteria, focused commands, likely files, and scope guidance;
- `VERIFICATION.md` adds stable Task-scoped checks for tool safety, kernel execution, lifecycle/cancellation, limits, recovery, and final review while retaining T002–T006 evidence;
- `EVALUATIONS.md` defines exact deterministic procedures, expected results, and EvaluationRun evidence fields.

Key candidate refinements include an offline non-coercing Draft 2020-12 schema profile, one authoritative parsed argument value, deterministic argument/request digests, stable tool/attempt/reserved-result identities, distinct durable handling/effect-start boundaries, durable pre-effect classification and policy, sequential source-order execution, mandatory provider-stream EOF finalization, explicit lifecycle/cancellation and limit boundaries, and a distinct-process F1–F8 restart matrix.

Deliberately deferred rather than added to G003: generic durable-format migration infrastructure, cross-adapter storage hardening, approval-bearing policy, idempotency-key replay, parallel tools, concrete providers, filesystem/process/network capabilities, SDK/CLI/TUI/server surfaces, and Flow/Orchestration semantics.

No implementation source, dependency, lockfile, or completed Task evidence was changed by this candidate. The requester must approve or reject the complete refined packet before the lock can be removed and T007 implementation can resume.

### D021 reference-grounding amendment — 2026-09-04

The requester asked for the open Tasks and specifications to correlate more directly with the architectural and design principles observed throughout the reference investigation. Commit `a901b987d267e968207f2dfc241e6246e938c9be` deepens the candidate without changing its Goal boundary, canonical T001–T012 sequence, completed T001–T006 semantics, or implementation state.

The amendment makes the design lineage operational for every open Task:

- T007 now distinguishes tools-owned execution inputs from Agent-owned durable identities, preventing a `ww-agent-tools` → `ww-agent-core` dependency cycle while retaining Pi's validate/preflight/ordered-result seams and Harness's durable correlation evidence in core;
- T007 also separates resolve/validate/classify rejection from an actual centralized policy denial, preserving a useful failure taxonomy instead of overloading `ToolAttemptDenied`;
- T008 covers both Pi-style in-stream failure and the existing Rust provider port's outer dispatch error, and requires each mutation/external-work decision to be authorized by one committed expected-version transition;
- T009/T010 define the last pre-effect cancellation check and a stable recovery precedence so cancellation, deadline, or budget state cannot erase a started Never-effect ambiguity;
- T011 expands F2 to process loss before any event and after non-durable partial deltas, and requires a competing stale resumer to launch no external work;
- T012 requires explicit evidence of what was observed, adopted, adapted, rejected, or deferred from Pi production, Pi Harness, LangGraph, OWS, and the WorkWeave dossier.

The amended `SPEC.md`, `PLAN.md`, `TASKS.md`, `VERIFICATION.md`, and `EVALUATIONS.md` remain specification/planning-only changes permitted by the active `REPLAN_LOCK`. No dependency, source, lockfile, migration, Task identity, or completed evidence changed. Requester approval remains pending.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite transaction ownership without Agent DTO leakage into shared store contracts;
- tool argument validation, policy, replay classification, and logical-result uniqueness;
- crash/restart behavior at every model/tool ambiguity boundary;
- cancellation and durable budget accounting;
- absence of concrete transport, filesystem, CLI/product surface, Flow/OWS, or WorkWeave Orchestration semantics in the kernel.
