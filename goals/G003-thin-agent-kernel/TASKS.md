# G003 Tasks

- Planning basis: `SPEC v2` + `PLAN v2`
- Approval: `pending requester approval under D021`
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
| T008 Implement functional recorded-provider model → tool → model kernel | open | complete `V-T008`; the real functional kernel completes text-only and one-tool runs, drains/finalizes provider streams fail-closed, commits model/tool boundaries in order, and imports no concrete transport/SQLite/filesystem/Flow types | T005, T006, T007 |
| T009 Integrate G002 lifecycle and durable cancellation | open | complete `V-T009`; one Agent run maps to one common execution; start/resume/terminal repair is idempotent; durable cancellation reaches provider/tool child tokens and never-replayable ambiguity requires intervention | T008 |
| T010 Implement durable deadlines and execution budgets | open | complete `V-T010`; model/turn/tool/token counters derive from durable history, reserve before work, prohibit operation `limit + 1`, and settle deadline/budget outcomes consistently | T009 |
| T011 Prove crash/restart and ambiguous-effect recovery matrix | open | complete `V-T011`; distinct-process F1–F8 tests and second restart prove safe retry, no duplicate logical result, no Never replay, ordered repair, and idempotent terminal settlement | T010 |
| T012 Record required EvaluationRuns and perform G003 recovery/architecture review | open | complete `V-T012`; every required Evaluation passes on the exact reviewed commit, permanent gates are green, and review finds no blocking provider/tool/store/Flow/Orchestration boundary violation | T011 |

## T007 — Tool contract, validation, policy, and replay fixtures

**Description:** Add the provider-independent tool subsystem and the Agent durable metadata required to decide whether a finalized model tool call can execute, must return a no-effect error, can be replayed safely, or requires intervention after ambiguity.

### Work units

1. Tool identity, Draft 2020-12 schema profile, and compiled validator.
2. Registry, deterministic argument digest, effect/replay classification, and Allow/Deny policy.
3. `test.echo` and `test.unsafe_once` fixtures with effect probes.
4. Agent durable call classification/reserved-result/interruption/effect-output records and reducer rules.
5. Contract and corruption tests, boundary guard, and evidence.

### Acceptance criteria

- [ ] `ww-agent-tools` exists with the dependency direction in SPEC §5.1.
- [ ] Registry rejects duplicate IDs and malformed/external-reference schemas before a run.
- [ ] Validation accepts/rejects the exact parsed `Value`; it never coerces or injects defaults.
- [ ] Invalid arguments invoke policy zero times and tool execution zero times.
- [ ] Canonical digests are stable across object key order and change for different values.
- [ ] Effect/replay classification occurs after validation and before policy.
- [ ] Policy denial invokes the effect zero times and yields one `policy_denied` model-visible result.
- [ ] `test.echo` is deterministic and `Safe`; `test.unsafe_once` is synthetic and `Never`.
- [ ] Durable history contains the required tool pin, digest, effect, replay, policy, source position, attempt ID, and reserved result ID before an allowed effect.
- [ ] Reducer reconstructs executable/no-effect/completed/interrupted/intervention states and rejects the SPEC §7.6 corrupt cases.
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

### Work units

1. Typed run configuration, deterministic context/request builder, and model-attempt preparation.
2. Mandatory stream drain/finalization and assistant persistence.
3. Sequential tool/no-effect/effect-result handling and turn commit.
4. Text-only and model→tool→model integration fixtures.

### Acceptance criteria

