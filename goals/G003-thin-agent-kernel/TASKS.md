# G003 Tasks

- Planning basis: `SPEC v3` + `PLAN v3`
- Refinement authority: D021 baseline + resumed D022
- Approval: approved by requester 2026-09-05 under resumed D022
- Completed T001–T006 retain their original meaning and evidence.
- T007–T012 retain their identifiers and dependency order.
- G003 implementation is unblocked; the D022 `REPLAN_LOCK` is removed.

## Canonical Task index

| Task | State | Acceptance | Dependencies |
| --- | --- | --- | --- |
| T001 Accept G002 review and activate G003 | complete | G002 achieved; ADR-0003 accepted; G003 active | G002 T010 |
| T002 Define provider-neutral protocol and stream assembler | complete | normalized protocol + fail-closed assembler | T001 |
| T003 Define Agent entries, operational records, and recovery reducer | complete | deterministic recovery + typed corruption | T001 |
| T004 Implement Agent SQLite persistence and reconstruction | complete | Agent-owned persistence + reopen/restart + stale-writer rejection | T003 |
| T005 Prove common/Agent SQLite transaction coordination | complete | atomic create/link + rollback; no Agent DTO leakage | T004 |
| T006 Implement recorded provider and provider conformance fixtures | complete | deterministic provider fixtures and protocol conformance | T002 |
| T007 Implement tool contract, preparation seam, policy/replay fixtures, and durable tool grammar | complete / verified | complete `V-T007`; one tools-owned preparation seam, offline schema, exact ordering/canonicalization/policy semantics, distinct cancellation outcome, fixtures, and reducer grammar; no production commit-before-effect claim | T002, T003, T004 |
| T008 Implement functional recorded-provider model → tool → model kernel | open / NEXT | complete `V-T008`; real kernel consumes T007 preparation, persists no-effect results, proves commit-before-effect before execution, preserves cancellation/error distinction, and completes text/tool round trips | T005, T006, T007 |
| T009 Integrate G002 lifecycle and durable cancellation | open | complete `V-T009`; one Agent run ↔ one common execution; durable cancellation and idempotent terminal repair; Never ambiguity requires intervention | T008 |
| T010 Implement durable deadlines and execution budgets | open | complete `V-T010`; canonical common deadline, durable counters, usage observability, whole-batch tool admission, deterministic terminal limits | T009 |
| T011 Prove crash/restart and ambiguous-effect recovery matrix | open | complete `V-T011`; distinct-process F1–F8 + second restart + no duplicate result/Never replay | T010 |
| T012 Record required EvaluationRuns and perform terminal review | open | complete `V-T012`; exact-code evaluations, green permanent gates, independent architecture/recovery review, requester acceptance request | T011 |

## T007 — Tool contract, preparation, policy, replay fixtures, and durable grammar

**Outcome:** Deliver the provider-independent tool subsystem and Agent durable grammar needed to classify one finalized model tool call without executing it through the Agent kernel.

**Design basis:** Pi production separates preparation from execution. Pi Harness separates conversation entries from operational records and reduces durable tool state. T007 adopts those seams; T008 owns real Agent execution sequencing.

### Hard architecture requirements

1. `ww-agent-tools` owns **one production preparation seam**. It MUST compose:

   ```text
   exact resolve
   → exact non-coercing validation
   → canonical bytes/digest
   → effect classification
   → replay classification
   → policy
   → Executable | NoEffect
   ```

2. If any preparation stage fails, later stages MUST NOT run.
3. `ww-agent-core` MUST NOT duplicate resolution, validation, digesting, effect/replay classification, or policy evaluation.
4. `ww-agent-tools` MUST NOT depend on Agent core/runtime/store/SQLite/capability/Flow/Orchestration crates.
5. Preparation owns no Agent run/logical-call/attempt/entry identity and performs no external effect.
6. `ToolPreparationDisposition::{Executable, NoEffect}` and `ToolPreparationStage::{Resolve, Validate, Classify, Policy}` are **defined in `ww-agent-tools`**. `ww-agent-core` embeds those exact tools-owned types in Agent-owned durable records and MUST NOT define an equivalent second taxonomy.
7. Function/module/file names are implementation choices. Prefer a direct preparation function and small modules; do not introduce a generic manager/service/orchestrator to wrap the seam.
8. `ToolRegistry` is immutable per run and owns no run state.
9. Run configured pin order is authoritative. Registration order has no model-visible ordering authority.
10. D022 adds no durable record variant.

### Schema requirements

