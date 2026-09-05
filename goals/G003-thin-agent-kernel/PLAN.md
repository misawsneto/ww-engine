# Plan — G003 Durable Agent Kernel

- Version: `v3`
- State: `draft`
- Approval: `pending requester approval under resumed D022`
- Specification basis: `G003 SPEC v3 (draft)`
- Goal state: `active`
- Implementation readiness: blocked while G003 `REPLAN_LOCK` is present
- Task topology: unchanged — `T007 → T008 → T009 → T010 → T011 → T012`

## 1. Planning contract

This plan reconciles the A004 dry-run findings and subsequent critique without creating new Tasks, Goals, ADRs, or domain concepts.

Rules:

- completed T001–T006 remain unchanged;
- work units are checkpoints inside existing Tasks, not new Task identities;
- architecture contracts come from SPEC; Tasks define delivery; Verification/Evaluations define proof;
- reference projects are evidence, not implementation authority;
- prefer deletion and reuse over new layers;
- roughly five implementation files per work unit is a decomposition signal, not a gate.

## 2. Dependency graph

```text
T002 provider protocol ─→ T006 RecordedProvider ──────────────┐
                                                              │
T003 durable model ─→ T004 store ─→ T005 coordinator ────────┼─→ T008 kernel
                                                              │       ↓
T007 tools preparation + durable grammar ─────────────────────┘     T009 lifecycle/cancel
                                                                      ↓
                                                                   T010 limits
                                                                      ↓
                                                                   T011 restart matrix
                                                                      ↓
                                                                   T012 evaluations/review
```

T008 does not start until T007 is complete and verified.

## 3. Implementation strategy

### Phase A — T007: tool preparation and durable grammar

T007 proves **what a tool call means and whether it is executable**. It does not claim production commit-before-effect execution.

#### Work unit A — identity, schema, and configured-order projection

Deliver:

- `ww-agent-tools` crate with allowed dependencies only;
- stable tool identity/version/spec contracts;
- immutable duplicate-rejecting registry;
- exact-pin lookup/projection driven by an explicit ordered run pin list;
- registration order deliberately different from configured order in conformance tests;
- Draft 2020-12 reusable validator;
- non-fragment `$ref` and `$dynamicRef` rejection before compile;
- `$id` accepted as metadata/base identity without enabling external retrieval;
- exact non-coercing validation.

Checkpoint:

```bash
cargo test -p ww-agent-tools --locked
# then permanent D017 gate
```

#### Work unit B — canonical arguments and the single preparation seam

Deliver one production `ww-agent-tools` preparation boundary with this semantic order:

```text
resolve
→ validate
→ canonical bytes/digest
→ effect
→ replay
→ policy
→ ToolPreparationDisposition::Executable | NoEffect
```

Requirements:

- `ToolPreparationDisposition` and `ToolPreparationStage` are defined in `ww-agent-tools`;
- core embeds those exact tools-owned types and does not define a second taxonomy;
- later stages do not run after earlier failure;
- core does not duplicate preparation;
- nested canonical bytes are tested directly;
- policy input structurally requires effect/replay metadata;
- behavioral policy proof covers exact classified values and substitution sensitivity;
- function/module naming follows normal Rust conventions; do not create a manager/service layer merely to name the seam.

Checkpoint: preparation conformance target + permanent gate.

#### Work unit C — execution contract and fixtures

Deliver:

- `test.echo` and `test.unsafe_once`;
- injected effect probe;
- tool execution API with machine-distinguishable Output / OrdinaryError / Cancelled semantics;
- panic/invariant remains outside normal outcomes;
- direct fixture tests prove execution behavior only, not Agent durability ordering.

#### Work unit D — Agent-owned durable grammar and reducer

Deliver the already-approved record/reducer vocabulary:

- Agent-owned durable records embedding the tools-owned preparation disposition/stage types;
- reserved result identity;
- effect-start ambiguity marker;
- effect completion;
- rejected/denied/completed/interrupted/intervention attempt states;
- Q008 taxonomy exactly as SPEC v3 §7.3;
- corruption tests for conflicting metadata, wrong result identity, illegal effect start/completion, duplicate result, and source-order violations;
- reopen reconstruction proof.

T007 tests may construct histories directly. Do not build the T008 kernel to make reducer tests easier.

#### T007 closure checkpoint

T007 closes only when:

- every `V-T007` check passes;
- the tools crate has one preparation seam and no core dependency;
- preparation disposition/stage ownership matches SPEC v3 and no duplicate taxonomy exists;
- the reducer reconstructs/corrupt-checks the full durable grammar;
- no production effect-order claim is attributed to T007;
- no G004 capability or T008 loop is introduced;
- focused tests and full D017 gate pass on the closure commit.

### Phase B — T008: functional kernel and real effect boundary

T008 proves **when an executable call may actually run**.

#### Work unit A — request builder and model-attempt finalization

Deliver:

- typed run configuration;
- deterministic request construction from durable entries;
- provider-visible tool specs in exact run pin order, independent of registry registration order;
- request digest/provider/model attempt state before provider I/O;
- one production stream drain → EOF → `finish()` path;
- immutable finalized assistant persistence;
- typed outer provider-dispatch failure.

#### Work unit B — no-effect settlement