- [ ] Kernel dependencies are injected and no database/provider transport is constructed inside core.
- [ ] Request content derives only from typed configuration and ordered durable entries.
- [ ] Provider/model/request digest attempt state is durable before provider I/O.
- [ ] Every stream is consumed through EOF and `ResponseAssembler::finish`; interrupted or malformed streams create no assistant entry or effect.
- [ ] Finalized assistant entry commits before any requested tool effect.
- [ ] Logical call IDs are allocated once in source order and then recovered from the durable assistant entry.
- [ ] Invalid/unknown/denied/tool-error paths return exactly one ordered model-visible error result and continue only where specified.
- [ ] Allowed tools execute sequentially and result ordering equals provider call ordering.
- [ ] A turn commits only after all results are durable/model-visible.
- [ ] Text-only and `model → test.echo → model` runs commit the expected terminal Agent result.
- [ ] `Length` does not produce a successful Agent result.
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
- [ ] Never-replayable started/no-result cancellation settles `RequiresIntervention`, not falsely `Cancelled`.
- [ ] Durable completed results survive later cancellation and are repaired/committed.
- [ ] Agent terminal results map to the matching common status.
- [ ] Terminal repair is idempotent and never contacts provider/tool.
- [ ] Shared runtime contains no Agent DTO or message/tool semantics.

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

### Work units

1. Typed limits and pure boundary decisions.
2. Durable model/turn/tool/token counting.
3. Provider/tool reservation enforcement.
4. deadline/token/count terminal settlement.

### Acceptance criteria

- [ ] Count limits are positive and typed; effective deadline is deterministic.
- [ ] model requests count durable model attempt starts.
- [ ] turns count durable `ModelAttemptCompleted` records, including terminal assistant responses.
- [ ] tool calls count durable tool attempt starts, including retries/no-effect attempts.
- [ ] counters reconstruct identically after reopen.
- [ ] provider/tool work is never launched as operation `limit + 1`.
- [ ] `now >= deadline` prevents launch and active deadline expiry cancels child work.
- [ ] token usage accumulates from finalized normalized usage.
- [ ] reaching/exceeding token limit stops before the next model request.
- [ ] BudgetExhausted and TimedOut are audited Agent/common terminal outcomes.
- [ ] Never ambiguity outranks timeout/cancel when effect outcome is unknown.

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

### Work units

1. Test-only F1–F8 fault injector and process driver.
2. durable unsafe-effect probe.
3. restart/repair matrix.
4. second-restart idempotency and corruption cases.

### Acceptance criteria

- [ ] Every F1–F8 state in SPEC §11 is constructed at the exact boundary.
- [ ] Resume occurs in a new OS process against the same SQLite database.
- [ ] F2 creates a distinct model attempt only when permitted.
- [ ] F3 performs zero additional provider calls before pending tool/terminal handling.
- [ ] F4 safely retries with a new attempt and one logical result.
- [ ] F5 effect probe remains exactly one and result is RequiresIntervention.
- [ ] F6 repairs the reserved model-visible result without effect re-execution.
- [ ] F7 appends one turn commit without provider/tool work.
- [ ] F8 terminalizes common execution once without provider/tool work.
- [ ] A second restart is a no-op for effects, logical results, and terminal events.
- [ ] corrupt histories outside the repair matrix fail closed.

### Verification

```bash
cargo test -p ww-agent-store-sqlite --test recovery_matrix --locked
# then permanent gate
```

**Files likely touched:** test-only Agent kernel process fixture, fault support, recovery-matrix tests, package target declaration.

**Estimated scope:** Medium; matrix is table-driven rather than eight unrelated harnesses.

## T012 — EvaluationRuns and terminal review

**Description:** Pin the exact final code state, execute every required deterministic Evaluation, and independently review the resulting architecture and recovery behavior.

### Acceptance criteria

- [ ] Every check in `EVALUATIONS.md` has a current passing EvaluationRun.
- [ ] Every SPEC requirement family maps to passing Verification evidence.
- [ ] Exact reviewed commit passes the permanent gate and hosted CI.
- [ ] Review confirms no concrete provider/filesystem/process/network/product/Flow/Orchestration scope.
- [ ] Review confirms no duplicate logical result or silent Never replay path.
- [ ] Review confirms common/Agent terminal consistency and idempotent repair.
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