- Pin `jsonschema 0.52.1` with default resolver features disabled.
- Explicitly select Draft 2020-12.
- Reject non-fragment `$ref` and non-fragment `$dynamicRef` before validator compilation.
- Permit self-contained fragment references, including local dynamic anchors/references.
- Do not reject `$id` merely because it exists; it MUST NOT enable external retrieval or relax fragment-only references.
- Compile each schema once at registry construction.
- Return WorkWeave-owned deterministic validation errors.
- Never coerce or inject defaults.

### Ordering and canonicalization requirements

- Tests MUST register tools in one order and request configured pins in another order.
- Projection MUST return only configured exact pins in configured order.
- Canonical JSON bytes MUST recursively sort object keys.
- Tests MUST inspect nested canonical bytes directly, not only equal hashes.
- Hash is SHA-256 over canonical bytes.
- Same semantic nested object with different insertion order → same bytes/hash.
- Different value → different hash.

### Policy requirements

- Validation precedes classification; classification precedes policy.
- `ToolPolicyInput.effect` and `ToolPolicyInput.replay` are non-optional. Type construction prevents omission; no runtime "missing metadata" case is required.
- Policy runs exactly once per preparation attempt.
- At least one deterministic conformance policy MUST depend on `EffectDescriptor` and/or `ReplayPolicy`.
- ToolId-only allow-list behavior MAY exist but cannot be the sole policy proof.
- Behavioral proof MUST show that substituting effect/replay metadata can change the decision and that policy observes the exact classified values before evaluation.
- Policy denial returns `NoEffect(Policy)` with stable `policy_denied` code/message and invokes no effect.

### Cancellation/execution contract requirements

The tool execution contract MUST make these normal outcomes machine-distinguishable:

- completed output;
- completed ordinary tool error;
- cooperative cancellation.

`ToolExecutionError` MUST NOT encode cancellation. Panic/impossible invariant failure is outside the normal outcome contract.

T007 proves representability and direct fixture behavior. It does not claim Agent durability ordering around execution.

### Q008 — fixed failure-stage taxonomy

Policy denial:

- `ToolCallPrepared::NoEffect.failed_at = Policy`;
- disposition carries `PolicyDecision::Deny`;
- final attempt record is existing `ToolAttemptDenied`;
- do **not** add `failed_at` to `ToolAttemptDenied`.

Resolve/Validate/Classify failure:

- final attempt record is `ToolAttemptRejected`;
- `failed_at` identifies the actual failed stage.

### Work units

1. Identity, registry, configured-order projection, schema profile.
2. Canonical bytes/digest and single preparation seam returning the tools-owned preparation disposition.
3. Effect/replay-aware policy and `test.echo` / `test.unsafe_once` fixtures.
4. Distinct tool execution outcome contract.
5. Agent durable grammar/reducer embedding the tools-owned preparation types, plus corruption/reopen tests.

### Acceptance criteria

- [x] tools crate dependency direction matches SPEC §5.
- [x] one production preparation seam exists and core has no competing preparation pipeline.
- [x] `ToolPreparationDisposition` and `ToolPreparationStage` are tools-owned; core embeds those exact types without duplicate definitions or tools→core dependency.
- [x] exact configured pin/version resolution rejects missing/mismatched tools.
- [x] configured order is proved independently of registration order.
- [x] malformed schema rejects registration.
- [x] non-fragment `$ref` and `$dynamicRef` reject before compile with no retrieval.
- [x] `$id` alone does not cause rejection/retrieval.
- [x] local fragment/dynamic reference fixture validates.
- [x] validation is exact/non-coercing and raw provider JSON is never reparsed.
- [x] invalid args invoke classification 0, policy 0, execution 0.
- [x] nested canonical bytes and digest behavior are explicitly proved.
- [x] preparation order and short-circuit behavior are observed through the production seam.
- [x] policy input structurally requires effect/replay; behavioral proof covers exact observation and substituted classification values.
- [x] policy denial invokes execution/probe 0 and exposes stable `policy_denied` no-effect data.
- [x] `test.echo` is deterministic/Safe; `test.unsafe_once` is probe-observable/Never.
- [x] tool execution API distinguishes output, ordinary error, and cancellation; panic/invariant is outside normal outcomes.
- [x] durable history/reducer can represent executable, rejected, denied, effect-in-flight, completed-awaiting-result, settled, interrupted, and intervention states.
- [x] `ToolEffectStarted` is treated only as an ambiguity marker.
- [x] Q008 taxonomy is enforced with no duplicate denial stage field.
- [x] reducer rejects SPEC §7 corruption cases and reconstructs identically after reopen.
- [x] T007 does **not** claim a real kernel effect was invoked only after durable commit.
- [x] no concrete capability, approval workflow, parallel scheduling, generic policy engine, or T008 kernel loop is added.

