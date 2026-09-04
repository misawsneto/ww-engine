# G003 Tasks

- Planning basis: `SPEC v2` + `PLAN v2`
- Approval: `approved by requester 2026-09-04 under D021`
- Completed T001–T006 retain their original meaning and evidence.
- T007–T012 retain their existing identifiers and dependency order.

## Canonical Task index

| Task | State | Acceptance | Dependencies |
| --- | --- | --- | --- |
| T001 Accept G002 review and activate G003 | complete | G002 achieved; ADR-0003 accepted; G003 state active; project state points to G003 | G002 T010 |
| T002 Define provider-neutral protocol and stream assembler | complete | immutable provider/model/message/tool-call types exist; pure assembler passes valid text/tool streams and fails closed on malformed/truncated/duplicate terminal streams | T001 |
| T003 Define Agent entries, operational records, and recovery reducer | complete | immutable context-entry/attempt vocabulary reconstructs deterministic `AgentRecoveryState`; impossible references/order reject with typed corruption errors | T001 |
| T004 Implement Agent SQLite persistence and reconstruction | complete | Agent schema remains Agent-owned; append/query/reopen/process-restart reconstruction works; stale Agent writers reject without partial Agent mutation | T003 |
| T005 Prove common/Agent SQLite transaction coordination | complete | common execution + Agent creation/link commit atomically without Agent DTOs in shared `ww-store`; injected failure leaves neither half committed; terminal repair remains T009 | T004 |
| T006 Implement recorded provider and provider conformance fixtures | complete | deterministic fixtures cover text, tool calls, usage, failure, cancellation, truncation, and interrupted attempts through the normalized provider contract | T002 |
| T007 Implement tool contract, schema validation, policy, and replay fixtures | open | complete `V-T007`; deterministic replay-safe and synthetic non-replayable fixtures validate exact arguments before policy/effect; no-effect paths invoke no effect and create one ordered model-visible result; replay/effect/policy/result identity required by recovery is durable before execution | T002, T003, T004 |
| T008 Implement functional recorded-provider model → tool → model kernel | open | complete `V-T008`; the real functional kernel completes text-only and one-tool runs, drains/finalizes provider streams fail-closed, preserves ordinary tool errors as model-visible results while keeping cancellation/invariant failures distinct, commits model/tool boundaries in order, and imports no concrete transport/SQLite/filesystem/Flow types | T005, T006, T007 |
| T009 Integrate G002 lifecycle and durable cancellation | open | complete `V-T009`; one Agent run maps to one common execution; start/resume/terminal repair is idempotent; durable cancellation reaches provider/tool child tokens and never-replayable ambiguity requires intervention | T008 |
| T010 Implement durable deadlines and execution budgets | open | complete `V-T010`; common execution deadline is canonical; model/completed-turn/logical-tool/token counters derive from durable history; token limits require provider usage observability; whole tool-call batches are admitted before execution; operation/batch beyond the limit is never launched | T009 |
| T011 Prove crash/restart and ambiguous-effect recovery matrix | open | complete `V-T011`; distinct-process F1–F8 tests and second restart prove safe retry, no duplicate logical result, no Never replay, ordered repair, and idempotent terminal settlement | T010 |
| T012 Record required EvaluationRuns and perform G003 recovery/architecture review | open | complete `V-T012`; every required Evaluation passes on the exact reviewed commit, permanent gates are green, and review finds no blocking provider/tool/store/Flow/Orchestration boundary violation | T011 |

## T007 — Tool contract, validation, policy, and replay fixtures

**Description:** Add the provider-independent tool subsystem and the Agent durable metadata required to decide whether a finalized model tool call can execute, must return a no-effect error, can be replayed safely, or requires intervention after ambiguity.

**Design basis:** Pi production establishes resolve → validate → preflight → execute → normalized-result ordering and source-ordered result emission. Pi Harness establishes assistant-entry/source-index linkage, reserved result identity, attempt continuity, and fail-closed reduction. WorkWeave keeps those seams, adds durable effect/replay/policy state, and rejects Pi argument coercion, parallel execution, postflight mutation, and Harness product/lane machinery.

### Work units

1. Tool identity, Draft 2020-12 schema profile, and compiled validator.
2. Registry, deterministic argument digest, effect/replay classification, and Allow/Deny policy.
3. `test.echo` and `test.unsafe_once` fixtures with effect probes.
4. Agent durable call classification/reserved-result/interruption/effect-output records and reducer rules.
5. Contract and corruption tests, boundary guard, and evidence.

