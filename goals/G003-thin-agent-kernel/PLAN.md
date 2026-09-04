# Plan — G003 Durable Agent Kernel

- Version: `v2`
- Approval: `pending requester approval`
- Specification basis: `G003 SPEC v2`
- Refinement: `D021`
- Goal state: `active`
- Implementation state while pending: blocked by the G003 `REPLAN_LOCK`

## 1. Planning contract

This plan makes the remaining accepted G003 sequence executable. It preserves:

```text
T007 → T008 → T009 → T010 → T011 → T012
```

T001–T006 remain complete. Work units below are implementation checkpoints inside existing Tasks, not new Task identities.

The Addy Osmani planning model is adapted as follows:

- the Goal packet remains the task-list target; no parallel `tasks/todo.md` is created;
- dependency order and vertical proof slices are retained;
- likely files, acceptance, verification, risks, and checkpoints are explicit;
- approximately five implementation files per work unit is a strong sizing recommendation, not a rejection rule.

## 2. Dependency graph

```text
T002 provider protocol ──→ T006 RecordedProvider ────────────┐
                                                            │
T003 durable model ──→ T004 store ──→ T005 coordinator ─────┼──→ T008 kernel
                                                            │          ↓
T007 tool contract + durable preparation ───────────────────┘        T009 lifecycle/cancel
                                                                       ↓
                                                                    T010 limits
                                                                       ↓
                                                                    T011 restart matrix
                                                                       ↓
                                                                    T012 evaluations/review
```

No T008 implementation begins until T007 is complete and verified.

### 2.1 Design-lineage rule

Each open Task implements a WorkWeave contract, not a reference-project clone:

- T007 translates Pi's validate/preflight/result ordering plus Harness source-index/reserved-result reduction into a provider-independent tool contract.
- T008 translates Pi's injected `StreamFn` and small `runLoop` into a durable functional driver over WorkWeave ports.
- T009 combines Pi's propagated abort signal with G002's durable-request-before-live-signal lifecycle.
- T010 applies the dossier's reserve-before-work budget rule using durable Agent records instead of Pi callback-local state.
- T011 applies Harness corruption/reduction discipline and LangGraph checkpoint/pending-write lessons without adopting graph execution or unsafe node re-execution.
- T012 proves those adaptations behaviorally; it does not require source or API parity with Pi, Harness, or LangGraph.

The exact immutable source paths and preserve/adapt/reject decisions are recorded in SPEC §4. Implementation reviews MUST cite the SPEC requirement being satisfied rather than appeal directly to an upstream implementation.

## 3. Implementation strategy

### Phase A — T007 tool safety and replay contract

Deliver one complete vertical tool-preparation path:

```text
registered fixture
→ compiled offline schema
→ exact argument validation
→ effect/replay classification
→ deterministic policy
→ durable pre-effect metadata
→ allowed execution or no-effect result
→ reducer reconstruction
```

#### T007 work unit A — crate boundary, identity, schema

Likely files:

- root `Cargo.toml`
- `Cargo.lock`
- `crates/ww-agent-tools/Cargo.toml`
- `crates/ww-agent-tools/src/lib.rs`
- `crates/ww-agent-tools/src/identity.rs`
- `crates/ww-agent-tools/src/schema.rs`

Deliver:

- crate added with permitted dependencies only;
- `ToolId`, `ToolVersion`, `ToolIdentity`, `ToolSpec`;
- tools-owned request/context types that contain no Agent run/call/attempt/entry identity;
- Draft 2020-12 schema validation with `jsonschema 0.52.1`, default features disabled;
- external-reference rejection;
- non-coercing deterministic validation errors.

Checkpoint:

```bash
cargo test -p ww-agent-tools --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

#### T007 work unit B — registry, policy, effect/replay, fixtures

Likely files:

- `crates/ww-agent-tools/src/registry.rs`
- `crates/ww-agent-tools/src/policy.rs`
- `crates/ww-agent-tools/src/fixtures.rs`
- `crates/ww-agent-tools/tests/tool_contract.rs`
- exports in `src/lib.rs`

Deliver:

- immutable duplicate-rejecting registry;
- deterministic model-spec ordering;
- canonical argument digest;
- `EffectDescriptor`, `ReplayPolicy`, `PolicyDecision`;
- `test.echo` and injected-probe `test.unsafe_once`;
- zero-effect validation failure and policy denial tests.

Checkpoint: focused tool tests + permanent gate.

#### T007 work unit C — Agent durable preparation/recovery vocabulary

Likely files:

- `crates/ww-agent-core/Cargo.toml`
- `crates/ww-agent-core/src/history.rs`
- `crates/ww-agent-core/src/reducer.rs`
- `crates/ww-agent-core/src/lib.rs`
- `crates/ww-agent-core/tests/recovery.rs`

Deliver:

- durable call classification/preparation snapshot;
- reserved result identity;
- explicit `ToolEffectStarted`, effect-output, and interrupted-attempt state;
- distinct `ToolAttemptRejected` preparation failure and `ToolAttemptDenied` policy failure;
- reducer support for executable, denied/rejected, completed, interrupted, and intervention states;
- corruption tests for changed replay/policy, duplicate result, wrong reserved ID, and source-order violations.

Checkpoint: core recovery tests + permanent gate.

#### T007 closure checkpoint

T007 closes only when:

- every `V-T007` check passes;
- all new durable metadata reconstructs identically after SQLite reopen;
- `TASKS.md` and `VERIFICATION.md` contain exact evidence;
- no T008 loop code or G004 capability was introduced.

### Phase B — T008 functional kernel

Deliver the first real Agent execution, still independent from common lifecycle settlement.

#### T008 work unit A — request builder and model-attempt finalization

Likely files:

- `crates/ww-agent-core/src/kernel.rs`
- optional `crates/ww-agent-core/src/model.rs`
- `crates/ww-agent-core/src/history.rs`
- `crates/ww-agent-core/src/reducer.rs`
- `crates/ww-agent-core/src/lib.rs`
- `crates/ww-agent-core/tests/kernel.rs`

Deliver:

- typed run configuration;
- deterministic request construction from durable entries;
- request digest/pin record;
- one production stream-drain/finalize path;
- normalized outer provider-dispatch failure with no assistant/effect;
- versioned snapshot → reduce → expected-version append mutation cycle;
- immutable assistant persistence or typed interruption.

#### T008 work unit B — sequential tool loop and terminal result

Likely files:

- `kernel.rs`
- `tests/kernel.rs`
- minimal supporting core modules already introduced

Deliver:

- source-ordered preparation/execution/result handling;
- policy denial and tool failure as one model-visible error result;
- `TurnCommitted`;
- second provider request sees ordered results;
- text-only and model→tool→model success.

Checkpoint: kernel tests + permanent gate.

T008 MUST remain a small functional driver. Discovery of a need for session queues, compaction, hooks, parallel scheduling, or a broad Agent object is scope escalation, not implementation discretion.

### Phase C — T009 common lifecycle and cancellation

#### T009 work unit A — runtime cancellation/terminal primitives

Likely files:

- `crates/ww-runtime/src/cancellation.rs`
- `crates/ww-runtime/src/service.rs`
- runtime tests
- shared event/reducer files only when an already-declared status lacks a legal event transition

Deliver:

- one root token per execution with child tokens for consumers;
- durable cancel intent before live signal;
- generic terminal methods for declared common statuses needed by Agent settlement.

#### T009 work unit B — Agent lifecycle binding and repair

Likely files:

- `crates/ww-agent-core/src/lifecycle.rs`
- `crates/ww-agent-core/src/kernel.rs`
- `crates/ww-agent-core/src/lib.rs`
- `crates/ww-agent-core/tests/lifecycle.rs`
- coordinator/link integration tests as needed

Deliver:

- one-to-one link validation;
- pending start, running resume, pre-start cancellation;
- provider/tool token propagation;
- cancellation recheck immediately before `ToolEffectStarted` plus fixed recovery precedence;
- Agent-terminal/common-nonterminal idempotent repair;
- never-replayable cancellation ambiguity becomes intervention.

Checkpoint: lifecycle tests + permanent gate.

### Phase D — T010 durable limits

#### T010 work unit A — limit model and pure decisions

Likely files:

- `crates/ww-agent-core/src/limits.rs`
- `crates/ww-agent-core/src/history.rs`
- `crates/ww-agent-core/src/reducer.rs`
- `crates/ww-agent-core/src/lib.rs`
- `crates/ww-agent-core/tests/limits.rs`

Deliver:

- typed positive limits;
- durable model/turn/tool/token counts;
- pure “may start next operation?” decisions;
- exact deadline and inclusive/exclusive boundary tests.

#### T010 work unit B — enforcement and common settlement

Likely files:

- `kernel.rs`
- `lifecycle.rs`
- shared runtime event/service files if required
- `tests/limits.rs`

Deliver:

- reserve before provider/tool launch;
- no operation `limit + 1`;
- token stop before next model request;
- deadline cancellation during active work;
- BudgetExhausted/TimedOut terminal mapping.

Checkpoint: limits tests + permanent gate.

### Phase E — T011 fault/restart matrix

#### T011 work unit A — deterministic fault harness

Likely files:

- `crates/ww-agent-core/src/fault.rs` or test-only equivalent
- `crates/ww-agent-store-sqlite/src/bin/agent-kernel-fixture.rs`
- `crates/ww-agent-store-sqlite/Cargo.toml`
- test fixture support files

Deliver:

- named F1–F8 failpoints;
- F2 pre-event and transient-partial-delta subcases;
- seed/resume/inspect process commands;
- test-only durable unsafe-effect probe;
- no public CLI/SDK surface.

#### T011 work unit B — process restart matrix

Likely files:

- `crates/ww-agent-store-sqlite/tests/recovery_matrix.rs`
- supporting fixture code only

Deliver:

- one test per F1–F8;
- second-restart idempotency;
- provider/effect invocation counters;
- exact durable history assertions;
- corrupt-state cases reject rather than repair.

Checkpoint: recovery matrix focused command + permanent gate.

### Phase F — T012 exact-code evaluation and terminal review

Likely files:

- `goals/G003-thin-agent-kernel/EVALUATIONS.md`
- `VERIFICATION.md`
- `REVIEWS.md`
- `TASKS.md`
- `PROJECT_STATE.md`
- Decision/ADR only if review discovers a governed change

Deliver:

- current EvaluationRuns for every required check;
- exact reviewed commit pin;
- final architecture/dependency/recovery review;
- complete evidence ledger;
- requester acceptance remains separate.

## 4. Verification checkpoints

Every work unit:

1. run its focused test target;
2. run formatting;
3. run workspace clippy with `--locked -D warnings`;
4. run full workspace tests with `--locked`;
5. land only a coherent green increment on `main`.

Task closure additionally requires:

- Task-specific acceptance;
- updated verification evidence;
- hosted CI success on the exact closure commit.

No temporary verifier may omit a permanent gate.

## 5. Parallelization

Safe after its provider contract is stable:

- writing table-driven tests for an already-defined module;
- documentation/evidence updates;
- independent negative fixtures.

Must remain sequential:

- T007 contracts before T008;
- durable record/reducer changes before kernel reliance;
- lifecycle before limits;
- limits before restart matrix;
- EvaluationRuns after final code basis.

Two agents MUST NOT concurrently edit the same durable record vocabulary or Task evidence without explicit coordination.

## 6. Risks and mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| T007 grows into generic policy/sandbox infrastructure | high | G003 Allow/Deny + synthetic effects only; defer approvals/real capabilities |
| schema validator imports network/filesystem | high | `jsonschema 0.52.1`, `default-features = false`, explicit external-ref rejection |
| tool contract acquires Agent-owned IDs and creates a dependency cycle | high | keep run/logical-call/attempt/entry IDs in core; tool request is execution-only and tools-owned |
| raw and parsed arguments diverge semantically | high | parsed `Value` is sole authority; deterministic digest from it |
| preparation rejection is mislabeled as policy denial | medium | add `ToolAttemptRejected`; reserve `ToolAttemptDenied` for an actual Deny decision |
| durable pre-effect data is incomplete or handling start is confused with effect start | high | explicit `ToolEffectStarted`; `V-T007` asserts every required field and marker exist before fixture probe invocation |
| kernel stops reading after terminal event | high | drain stream to EOF, then `finish()` |
| safe retry creates duplicate logical result | high | reserved result ID + reducer uniqueness + F4/F6 tests |
| Never effect is replayed after ambiguous crash | critical | replay pin and `ToolEffectStarted` durable before invocation; F5 effect counter remains one |
| cancellation maps unsafe ambiguity to Cancelled | high | Never + started + no result maps to intervention |
| limits use process-local counters | high | derive all counters from records and entries |
| stale concurrent driver launches work from a rejected append | critical | expected-version append must commit before I/O; on conflict discard decision, reload and reduce |
| cancel/deadline masks unsafe ambiguity | critical | fixed recovery precedence with Never ambiguity ahead of cancel/deadline/budget |
| Goal expands into G004/G010 work | high | D021 lock, explicit exclusions, existing Stop Conditions |
| Task/file scope becomes too large | medium | use internal work units; ~5 files is a strong signal, not a hard gate |

## 7. Stop and escalation rules

Stop implementation and retain current durable state if:

- Agent DTOs must enter shared `ww-store`;
- provider/tool output can cross an effect boundary before validation/finalization;
- one logical call can acquire two committed model-visible results;
- a Never attempt can be silently re-executed;
- counters require process-local truth;
- core requires concrete transport/SQLite/filesystem/Flow/Orchestration;
- an internal work unit cannot remain a focused verifiable increment.

A non-blocking improvement does not stop the Goal. Record it and map it to an existing Task only when needed for acceptance; otherwise defer it.

## 8. Rollback

A failed open Task may be reverted while retaining its evidence and findings. T001–T006 and ADR-0003 remain valid. Do not renumber Tasks or insert a generic cleanup gate.