Kernel calls the single T007 preparation seam and consumes the tools-owned `ToolPreparationDisposition`.

If `NoEffect`:

```text
persist handling start
+ ToolCallPrepared::NoEffect
+ exactly one reserved model-visible error entry
+ Rejected or Denied terminal attempt record
→ commit
→ execute zero times
```

This work unit proves actual result persistence for invalid/unknown/classification/policy-denied calls.

#### Work unit C — commit-before-effect and outcome mapping

If `Executable`:

```text
persist handling start
+ ToolCallPrepared::Executable
+ ToolEffectStarted
→ commit succeeds
→ execute once
```

Then:

- Output → effect completion + reserved success result;
- OrdinaryError → effect completion error + one model-visible ordinary tool error;
- Cancelled → control/interruption path, never ordinary tool error;
- panic/invariant → kernel failure from last durable boundary.

A probe MUST prove the executor is never called before the `ToolEffectStarted` append commits.

#### Work unit D — turn and walking skeleton

Deliver:

- sequential source-order tool handling;
- ordered result IDs in `TurnCommitted`;
- second provider request sees ordered tool results;
- text-only success;
- `model → test.echo → model` success;
- Length is not success;
- stale expected-version decisions launch no external work.

T008 remains a small functional driver. Session façade, hooks, queues, compaction, parallel tools, transport, SDK/CLI, and common terminalization remain outside.

### Phase C — T009: common lifecycle and durable cancellation

Deliver:

- one Agent run ↔ one common `agent` execution;
- durable cancellation request before live root-token signal;
- child tokens to provider/tool consumers;
- pre-launch and pre-effect-start cancellation checks;
- distinct handling of tool Cancelled outcome;
- Never ambiguity → intervention;
- idempotent Agent-terminal/common-nonterminal repair;
- fixed recovery precedence from SPEC §9.

### Phase D — T010: durable limits

Deliver:

- canonical common deadline + matching Agent snapshot validation;
- durable model/completed-turn/logical-tool/token counts;
- usage-capability validation for token limits;
- whole assistant tool-call batch admission before preparation/execution;
- no `limit + 1` provider request;
- no partially executed over-budget tool batch;
- token/deadline/budget terminal settlement.

### Phase E — T011: restart matrix

Deliver:

- deterministic F1–F8 failpoints;
- distinct-process resume against the same SQLite database;
- durable unsafe-effect probe;
- Safe retry, Never no-replay, result repair, turn repair, terminal repair;
- competing-resumer proof;
- second-restart idempotency;
- corruption outside the matrix fails closed.

### Phase F — T012: exact-code evaluations and review

Deliver:

- current EvaluationRuns for every active evaluation;
- exact reviewed commit pin;
- requirement-to-evidence ledger;
- architecture/dependency/scope review;
- permanent local/hosted gate;
- explicit requester Goal acceptance request.

## 4. Verification checkpoints

Every work unit:

1. run focused tests;
2. run formatting;
3. run architecture-boundary checks;
4. run locked Clippy with warnings denied;
5. run full locked workspace tests;
6. land only a coherent green increment on `main`.

Task closure additionally requires hosted CI success on the exact closure commit and recorded Verification evidence.

## 5. Parallelization

Safe:

- table-driven tests for already-set contracts;
- documentation/evidence updates;
- independent negative fixtures.

Sequential:

- T007 preparation contract before T008;
- durable grammar before kernel reliance;
- lifecycle before limits;
- limits before restart matrix;
- EvaluationRuns after final code basis.

Two agents MUST NOT concurrently alter the durable record vocabulary or the same Task evidence without explicit coordination.

## 6. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| preparation types placed in core create a forbidden tools→core dependency or duplicate taxonomy | own disposition/stage in `ww-agent-tools`; core embeds exact types |
| tools logic fragments across core and tools | one production preparation seam; core consumes, never duplicates |
| run tool order leaks registry insertion order | configured-pin projection tests in T007 + request integration test in T008 |
| schema silently reaches external resources | fragment-only `$ref`/`$dynamicRef`; disabled resolver features; explicit rejection |
| canonical digest test false-greens | assert nested canonical bytes, then digest |
| policy seam is structurally present but behaviorally unused | non-optional classification fields + effect/replay-aware substitution/observation proof |
| cancellation becomes ordinary tool error | machine-distinguishable Cancelled execution outcome + T008/T009 mapping |
| T007 claims a proof it cannot perform | T007 proves grammar; T008 owns real commit-before-effect probe |
| `ToolEffectStarted` is misread as effect receipt | treat only as ambiguity boundary |
| Task wording conflicts with SPEC flexibility | SPEC owns semantics; function/module names are conventional, not normative |
| Goal expands into G004/G010 | existing exclusions and Stop Conditions remain |

## 7. Stop and escalation rules

Stop and retain durable state if:

- satisfying the Task requires changing Goal/ADR-0003 boundaries;
- Agent DTOs must enter shared `ww-store`;
- a provider/tool external operation would occur before its authorizing durable commit;
- the single preparation seam cannot preserve tools/core dependency direction;
- cancellation cannot remain distinct from ordinary error;
- safe completion requires a new durable record/state not already governed;
- a new prerequisite Task/Goal appears necessary.

Otherwise, keep implementation inside the current Task and prefer the simplest compliant path.