### Acceptance criteria

- [ ] `ww-agent-tools` exists with the dependency direction in SPEC §5.1.
- [ ] `ww-agent-tools` public request/context types contain no Agent-owned run, logical-call, attempt, or entry ID and require no core dependency.
- [ ] Registry rejects duplicate IDs and malformed/external-reference schemas before a run.
- [ ] Validation accepts/rejects the exact parsed `Value`; it never coerces or injects defaults.
- [ ] Invalid arguments invoke policy zero times and tool execution zero times.
- [ ] Canonical digests are stable across object key order and change for different values.
- [ ] Effect/replay classification occurs after validation and before policy.
- [ ] Policy denial invokes the effect zero times and yields one `policy_denied` model-visible result.
- [ ] `test.echo` is deterministic and `Safe`; `test.unsafe_once` is synthetic and `Never`.
- [ ] Durable history contains the required tool pin, digest, effect, replay, policy, source position, attempt ID, reserved result ID, and explicit effect-start marker before an allowed invocation.
- [ ] No-effect settlements contain no effect-start/effect-completion record; reducer reconstructs executable/no-effect/completed/interrupted/intervention states and rejects the SPEC §7.6 corrupt cases.
- [ ] Resolve/Validate/Classify failures end in `ToolAttemptRejected`; only a durable policy Deny ends in `ToolAttemptDenied`.
- [ ] No concrete capability, approval workflow, parallel scheduling, or T008 kernel loop is added.

### Verification

