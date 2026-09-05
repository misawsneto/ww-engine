# G003 Reviews

## Implementation progress review — 2026-09-03

State: active. No terminal G003 review has occurred.

Verified foundation:

- T002 provider-neutral protocol/stream assembler complete and verified.
- T003 durable Agent entries/operational records + pure recovery reducer complete and verified.
- T004 Agent-owned SQLite persistence/reconstruction complete and verified.
- T005 common/Agent creation/link coordination complete and verified; terminal repair remains T009.
- T006 RecordedProvider complete and verified under the full permanent gate; T007 remains the next open implementation Task.

## D018 experiment disposition — 2026-09-03

D018 inserted a durability/hygiene gate after T006. The implementation produced useful findings but changed the accepted G003 Task structure without a Stop Condition or acceptance requirement demanding the interruption. D019 superseded D018, restored T001–T012, and retained the technical findings as historical evidence. Non-blocking persistence hardening moved to proposed G010 rather than becoming a prerequisite cleanup cycle.

See `docs/memories/recall/D018-DURABILITY-HYGIENE-RETROSPECTIVE-2026-09-03.md`.

## D021 specification and plan refinement — 2026-09-04

D021 authorized a reference-grounded precision pass before T007 while preserving ADR-0003, Goal boundary, T001–T006 semantics, and Task IDs.

Evidence basis included:

- pinned Addy Osmani spec-driven-development and planning/task-breakdown skills;
- pinned Pi production Agent seams and future Harness durability evidence;
- ADR-0003, Engine architecture dossier, and current Rust contracts.

D021 produced approved SPEC/PLAN v2, refined open Tasks, requirement-traceable Verification, executable Evaluations, and the current Goal-owned handoff. The requester also approved four final boundaries: common deadline authority, usage-observable token limits, whole logical-tool-batch admission, and distinct ordinary-tool-error/cancellation/invariant semantics.

Approval-reconciliation head `2cde0a9ceb7abf448bed57cd363065dce5494a22` passed hosted CI run `33881904717`, after which the D021 lock was removed.

## A004 T007 dry run — 2026-09-04

A004-builder recorded `artifacts/A004-builder-T007-claude-opus-5-dryrun-01.md` before implementation.

Useful findings:

- canonical-digest tests can false-green under current `serde_json::Map` ordering unless canonical bytes are asserted directly;
- `jsonschema 0.52.1` with default features disabled supports the intended offline profile;
- explicit external-reference rejection is needed for clear WorkWeave-owned schema errors;
- existing `AgentAppend` already supports atomic multi-record appends under one optimistic version;
- the preparation/execution boundary should follow Pi's distinct preparation seam;
- Q008 asked where Policy `failed_at` belongs.

Q008 is resolved in `QUESTIONS.md`: Policy stage belongs in `ToolCallPrepared::NoEffect` with durable Deny; `ToolAttemptDenied` gains no duplicate stage field.

## D022 first pass and critique — 2026-09-04

D022 authorized T007 hardening after the dry run without changing the domain model, ADR-0003, Goal boundary, completed Tasks, or Task IDs.

The first pass correctly added a single preparation seam direction, effect/replay-aware policy proof, canonical-byte proof, and Q008 resolution. It then unlocked at `790cf17e5fa1fbce7e8bd0449e3ab28485db0a92` after green CI.

A subsequent independent critique found the unlock premature. The critique was accepted because:

1. material requirements existed only in lower-authority `TASKS.md` while SPEC/PLAN/Verification/Evaluations remained v2;
2. `ww-refine-goal` requires a reconciled complete packet before unlock;
3. T007 preparation cannot by itself prove real commit-before-effect execution; that production proof belongs in T008;
4. tool cancellation behavior was semantically specified but not representable distinctly from ordinary `ToolExecutionError` at the tools/core API boundary;
5. exact function/module naming in TASKS contradicted SPEC's implementation-flexibility rule;
6. D022 requirements lacked stable Verification IDs and Evaluation coverage;
7. the Draft 2020-12 offline profile needed `$dynamicRef` coverage while not treating `$id` itself as external retrieval;
8. configured run pin order needed proof independent of registry registration order.

This is a planning-authority/process defect, not a domain-model or ADR change.