### Verification

```bash
cargo test -p ww-agent-tools --test preparation --locked
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
# then complete permanent D017 gate
```

The focused preparation target MUST call the production preparation seam. Direct isolated helper tests alone do not satisfy the Task.

**Likely files:** root Cargo files; `crates/ww-agent-tools/**`; Agent history/reducer tests; existing CI boundary block only when required. Exact module/file names are not acceptance criteria.

**Estimated scope:** Large; use the work units above. Do not split Task identity unless an existing Stop Condition requires it.

## T008 — Functional recorded-provider kernel

**Outcome:** Implement the smallest real durable Agent driver and prove actual provider/tool sequencing over the completed T002–T007 ports.

### Hard architecture requirements

- Kernel dependencies are injected; core constructs no database or transport.
- Kernel uses the single T007 preparation seam and the tools-owned preparation disposition/stage types.
- Kernel never reimplements preparation or defines a duplicate preparation taxonomy.
- Provider-visible tool specs come from ordered run pins, not registration order.
- Finalized assistant state commits before tool handling.
- No-effect paths persist one model-visible error result and invoke execution zero times.
- Allowed effects execute only after the append containing `ToolEffectStarted` commits.
- Output / OrdinaryError / Cancelled / panic-invariant paths remain distinct.
- Tool calls execute sequentially and model-visible results remain in provider source order.

### Work units

1. Typed run configuration and ordered request projection.
2. Provider attempt durability + mandatory stream drain/finalization.
3. No-effect settlement through T007 preparation.
4. Commit-before-effect + execution outcome mapping.
5. Turn commit and text-only / model→tool→model fixtures.

### Acceptance criteria

- [ ] typed stored configuration decodes before provider/tool work.
- [ ] exact run tool pins are available and provider request tool specs follow configured order despite different registry order.
- [ ] provider/model/request digest state commits before provider I/O.
- [ ] outer provider-dispatch error creates one typed attempt failure and no assistant/tool preparation/effect.
- [ ] stream drains through EOF and finalizes exactly once.
- [ ] malformed/truncated/interrupted streams create no assistant entry/effect.
- [ ] finalized assistant entry commits before tool handling.
- [ ] logical call IDs allocate once in source order and survive reconstruction.
- [ ] kernel invokes the T007 preparation seam exactly once per preparation attempt.
- [ ] invalid/unknown/classification/policy-denied calls execute 0 and durably create exactly one ordered model-visible error result.
- [ ] before any allowed fixture executor call, `ToolAttemptStarted + ToolCallPrepared::Executable + ToolEffectStarted` append commits successfully.
- [ ] an injected probe demonstrates no executor call can occur before that commit.
- [ ] completed output persists effect result then one reserved model-visible result.
- [ ] ordinary returned tool error persists `ToolEffectCompleted::Error` then one model-visible `is_error=true` result and may continue to next model request.
- [ ] cooperative cancellation is not persisted/presented as ordinary tool error; it enters interruption/control flow for T009 settlement.
- [ ] panic/impossible invariant is not normalized to a normal tool result.
- [ ] allowed tools execute sequentially and next request sees source-ordered results.
- [ ] turn commits after all results and before next provider request.
- [ ] text-only and `model → test.echo → model` runs produce expected Agent terminal result.
- [ ] Length is not successful.
- [ ] optimistic conflict discards stale decision and launches no external work before reload/reduction.
- [ ] no common lifecycle, limits, restart matrix, public SDK/CLI, concrete provider, or capability scope is pulled forward.

### Verification

```bash
cargo test -p ww-agent-core --test kernel --locked
cargo test -p ww-agent-provider --test recorded_provider --locked
cargo test -p ww-agent-tools --locked
# then complete permanent D017 gate
```

**Likely files:** Agent core kernel/request/history/reducer/lib/tests and minimal supporting contract files.

## T009 — Common lifecycle and durable cancellation

**Outcome:** Bind the functional kernel to G002 lifecycle/cancellation without moving Agent semantics into shared runtime.

### Acceptance criteria