```bash
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

**Files likely touched:** root Cargo files; new `crates/ww-agent-tools/**`; `ww-agent-core` history/reducer/tests; the existing CI boundary block if needed.

**Estimated scope:** Large at Task level, intentionally split into focused work units. Keep each work unit near five implementation files when coherent.

## T008 — Functional recorded-provider kernel

**Description:** Implement the smallest real durable Agent loop over the completed provider, tool, and store ports. Prove text-only and one-tool round trips without common lifecycle integration or product surface.

**Design basis:** Pi's injected `StreamFn` and small `runLoop` prove the provider/loop separation and alternating assistant/tool-result structure. WorkWeave adapts the loop to immutable entries, operational records, expected-version appends, and recovery-derived next actions; it does not import Pi's stateful Agent façade, queues, hooks, or product session behavior.

### Work units

1. Typed run configuration, deterministic context/request builder, and model-attempt preparation.
2. Mandatory stream drain/finalization and assistant persistence.
3. Sequential tool/no-effect/effect-result handling and turn commit.
4. Text-only and model→tool→model integration fixtures.

### Acceptance criteria

- [ ] Kernel dependencies are injected and no database/provider transport is constructed inside core.
- [ ] Request content derives only from typed configuration and ordered durable entries.
- [ ] Provider/model/request digest attempt state is durable before provider I/O.
- [ ] An outer provider-dispatch error creates one typed failed/interrupted attempt and no assistant entry, tool preparation, or automatic retry.
- [ ] Every stream is consumed through EOF and `ResponseAssembler::finish`; interrupted or malformed streams create no assistant entry or effect.
- [ ] Finalized assistant entry commits before any requested tool effect.
- [ ] Logical call IDs are allocated once in source order and then recovered from the durable assistant entry.
- [ ] Invalid/unknown/denied paths return exactly one ordered model-visible error result and continue only where specified.
- [ ] A returned ordinary `ToolExecutionError` is durably recorded as exactly one model-visible `is_error=true` result and may be presented to the next model request.
- [ ] Cancellation is handled by cancellation semantics and MUST NOT be normalized into an ordinary tool error result.
- [ ] A panic or impossible invariant/contract violation is not converted into an ordinary model-visible tool error; the Agent fails/crashes and subsequent behavior derives from durable state.
- [ ] Allowed tools execute sequentially and result ordering equals provider call ordering.
- [ ] A turn commits only after all results are durable/model-visible.
- [ ] Text-only and `model → test.echo → model` runs commit the expected terminal Agent result.
- [ ] `Length` does not produce a successful Agent result.
- [ ] Every mutation decision derives from one versioned snapshot; an optimistic conflict discards the stale decision and launches no provider/tool work before reload/reduction.
- [ ] No lifecycle, limits, crash matrix, public SDK/CLI, or concrete provider work is pulled forward.

### Verification

```bash
cargo test -p ww-agent-core --test kernel --locked
cargo test -p ww-agent-provider --test recorded_provider --locked
cargo test -p ww-agent-tools --locked
# then permanent gate
```

**Files likely touched:** `ww-agent-core` kernel/model/history/reducer/lib/tests and only minimal supporting contract files.

**Estimated scope:** Medium-to-large; use separate request/finalization and tool-loop work units.

## T009 — Common lifecycle and durable cancellation

**Description:** Bind the functional kernel to the existing G002 execution lifecycle without moving Agent semantics into the shared runtime.

**Design basis:** Pi propagates one abort signal through model/tool work. G002 makes cancellation a durable request followed by live delivery. WorkWeave composes those ideas through one execution root and child tokens, while refusing to interpret cancellation as proof that a started Never effect did not occur.

### Work units

1. Generic common-runtime cancellation/terminal primitives already implied by declared statuses.
2. Agent/common link validation and start/resume behavior.
3. Provider/tool child-token propagation.
4. Agent-terminal/common-nonterminal repair and ambiguity mapping.

### Acceptance criteria

- [ ] Exactly one Agent run maps to exactly one common execution of kind `agent`.
- [ ] Pending execution starts once; Running/Waiting resumes; matching terminal state performs no work.
- [ ] Durable cancellation request precedes live cancellation signal.
- [ ] Repeated registrations share one root cancellation and consumers cannot cancel sibling work.
- [ ] Cancellation before launch invokes no provider/tool.
- [ ] Cancellation reaches active provider and safe tool tokens.
- [ ] Cancellation is rechecked immediately before `ToolEffectStarted`; a request already durable at that point prevents both marker and invocation.
- [ ] Never-replayable started/no-result cancellation settles `RequiresIntervention`, not falsely `Cancelled`.
- [ ] Durable completed results survive later cancellation and are repaired/committed.
- [ ] Agent terminal results map to the matching common status.
- [ ] Terminal repair is idempotent and never contacts provider/tool.
- [ ] Shared runtime contains no Agent DTO or message/tool semantics.
- [ ] Corruption, existing terminal state, Never ambiguity, cancel, deadline, and budget use the fixed SPEC §9.6 precedence.

### Verification

```bash
cargo test -p ww-agent-core --test lifecycle --locked
cargo test -p ww-runtime --locked
cargo test -p ww-agent-store-sqlite --test coordinator --locked
# then permanent gate
```

**Files likely touched:** runtime cancellation/service/tests; Agent core lifecycle/kernel/tests; bounded coordinator tests.

**Estimated scope:** Large at Task level; retain two clear boundaries—generic runtime primitives and Agent binding.

## T010 — Durable deadlines and execution budgets

**Description:** Make every provider/tool launch conditional on limit decisions reconstructed from durable history.

**Design basis:** The WorkWeave architecture dossier requires reservation before expensive work and reconciliation from durable usage. Pi's stop-after-turn callback identifies a useful decision boundary but remains process-local; G003 replaces that discretion with typed pure decisions over reduced durable state.

### Work units

1. Typed limits, canonical deadline snapshot validation, and pure boundary decisions.
2. Durable model/completed-model-turn/logical-tool-call/token counting.
3. Provider usage-capability validation and tool-batch admission.
4. Provider/tool enforcement and deadline/token/count terminal settlement.

### Acceptance criteria

- [ ] Count limits are positive and typed.
- [ ] The linked G002 `ExecutionRecord.deadline` is the canonical deadline. Any Agent configuration deadline is a snapshot that must exactly match it; a mismatch rejects/fails closed before provider/tool work.
- [ ] model requests count durable model attempt starts.
- [ ] `max_turns` uses a distinct durable completed-model-turn count from `ModelAttemptCompleted`; the existing T003 `turn_count` remains the count of `TurnCommitted` records.
- [ ] `max_tool_calls` counts logical model-requested calls from finalized assistant responses; the existing T003 `tool_attempt_count` remains attempt audit state and safe retries do not redefine the logical-call budget.
- [ ] counters reconstruct identically after reopen.
- [ ] when any token limit is configured, a provider/model with `usage == false` rejects before provider I/O.
- [ ] when usage capability is declared but a finalized response omits normalized usage, the attempt fails closed as provider protocol failure before another model request.
- [ ] before executing any tool from a finalized multi-call response, the complete source-ordered logical-call batch is checked against remaining `max_tool_calls` capacity.
- [ ] if the complete batch exceeds remaining capacity, no tool in that response is prepared/executed and the Agent/common execution settles `BudgetExhausted`.
- [ ] provider work is never launched as model request `limit + 1`; logical tool-call batches beyond capacity are never partially executed.
- [ ] `now >= deadline` prevents launch and active deadline expiry cancels child work.
- [ ] token usage accumulates from finalized normalized usage.
- [ ] reaching/exceeding token limit stops before the next model request.
- [ ] BudgetExhausted and TimedOut are audited Agent/common terminal outcomes.
- [ ] Never ambiguity outranks timeout/cancel when effect outcome is unknown.
- [ ] Simultaneous cancel/deadline/budget observations produce the same terminal disposition before and after reopen according to SPEC §9.6.

### Verification

```bash
cargo test -p ww-agent-core --test limits --locked
cargo test -p ww-runtime --locked
# then permanent gate
```

**Files likely touched:** Agent core limits/history/reducer/kernel/lifecycle/tests and only required generic runtime event/service files.

**Estimated scope:** Medium-to-large; pure decisions first, integration second.

## T011 — Crash/restart and ambiguous-effect recovery matrix

**Description:** Prove the durability thesis in distinct processes using named fault boundaries and observable provider/effect counters.

**Design basis:** Pi Harness reduces entries plus operational records and rejects contradictory histories. LangGraph demonstrates checkpoint-keyed resume and pending-write reconstruction, while its interrupt contract explicitly re-executes node logic from the start. G003 adopts durable reconstruction and pending-result repair but permits re-execution only when WorkWeave's recorded replay policy says `Safe`.

### Work units

1. Test-only F1–F8 fault injector and process driver.
2. durable unsafe-effect probe.
3. restart/repair matrix.
4. second-restart idempotency and corruption cases.

### Acceptance criteria

- [ ] Every F1–F8 state in SPEC §11 is constructed at the exact boundary.
- [ ] Resume occurs in a new OS process against the same SQLite database.
- [ ] F2 creates a distinct model attempt only when permitted.
- [ ] F2 covers both process loss before the first provider event and after transient partial deltas; neither produces a durable assistant entry.
- [ ] F3 performs zero additional provider calls before pending tool/terminal handling.
- [ ] F4 with durable Safe `ToolEffectStarted` and no effect result retries with a new attempt and one logical result.
- [ ] F5 with durable Never `ToolEffectStarted` and no effect result leaves the effect probe exactly one and settles RequiresIntervention.
- [ ] F6 repairs the reserved model-visible result without effect re-execution.
- [ ] F7 appends one turn commit without provider/tool work.
- [ ] F8 terminalizes common execution once without provider/tool work.
- [ ] A second restart is a no-op for effects, logical results, and terminal events.
- [ ] corrupt histories outside the repair matrix fail closed.
- [ ] two competing resume attempts cannot both authorize external work; the losing expected-version append reloads/reduces without provider/tool invocation.

### Verification

```bash
cargo test -p ww-agent-store-sqlite --test recovery_matrix --locked
# then permanent gate
```

**Files likely touched:** test-only Agent kernel process fixture, fault support, recovery-matrix tests, package target declaration.

**Estimated scope:** Medium; matrix is table-driven rather than eight unrelated harnesses.

## T012 — EvaluationRuns and terminal review

**Description:** Pin the exact final code state, execute every required deterministic Evaluation, and independently review the resulting architecture and recovery behavior.

**Design basis:** The approved refinement method adapts Addy Osmani's explicit specification/plan/acceptance discipline. Reference parity is behavioral: review proves the selected Pi/Harness/LangGraph lessons and WorkWeave deviations, not package layout or API compatibility.

### Acceptance criteria

- [ ] Every check in `EVALUATIONS.md` has a current passing EvaluationRun appended under that check.
- [ ] Every SPEC requirement family maps to passing Verification evidence.
- [ ] Exact reviewed commit passes the permanent gate and hosted CI.
- [ ] Review confirms no concrete provider/filesystem/process/network/product/Flow/Orchestration scope.
- [ ] Review confirms no duplicate logical result or silent Never replay path.
- [ ] Review confirms common/Agent terminal consistency and idempotent repair.
- [ ] Review traces every open Task to SPEC §4's observed evidence, WorkWeave adaptation, and explicit rejected/deferred behavior.
- [ ] Review records residual findings without automatically adding prerequisite work.
- [ ] No G003 Stop Condition remains triggered.
- [ ] Goal acceptance is explicitly requested from the human owner; it is not inferred from `main`.

### Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

**Files likely touched:** G003 Evaluation, Verification, Review, Task, and Project State records.

**Estimated scope:** Small-to-medium, documentation/evidence only.