## D022 resumed reconciliation — 2026-09-04

The D022 lock was restored at `e26bcc3737a5f78e906bd22c64d09bcf490be2e4` before further planning mutations.

The v3 draft reconciliation now:

- places material tool architecture in `SPEC v3` with draft state separate from version identity;
- reconciles `PLAN v3`, Tasks, Verification, Evaluations, handoff, Project State, and README;
- keeps one production tool-preparation seam semantically mandatory while allowing idiomatic function/module naming;
- makes run configured pin order authoritative and proves it against different registration order;
- expands offline schema rules to non-fragment `$ref` and `$dynamicRef`; `$id` alone is not rejected;
- requires effect/replay-aware policy conformance and nested canonical-byte proof;
- makes Output / OrdinaryError / Cancelled machine-distinguishable normal tool outcomes; panic/invariant remains outside normal outcomes;
- keeps T007 responsible for pure preparation + durable grammar/reducer;
- moves actual `ToolEffectStarted` commit-before-executor proof to T008;
- retains Q008 without adding a new record field;
- preserves published Verification identifiers rather than silently repurposing them.

### Identity/version reconciliation

Review of the v2→v3 Verification diff found that `V-T007-18`, `V-T007-21`, `V-T007-22`, and `V-T007-23` had been reused for materially different propositions. They are now treated as consumed v2 identifiers with explicit mappings to the current proof checks. New propositions use new check IDs.

The generic lesson was added to `ww-refine-goal`: explicit identifiers remain attached to the same semantic subject; materially changed propositions require a new identifier or explicit supersession/replacement mapping; title-addressed record families do not gain artificial IDs; version identity and lifecycle state remain separate.

G003 now uses `v3` as the planning-generation identity and `draft` as its lifecycle state. The approved implementation basis remains v2 until requester approval promotes the same v3 generation.

### A004 follow-up ownership review

A004 then found one blocking ownership contradiction: a tools-side preparation seam could not return `ToolPreparationDisposition` if that type were core-owned, because SPEC also forbids `ww-agent-tools → ww-agent-core` dependency and D022 forbids a duplicate public/durable preparation taxonomy.

The resolved architecture is now explicit in SPEC v3:

```text
ww-agent-tools
  owns ToolPreparationDisposition
  owns ToolPreparationStage
  owns the single preparation seam
        ↓
ww-agent-core
  depends on ww-agent-tools
  embeds those exact tools-owned values in Agent-owned durable records
```

The phrase “Agent-owned record shapes” now refers to the durable record containers, not ownership of every field type inside them. Core MUST NOT redefine equivalent preparation enums.

A004 also correctly noted that “metadata omitted” was not a meaningful runtime conformance case because `ToolPolicyInput.effect` and `.replay` are non-optional. The v3 contract now treats omission as a structural/type-level guarantee; behavioral conformance proves exact classified values reach policy and substitution changes behavior where expected.

No new Goal, Task, Decision, domain entity, durable record variant, or dependency direction is introduced by these clarifications.

### D022 approval and unlock — 2026-09-05

The requester approved the complete corrected v3 packet and directed removal of the lock. `SPEC v3`, `PLAN v3`, reconciled Tasks, `VERIFICATION v3`, and `EVALUATIONS v3` are promoted from draft to approved without minting a new version. The G003 `REPLAN_LOCK` is removed and T007 implementation may begin.

The v3 correction resolved two findings raised by `A004-builder` in the T007 dry run: preparation-type ownership now sits in `ww-agent-tools` with core embedding those exact types (`V-T007-41`), and the seam/module naming mandate was demoted from acceptance requirement to convention in SPEC §6.1. The review additionally found a false-green class the dry run missed: registration order matching configured order, now proved distinct by `V-T007-04`.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- one tools preparation authority with tools-owned disposition/stage types and no core duplication;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite ownership without Agent DTO leakage;
- exact arguments, configured tool order, policy/replay classification, and result uniqueness;
- real commit-before-effect ordering in T008;
- cancellation distinct from ordinary tool error and replay-sensitive settlement;
- crash/restart behavior at every ambiguity boundary;
- absence of concrete transport/capability/product/Flow/Orchestration leakage.