- [ ] exactly one Agent run maps to one common execution of kind `agent`.
- [ ] Pending starts once; Running/Waiting resumes; matching terminal performs no work.
- [ ] durable cancellation commits before live root-token signal.
- [ ] repeated registrations share one root; consumers cannot cancel siblings.
- [ ] cancellation before launch invokes provider/tool 0.
- [ ] active provider/tool receives child cancellation token.
- [ ] durable cancellation is rechecked immediately before `ToolEffectStarted`; an already-durable request prevents marker/execution.
- [ ] tool Cancelled outcome is never rewritten into ordinary tool error.
- [ ] Safe post-start cancellation is interrupted; caller cancellation does not silently retry.
- [ ] Never post-start/no-completion cancellation settles `RequiresIntervention`.
- [ ] completed durable result survives later cancellation and is repaired.
- [ ] Agent terminal maps to matching common status.
- [ ] Agent-terminal/common-nonterminal repair is idempotent and calls provider/tool 0.
- [ ] shared runtime contains no Agent DTO/semantic type.
- [ ] fixed SPEC recovery precedence is identical before/after reopen.

### Verification

```bash
cargo test -p ww-agent-core --test lifecycle --locked
cargo test -p ww-runtime --locked
cargo test -p ww-agent-store-sqlite --test coordinator --locked
# then permanent gate
```

## T010 — Durable deadlines and execution budgets

**Outcome:** Make provider/tool launches conditional on decisions reconstructed from durable history.

### Acceptance criteria

- [ ] count limits are positive and typed.
- [ ] linked G002 deadline is canonical; Agent snapshot mismatch fails closed before work.
- [ ] model requests count durable starts.
- [ ] completed-model-turn count is distinct from T003 `turn_count`.
- [ ] logical-tool-call count is distinct from T003 `tool_attempt_count`.
- [ ] counters reconstruct identically after reopen.
- [ ] token-limit configuration rejects usage-incapable provider/model before I/O.
- [ ] declared usage capability with missing finalized usage fails closed before next request.
- [ ] complete finalized assistant tool-call batch is admitted before any call preparation/execution.
- [ ] over-budget batch prepares/executes zero calls and settles `BudgetExhausted`.
- [ ] exactly fitting batch is admitted as a whole.
- [ ] model request `limit + 1` never launches.
- [ ] `now >= deadline` prevents launch; active expiry cancels child work.
- [ ] token usage accumulates from finalized normalized usage.
- [ ] reaching token limit stops before next provider request.
- [ ] BudgetExhausted and TimedOut remain explicit unless Never ambiguity outranks them.

### Verification

```bash
cargo test -p ww-agent-core --test limits --locked
cargo test -p ww-runtime --locked
# then permanent gate
```

## T011 — Crash/restart and ambiguous-effect recovery matrix

**Outcome:** Prove the durability thesis in distinct OS processes.

### Acceptance criteria

- [ ] every F1–F8 state is constructed at the exact boundary.
- [ ] resume occurs in a new process against the same SQLite database.
- [ ] F2 covers pre-event and transient-partial-delta loss; neither creates durable assistant entry.
- [ ] F3 makes zero provider calls before pending tool/terminal work.
- [ ] F4 Safe ambiguity retries as a new attempt with one logical result.
- [ ] F5 Never ambiguity executes zero additional effects and settles RequiresIntervention.
- [ ] F6 repairs reserved result without execution.
- [ ] F7 appends one turn commit without provider/tool work.
- [ ] F8 terminalizes common once without provider/tool work.
- [ ] second restart is a no-op for effects/results/terminal events.
- [ ] corrupt histories outside the matrix fail closed.
- [ ] competing resume drivers cannot both authorize external work; stale writer reloads/reduces without invocation.

### Verification

```bash
cargo test -p ww-agent-store-sqlite --test recovery_matrix --locked
# then permanent gate
```

## T012 — EvaluationRuns and terminal review

**Outcome:** Pin the exact final code state, execute all deterministic Evaluations, and independently review architecture/recovery behavior.

### Acceptance criteria

- [ ] every active check in `EVALUATIONS.md` has a current passing EvaluationRun.
- [ ] every SPEC requirement family maps to Verification/Evaluation evidence.
- [ ] exact reviewed commit passes permanent local and hosted gates.
- [ ] review confirms no concrete provider/filesystem/process/network/product/Flow/Orchestration scope.
- [ ] review confirms no duplicate logical result or silent Never replay.
- [ ] review confirms common/Agent terminal consistency and idempotent repair.
- [ ] review traces T007–T012 to reference evidence, WorkWeave adaptation, and explicit deferrals.
- [ ] residual findings are classified without automatically changing the roadmap.
- [ ] no G003 Stop Condition remains active.
- [ ] requester explicitly accepts or rejects G003; acceptance is not inferred from `main`.

### Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```
