# Specification — G003 Durable Agent Kernel

- Version: `v3-candidate`
- Approval: `pending requester approval under resumed D022`
- Refinement: `D021` baseline + resumed `D022`
- Governing architecture: `ADR-0003`
- Supersedes: `SPEC v2` when approved
- Implementation code basis: T006-equivalent code; D021/D022 are planning/specification-only
- Completed-work boundary: T001–T006 retain their original accepted contracts and evidence

## 0. Document contract

This specification makes the remaining G003 work executable without changing the Goal boundary, ADR-0003, completed Task meanings, Task identifiers, or the WorkWeave domain model.

Normative words:

- **MUST / MUST NOT** — required for G003 acceptance.
- **SHOULD / SHOULD NOT** — strong default; deviation requires a recorded Task-review rationale.
- **MAY** — permitted but not required.

Authority:

```text
accepted Decisions + ADR-0003
        ↓
GOAL.md
        ↓
SPEC.md
        ↓
PLAN.md
        ↓
TASKS.md
        ↓
VERIFICATION.md / EVALUATIONS.md
        ↓
HANDOFF.md
        ↓
implementation
```

T001–T006 are frozen. This version applies prospectively to T007–T012.

### 0.1 D022 resumption

D022 remains the authorizing Decision. It is resumed rather than replaced.

The first D022 pass correctly identified useful T007 hardening but placed normative architecture only in `TASKS.md`. This v3 candidate reconciles that hardening into the full authority chain before implementation resumes.

D022 introduces **no new durable entity, relationship, lifecycle, authority, or record variant**. It clarifies:

- one production tool-preparation seam;
- ownership between `ww-agent-tools` and `ww-agent-core`;
- effect/replay-aware policy conformance;
- canonical JSON proof;
- configured tool ordering;
- the Draft 2020-12 offline reference profile;
- cancellation as a distinct execution outcome;
- T007 versus T008 proof ownership;
- Q008 failure-stage placement.

### 0.2 Reference evidence labels

- **Pi observed** — production Pi Agent behavior at the pinned revision.
- **Pi Harness observed** — future-Harness behavior at the pinned revision; useful evidence, not production parity.
- **WW adopted** — normative G003 behavior.
- **WW deferred** — deliberately outside G003.

## 1. Objective

G003 proves that one bounded probabilistic Agent execution can:

1. reconstruct its next safe action from durable history;
2. use a provider-neutral recorded model seam;
3. prepare and authorize deterministic/synthetic tool calls;
4. cross provider/effect ambiguity boundaries only after durable authorization commits;
5. recover without duplicate logical tool results or silent Never replay;
6. propagate cancellation and enforce durable limits;
7. settle one auditable Agent result consistently with one G002 execution.

Required walking skeleton:

```text
model → tool → model → terminal Agent result
```

## 2. Frozen and open scope

### 2.1 Frozen foundation

T001–T006 remain unchanged:

- ADR-0003 accepted and G003 active;
- provider-neutral protocol and fail-closed stream assembler;
- immutable Agent context entries and operational records;
- pure recovery reducer;
- Agent-owned SQLite persistence;
- common/Agent atomic creation/link coordination;
- deterministic `RecordedProvider` fixtures.

### 2.2 Open implementation scope

| Task | Capability |
| --- | --- |
| T007 | tool contract, offline schema, preparation seam, policy/replay fixtures, durable tool grammar and reducer |
| T008 | smallest functional recorded-provider model/tool loop; real commit-before-effect execution proof |
| T009 | G002 lifecycle binding, durable cancellation, terminal repair |
| T010 | durable deadlines and execution budgets |
| T011 | distinct-process crash/restart and ambiguity matrix |
| T012 | exact-code EvaluationRuns and terminal architecture/recovery review |

### 2.3 Explicit exclusions

G003 MUST NOT add:

- concrete network providers or credentials;
- public filesystem/process/network/MCP/plugin/A2A capabilities;
- SDK/CLI/TUI/server product surfaces;
- Flow/OWS or WorkWeave Orchestration semantic types;
- parallel tool scheduling;
- sessions, steering/follow-up queues, branching, compaction, subagents, or dynamic tool installation;
- generic durable-format migration infrastructure assigned to proposed G010.

## 3. Settled assumptions

1. The parsed `serde_json::Value` produced by T002 is the sole executable tool-argument authority.
2. Raw provider JSON is diagnostic provenance only and MUST NOT be reparsed for validation, digesting, policy, or execution.
3. Validation is non-coercing and injects no defaults.
4. G003 tool schemas use JSON Schema Draft 2020-12 and are self-contained.
5. G003 policy is `Allow | Deny`; approval workflows remain later.
6. G003 replay policy is `Safe | Never`; idempotency-key replay remains later.
7. G003 executes tool calls sequentially in provider source order.
8. Registration/availability order is not run configuration order.
9. Provider-declared failure does not trigger automatic transient retries in G003.
10. `CompletionReason::Length` is auditable but not successful.
11. Same-code reopen/restart is mandatory; generic old-format migration is G010.
12. The linked G002 `ExecutionRecord.deadline` is the single deadline authority; any Agent copy is a matching snapshot only.
13. Token limits require normalized usage observability.
14. `max_tool_calls` counts logical model-requested calls, not execution attempts.
15. Ordinary tool failure, tool cancellation, and panic/invariant failure are three distinct semantic paths.
16. `ToolEffectStarted` means the durable ambiguity boundary after which execution **may** have occurred; it is not proof that an external effect occurred.
17. Q008 is resolved: Policy-stage failure lives in `ToolCallPrepared::NoEffect.failed_at`; `ToolAttemptDenied` does not gain a duplicate stage field.

## 4. Reference-derived architecture

### 4.1 Pi production Agent

**Pi observed:** production Pi keeps a small functional model/tool loop and separates tool preparation from execution. Preparation resolves the tool, validates arguments, applies preflight policy, and either produces an immediate error result or a prepared invocation. Execution and result finalization happen afterward. Tool results remain in assistant source order.

**WW adopted:** preserve the same separation, but add explicit effect/replay classification and durable authorization:

```text
prepare
    resolve exact pinned tool
    → validate exact parsed value
    → canonicalize/digest
    → derive effect
    → derive replay policy
    → evaluate centralized policy
    → return typed preparation disposition

then, in the Agent kernel
    persist Agent-owned preparation/effect boundary
    → commit
    → execute or settle no-effect
    → persist normalized outcome
    → expose one ordered model-visible result
```

**WW deferred/rejected:** Pi argument coercion, postflight mutation, parallel execution, queues, session façade, product hooks, and coding-agent capabilities.

### 4.2 Pi Harness

**Pi Harness observed:** entries and operational records are separate; tool-start records correlate assistant entry, source index, effective arguments, result identity, and replay safety; a pure reducer rejects impossible histories and reconstructs unresolved work.

**WW adopted:** immutable context entries, Agent-owned operational records, reserved result identity, append-only attempts, explicit replay safety, pure reduction, and fail-closed corruption handling.

**WW adaptation:** WorkWeave records a stronger pre-effect grammar—effect descriptor, policy decision, and an explicit ambiguity marker—because restart safety is a first-class G003 requirement.

### 4.3 WorkWeave architectural rule

The tool subsystem decides **whether a call is executable**. The Agent kernel decides **whether execution is durably authorized now**.

Do not merge those responsibilities.

## 5. Container and dependency architecture

```text
RecordedProvider (ww-agent-provider)
          ↓
Agent kernel (ww-agent-core)
    ↙             ↘
Tool subsystem     AgentStore port
(ww-agent-tools)         ↓
                  ww-agent-store-sqlite
                         ↓
                    G002 runtime
```

Required dependency direction:

- `ww-agent-provider` MUST NOT depend on Agent core, tools, runtime, persistence, Flow, or transport.
- `ww-agent-tools` MAY depend on utility/schema/async crates only; it MUST NOT depend on Agent core, runtime/store, SQLite, filesystem/process/network libraries, Flow, or Orchestration.
- `ww-agent-core` MAY depend on provider, tools, and generic G002 runtime APIs; it MUST NOT depend on SQLite, concrete transport/capabilities, Flow, CLI, or Orchestration.
- `ww-agent-store-sqlite` owns Agent persistence and bounded backend coordination.
- Agent DTOs MUST NOT enter `ww-store`.
- G003 MUST NOT add a generic policy crate or capability-specific tool crate.

Agent operational identities remain owned by `ww-agent-core`. Public tools APIs MUST NOT depend on Agent run/call/attempt/entry identities.

## 6. Tool subsystem contract — T007

### 6.1 Required semantic contracts

T007 MUST provide WorkWeave-owned equivalents of the existing v2 concepts:

- `ToolId`, `ToolVersion`, `ToolIdentity`, `ToolSpec`;
- immutable `ToolRegistry`;
- `EffectDescriptor`;
- `ReplayPolicy::Safe | Never`;
- `PolicyDecision::Allow | Deny`;
- `ToolPolicy`;
- `ToolRequest`, `ToolContext`, `ToolOutput`, and ordinary `ToolExecutionError`;
- one async tool execution contract;
- one production tool-preparation seam.

The durable semantic names `ToolPreparationDisposition::{Executable, NoEffect}` and `ToolPreparationStage::{Resolve, Validate, Classify, Policy}` are canonical for the persisted/reducer vocabulary.

Function names and module/file placement are **not** architecture authority. A function named `prepare_tool_call` and a `preparation.rs` module are preferred conventional choices, not acceptance requirements. Equivalent naming/layout is allowed when there remains exactly one production preparation seam and no competing semantic taxonomy.

Do not introduce a generic `ToolManager`, `PolicyEngine`, orchestration object, or run-stateful registry.

### 6.2 One production preparation seam

`ww-agent-tools` MUST own one production seam that performs this exact semantic order:

```text
resolve exact tool/version
→ validate authoritative parsed Value
→ derive deterministic canonical bytes/digest
→ derive EffectDescriptor
→ derive ReplayPolicy
→ evaluate ToolPolicy
→ return Executable or NoEffect
```

Rules:

- If a stage fails, later stages MUST NOT run.
- `ww-agent-core` MUST NOT duplicate resolution, validation, digesting, effect/replay classification, or policy evaluation.
- The preparation result MUST carry enough stable data for core to persist the already-defined `ToolCallPrepared` record.
- A non-serializable executor handle MAY be retained internally for later execution, but MUST NOT create a second durable/public disposition model.
- Preparation performs no external effect and owns no Agent operational identity.

### 6.3 Identity, registry, and configured order

- `ToolId` and `ToolVersion` MUST be non-empty.
- Fixture IDs remain exactly `test.echo` and `test.unsafe_once`, both version `1`.
- Duplicate `ToolId` registration rejects before a run.
- Exact version mismatch rejects; no silent substitution.
- The registry is immutable for one Agent run and owns no run state.
- **Run configuration order is authoritative for model-visible tool order. Registration or availability order has no authority.**
- Given an explicit ordered pin list, registry projection/resolution MUST return only those exact pins in that order.
- A conformance fixture MUST use different registration and configured orders to prove this distinction.

### 6.4 Offline JSON Schema profile

T007 pins `jsonschema 0.52.1` with default resolver features disabled.

Normative profile:

- Draft 2020-12 selected explicitly.
- G003 fixture schema roots are objects.
- `$ref` and `$dynamicRef` MUST be self-contained fragment references only for G003.
- A non-fragment `$ref` or `$dynamicRef` MUST reject before validator compilation with a WorkWeave-owned tool-definition error.
- Local fragment references, including local `$dynamicRef`/`$dynamicAnchor`, MAY be used.
- `$id` is not rejected merely for existing. It MUST NOT relax the fragment-only reference rule or cause retrieval.
- HTTP/file retrieval is disabled.
- Malformed schemas reject registration.
- Validators compile once and are reused.
- Coercion/default injection is forbidden.
- `format` remains annotation-only for G003.
- Validation errors are WorkWeave-owned and deterministically ordered.
- `schemars` is not required in G003.

### 6.5 Canonical arguments and digest

- The exact parsed `Value` is passed unchanged through validation, classification, policy, and execution.
- Deterministic canonical bytes use compact JSON with object keys recursively sorted.
- `arguments_digest` is SHA-256 over those canonical bytes.
- Nested objects differing only in key insertion order MUST produce identical canonical bytes and digest.
- A semantically different value MUST produce a different digest.
- Tests MUST inspect canonical bytes directly; digest equality alone is insufficient.
- This is an internal G003 audit/recovery identity, not a cross-language canonicalization standard.

### 6.6 Effect, replay, and policy

- Classification occurs only after successful validation.
- `test.echo` is `Pure` and `Safe`.
- `test.unsafe_once` is `Synthetic` and `Never`.
- A tool cannot authorize itself.
- Policy runs only after validation and effect/replay classification.
- Policy runs exactly once per preparation attempt.
- At least one deterministic conformance policy MUST make its decision depend on `EffectDescriptor` and/or `ReplayPolicy`; a ToolId-only allow list cannot be the sole policy proof.
- `Deny` returns a `NoEffect` preparation with stable `policy_denied` code/message and performs no effect.
- Approval-bearing policy remains outside G003.

### 6.7 Tool execution outcome and cancellation

The tool execution API MUST make these outcomes machine-distinguishable at the `ww-agent-tools` → `ww-agent-core` boundary:

1. **Output** — completed tool output;
2. **OrdinaryError** — completed ordinary tool failure represented by `ToolExecutionError`;
3. **Cancelled** — cooperative cancellation/control outcome, not `ToolExecutionError`.

Equivalent idiomatic Rust encodings are allowed. The semantic distinction is mandatory.

Rules:

- `ToolContext` supplies the cancellation token.
- `ToolExecutionError` MUST NOT encode cancellation.
- Panic/impossible invariant/contract violation is not a normal tool outcome and MUST NOT be converted into Output, OrdinaryError, or Cancelled.
- T007 proves the contract can represent all three normal outcomes.
- T008 proves the kernel preserves the distinction.
- T009 owns durable cancellation intent, common lifecycle mapping, and final replay-sensitive cancellation settlement.

### 6.8 Fixture behavior

`test.echo`:

- requires exactly `value`;
- rejects extra properties;
- returns the same value structurally;
- has no external effect;
- is `Safe`.

`test.unsafe_once`:

- requires a non-empty string `key`;
- rejects extra properties;
- invokes an injected test-only probe once per actual `execute` call;
- returns `applied=true` plus the key when completed;
- is `Never`;
- exposes no public filesystem/process/network capability.

T007 unit tests MAY call fixture execution directly to prove fixture semantics. Such tests do **not** constitute commit-before-effect proof.

## 7. Durable tool grammar and reducer — T007

### 7.1 Stable Agent-owned identities

For each finalized provider tool call, core owns:

- stable logical call identity;
- unique attempt identity per handling/execution attempt;
- reserved result-entry identity;
- assistant entry identity and source index;
- provider call ID and requested name.

### 7.2 Existing durable vocabulary remains authoritative

D022 adds no durable record variant.

T007 retains the v2 record grammar:

- `ToolAttemptStarted`;
- `ToolCallPrepared` with `Executable | NoEffect`;
- `ToolEffectStarted`;
- `ToolEffectCompleted` with normalized Output/Error;
- `ToolAttemptRejected`;
- existing `ToolAttemptDenied`;
- `ToolAttemptCompleted`;
- `ToolAttemptInterrupted`;
- `ToolAttemptIntervention`.

Before an allowed effect may execute in T008, durable state must be able to contain:

- source/call/attempt/reserved-result identities;
- exact tool identity/version;
- canonical arguments digest;
- effect descriptor;
- replay policy;
- policy decision;
- explicit `ToolEffectStarted` ambiguity marker.

T007 proves this grammar and its reduction. T008 proves the production kernel commits it before execution.

### 7.3 No-effect taxonomy — Q008

For Resolve, Validate, or Classify failure:

```text
ToolCallPrepared::NoEffect.failed_at = actual stage
→ ToolAttemptRejected.failed_at = same stage
```

For policy denial:

```text
ToolCallPrepared::NoEffect.failed_at = Policy
ToolCallPrepared::NoEffect.policy = Deny
→ ToolAttemptDenied
```

`ToolAttemptDenied` MUST NOT gain a duplicate `failed_at` field.

No-effect preparation carries stable code/message sufficient for T008 to create exactly one model-visible error result. T007 does not claim the production result was persisted.

### 7.4 Reducer invariants

The reducer MUST reject:

- preparation for an unknown logical call/non-current assistant entry;
- duplicate preparation of one attempt;
- conflicting tool/version/digest/effect/replay/policy across attempts of one logical call;
- effect start after `NoEffect` or before executable preparation;
- effect completion without effect start;
- wrong reserved result identity;
- rejection/denial taxonomy mismatch;
- more than one model-visible result per logical call;
- source-order violations;
- records after terminal Agent result.

The reducer MUST distinguish:

- prepared executable;
- no-effect rejected/denied;
- effect in flight/ambiguous;
- completed effect awaiting result repair;
- settled logical result;
- interrupted Safe attempt;
- Never intervention.

### 7.5 T007/T008 proof boundary

T007 MUST prove:

- preparation ordering and short-circuiting;
- schema/policy/replay/canonicalization semantics;
- fixture behavior;
- cancellation outcome representability;
- durable tool grammar and reducer reconstruction/corruption behavior.

T007 MUST NOT claim that a real kernel effect was invoked only after a durable commit.

T008 MUST prove:

- core consumes the single preparation seam;
- no-effect results are durably settled once with zero execution;
- `ToolEffectStarted` is committed before any allowed fixture execution;
- ordinary error, cancellation, and panic/invariant paths remain distinct;
- ordered model-visible results reach the next provider request.

## 8. Functional Agent kernel — T008

### 8.1 Shape

`ww-agent-core` owns a small functional driver over injected provider, tool registry/policy, AgentStore, ID/clock sources, cancellation token, and typed configuration.

It is not a session object and owns no transport, database construction, UI, Flow, or Orchestration concerns.

### 8.2 Run configuration and tool ordering

The typed run configuration pins provider, model, system prompt, ordered tool identities, and limits.

Rules:

- decode/validate configuration before provider/tool work;
- unavailable exact tool pin fails before work;
- provider-visible tool specs follow the run's ordered pin list exactly;
- registry registration order MUST NOT leak into the request;
- an integration fixture MUST register tools in a different order from the run pins and verify the request follows run pins.

### 8.3 Provider boundary

Before provider I/O, persist model-attempt identity, request ordinal, provider/model pin, and canonical request digest.

The production stream path MUST drain through EOF and finalize exactly once. Unexpected EOF, stream error, post-terminal event, malformed order, incomplete/truncated tool call, or provider failure creates no assistant entry/effect.

A finalized assistant entry and model-completion record commit before tool handling.

### 8.4 Tool loop

For each admitted logical call in provider source order:

```text
call single tools preparation seam

if NoEffect:
    atomically persist handling start
    + ToolCallPrepared::NoEffect
    + one reserved model-visible error entry
    + Rejected or Denied terminal attempt record
    commit
    execute zero times

if Executable:
    atomically persist handling start
    + ToolCallPrepared::Executable
    + ToolEffectStarted
    commit
    only then call execute once
    persist outcome according to §8.5
```

Core MUST NOT reimplement preparation logic.

### 8.5 Execution outcome mapping

After `ToolEffectStarted` commits:

- **Output** → append `ToolEffectCompleted::Output`, then the reserved success result and `ToolAttemptCompleted`.
- **OrdinaryError** → append `ToolEffectCompleted::Error`, then exactly one reserved model-visible `is_error=true` result and `ToolAttemptCompleted`; the model loop may continue.
- **Cancelled** → do not normalize into `ToolEffectCompleted::Error` or a model-visible ordinary tool error; append interruption/control state as permitted by existing durable vocabulary and hand settlement to T009 rules.
- **panic/invariant violation** → do not normalize; fail/crash the kernel and recover only from the last committed durable boundary.

If cancellation is already observable before `ToolEffectStarted` is committed, the marker and execution MUST NOT occur.

### 8.6 Turn and terminal behavior

- Tool calls execute sequentially.
- Model-visible tool results preserve provider source order.
- `TurnCommitted` follows all durable results and precedes the next provider request.
- Text-only Stop commits successful Agent result.
- `model → test.echo → model` commits successful Agent result.
- Length is audited but not successful.
- T008 does not own common G002 terminalization; T009 adds that binding.

### 8.7 Optimistic conflict rule

Each mutation cycle reads one versioned Agent snapshot, reduces it, proposes one append/external operation, and commits the authorizing append before external work.

If expected-version append loses:

```text
conflict → discard stale decision → reload/reduce → no external work from stale decision
```

No new lease subsystem is introduced.

## 9. Common lifecycle and cancellation — T009

### 9.1 Identity and start

- One Agent run maps to exactly one G002 common execution of kind `agent`.
- Missing/mismatched/non-agent link rejects before work.
- Pending starts once; Running/Waiting resumes; matching terminal performs no work.

### 9.2 Durable cancellation

- Durable `request_cancel` commits before live root-token signal.
- Repeated registrations observe one root; consumers receive child tokens.
- Provider/tool work checks cancellation before launch.
- Cancellation is checked again immediately before `ToolEffectStarted` commit.
- Cancellation before that commit prevents marker and execution.
- Cancellation after effect-start cannot erase ambiguity.

### 9.3 Replay-sensitive cancellation

- provider cancelled before final response → no assistant entry; Agent Cancelled;
- Safe tool Cancelled after effect-start → interrupted; caller cancellation remains terminal, no silent replay;
- Never tool Cancelled after effect-start without durable completion → `RequiresIntervention`;
- already durable completion is repaired rather than discarded.

### 9.4 Terminal mapping and precedence

Agent terminal dispositions map one-to-one to common terminal statuses. Agent-terminal/common-nonterminal repair is idempotent and invokes no provider/tool.

Recovery precedence:

1. corruption/contradiction;
2. existing Agent terminal result;
3. Never effect ambiguity;
4. durable cancellation;
5. expired canonical deadline;
6. exhausted budget;
7. otherwise next permitted action.

## 10. Deadlines and execution budgets — T010

### 10.1 Deadline authority

The linked common `ExecutionRecord.deadline` is canonical. Any Agent deadline is a persisted matching snapshot only. Mismatch fails closed before work.

### 10.2 Durable counters

Derive from durable history:

- model request starts;
- completed model turns;
- logical tool calls from finalized assistant responses;
- input/output/total normalized tokens.

Do not repurpose T003 `turn_count` or `tool_attempt_count` for different semantics.

### 10.3 Enforcement

Before provider launch:

- cancellation/deadline checks;
- model/turn capacity;
- usage capability when token limits configured;
- durable attempt reservation.

Before tools from one finalized assistant response:

- compute whole source-ordered logical-call batch;
- admit whole batch or execute none;
- an over-budget batch settles `BudgetExhausted`.

After provider usage:

- missing promised usage fails closed before another request;
- reaching token limit prevents next request.

`now >= deadline` is expired. Never ambiguity outranks timeout/cancel/budget.

## 11. Recovery and fault matrix — T011

Required fault states remain F1–F8:

| Fault | Required restart behavior |
| --- | --- |
| F1 creation committed | continue same run once |
| F2 model attempt started/no final response | interrupt/new attempt only when permitted |
| F3 finalized assistant durable | no provider repeat; process pending tools/terminal |
| F4 Safe effect-start/no completion | new audited attempt; one logical result |
| F5 Never effect-start/no completion | no execution; RequiresIntervention |
| F6 effect completion durable/result entry absent | repair reserved result; no execute |
| F7 all tool results durable/turn absent | append one turn commit |
| F8 Agent terminal/common nonterminal | terminalize common once |

Tests run in distinct OS processes against the same SQLite database and include a second restart proving idempotency.

States outside the explicit repair matrix fail closed.

## 12. Evaluations and terminal review — T012

Record current EvaluationRuns for:

1. Agent protocol conformance;
2. Tool preparation and policy conformance;
3. Agent durable recovery safety;
4. Agent kernel execution conformance;
5. terminal architecture/scope review.

Each run pins exact commit, command/fixture, deterministic mode, result, date, and evidence.

Goal acceptance remains a separate requester action.

## 13. Project structure

Expected responsibilities:

```text
ww-agent-tools
    identity/schema/registry/policy/preparation/fixtures responsibilities

ww-agent-core
    history/reducer/kernel/limits/lifecycle responsibilities

ww-agent-store-sqlite
    persistence/coordinator + test-only restart driver
```

Exact source file/module names MAY vary. Do not create extra layers merely to match this diagram.

One Task/work unit should remain reviewable in one focused engineering session. Roughly five implementation files is a decomposition signal, not a prohibition.

## 14. Commands

Permanent gate:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Focused commands are defined in `TASKS.md` and MUST remain additive to the permanent D017 gate.

## 15. Testing strategy

- Tool unit/contract tests: identity, schema, canonical bytes/digest, configured-order projection, preparation ordering, effect-aware policy, execution outcome distinction, fixtures.
- Reducer tests: durable grammar, valid reconstruction, every corrupt-history case.
- Kernel integration tests: stream finalization, configured-order request projection, no-effect settlement, commit-before-effect probe, ordinary error versus cancellation/panic.
- Runtime integration tests: durable cancellation and terminal repair.
- Limit tests: canonical deadline, whole-batch admission, usage observability, durable counters.
- OS-process tests: F1–F8 and second-restart idempotency.

Tests MUST assert both the required result and prohibited provider/effect invocations.

## 16. Boundaries

### Normal path first

For one tool call:

```text
finalized provider call
→ prepare once in ww-agent-tools
→ persist Agent-owned decision state in ww-agent-core
→ commit
→ execute only when Executable
→ persist typed outcome
→ append one ordered model-visible result
```

### If condition, then action

- If validation fails, return `NoEffect(Validate)` and run no later preparation stage.
- If classification fails, return `NoEffect(Classify)` and do not evaluate policy.
- If policy denies, return `NoEffect(Policy)` and execute zero times.
- If preparation is Executable, core must commit pre-effect state before calling execute.
- If execution returns OrdinaryError, persist one model-visible ordinary tool error.
- If execution returns Cancelled, follow cancellation control semantics; do not create ordinary tool error.
- If expected-version append conflicts, discard the decision and reload before external work.
- If a Never effect is ambiguous, require intervention.

### Never

- never execute invalid or denied calls;
- never use registration order as run tool order;
- never silently retrieve external schema references;
- never silently replay `ReplayPolicy::Never`;
- never create two model-visible results for one logical call;
- never treat EOF without finalization as success;
- never treat cancellation/panic/invariant failure as ordinary tool error;
- never let Agent DTOs enter shared `ww-store`;
- never mutate completed T001–T006 semantics/evidence.

## 17. Normative requirement index

### Tool requirements

| ID | Requirement |
| --- | --- |
| TOOL-01 | Stable non-empty identity/version and exact run pinning |
| TOOL-02 | Immutable registry; run configured pins, not registration order, define model-visible tool order |
| TOOL-03 | Explicit self-contained Draft 2020-12 schema compiled at registration |
| TOOL-04 | Non-fragment `$ref` and `$dynamicRef` are rejected before compilation; `$id` alone is not forbidden |
| TOOL-05 | Exact non-coercing argument validation with WorkWeave-owned errors |
| TOOL-06 | Parsed `Value` is the sole executable argument authority |
| TOOL-07 | Deterministic nested canonical JSON bytes and SHA-256 digest |
| TOOL-08 | Validate → digest → effect/replay → policy ordering with short-circuiting |
| TOOL-09 | Centralized deterministic Allow/Deny policy; effect/replay-aware conformance is required |
| TOOL-10 | Preparation denial/rejection performs zero effect and exposes stable no-effect code/message |
| TOOL-11 | `test.echo` is deterministic, pure, Safe |
| TOOL-12 | `test.unsafe_once` is synthetic, probe-observable, Never |
| TOOL-13 | Public tools contracts contain no Agent-owned operational identity/core dependency |
| TOOL-14 | Exactly one production tool-preparation seam exists in `ww-agent-tools`; core does not duplicate it |
| TOOL-15 | Tool execution distinguishes Output, OrdinaryError, and Cancelled; panic/invariant is outside normal outcomes |

### Durability requirements

| ID | Requirement |
| --- | --- |
| DUR-01 | Logical call/attempt/source/provider/reserved-result identities are stable |
| DUR-02 | Existing record grammar can represent tool/version/digest/effect/replay/policy plus explicit effect-start ambiguity |
| DUR-03 | No-effect dispositions identify the failed stage and contain no effect-start/completion |
| DUR-04 | Effect completion may be durable before model-visible result for repair |
| DUR-05 | Safe interruption and Never intervention are distinct append-only outcomes |
| DUR-06 | Retries create new attempts; prior attempts are not rewritten |
| DUR-07 | Reducer fails closed on unknown/mismatched/duplicate/out-of-order history |
| DUR-08 | One logical call has at most one committed model-visible result in source order |
| DUR-09 | Optimistic conflict discards stale decisions before external work |
| DUR-10 | Rejected = Resolve/Validate/Classify; Denied = Policy; Q008 adds no duplicate field |

### Kernel requirements

| ID | Requirement |
| --- | --- |
| KERN-01 | Typed configuration and exact pins validate before work |
| KERN-02 | Provider request tool specs follow ordered run pins, independent of registry registration order |
| KERN-03 | Provider/model/request attempt state is durable before provider I/O |
| KERN-04 | Provider streams drain through EOF and finalize exactly once |
| KERN-05 | Finalized assistant state commits before tool handling |
| KERN-06 | Core consumes the single tool-preparation seam and does not duplicate preparation |
| KERN-07 | No-effect calls settle once with zero execution |
| KERN-08 | Allowed effect invocation occurs only after the append containing `ToolEffectStarted` commits |
| KERN-09 | Output/OrdinaryError/Cancelled/panic-invariant paths remain semantically distinct |
| KERN-10 | Tool results preserve source order and turn commit precedes next provider request |
| KERN-11 | Text-only and model→tool→model paths terminate explicitly; Length is not success |
| KERN-12 | Kernel owns no concrete transport, SQLite, capability, Flow, or product surface |
| KERN-13 | Outer provider-dispatch errors produce one durable failed/interrupted attempt and no assistant/effect |
| KERN-14 | Each mutation derives from one versioned snapshot; stale conflict launches no external work |

### Lifecycle requirements

| ID | Requirement |
| --- | --- |
| LIFE-01 | One Agent run links to one common `agent` execution |
| LIFE-02 | Pending starts once; Running/Waiting resumes; matching terminal performs no work |
| LIFE-03 | Durable cancellation request precedes live root-token signal |
| LIFE-04 | Provider/tool consumers receive cancellation children from one root |
| LIFE-05 | Cancellation before effect-start prevents marker and execution |
| LIFE-06 | Cancelled tool execution is not ordinary tool error; replay policy controls post-start ambiguity |
| LIFE-07 | Never ambiguity maps to RequiresIntervention |
| LIFE-08 | Agent/common terminal repair is idempotent and uses fixed recovery precedence |

### Limit requirements

| ID | Requirement |
| --- | --- |
| LIMIT-01 | Common deadline is authoritative; Agent deadline is matching snapshot only |
| LIMIT-02 | Model/turn/logical-tool/token counters derive from durable history without reinterpreting T003 counters |
| LIMIT-03 | Provider capacity and whole tool batches are checked before launch |
| LIMIT-04 | Provider `limit+1` never launches; over-budget tool batch executes zero calls |
| LIMIT-05 | Finalized provider usage accumulates durably |
| LIMIT-06 | Token limit stops before next provider request |
| LIMIT-07 | `now >=` deadline expires; active expiry cancels child work |
| LIMIT-08 | BudgetExhausted/TimedOut are explicit unless Never ambiguity outranks them |
| LIMIT-09 | Token limits require provider/model normalized usage capability before I/O |
| LIMIT-10 | Missing promised finalized usage fails closed before another request |
| LIMIT-11 | Safe retries remain attempt audit, not new logical-tool budget units |

### Recovery requirements

| ID | Requirement |
| --- | --- |
| REC-01…REC-08 | F1–F8 follow the explicit matrix in §11 |
| REC-09 | F1–F8 run across real OS-process restart; second restart is idempotent |
| REC-10 | States outside the matrix fail closed |
| REC-11 | F2 partial transient deltas never become durable assistant state |

### Evaluation requirements

| ID | Requirement |
| --- | --- |
| EVAL-01 | Current EvaluationRuns pin exact commit, command/fixture, mode, result, date, evidence |
| EVAL-02 | Every normative requirement maps to Verification/Evaluation evidence |
| EVAL-03 | Exact reviewed commit passes local/permanent and hosted gates |
| EVAL-04 | Independent review confirms architecture, scope, durability, replay boundaries |
| EVAL-05 | Goal acceptance is explicit requester action |

## 18. Requirement traceability

| Requirement family | Primary Task | Verification |
| --- | --- | --- |
| TOOL-01…TOOL-15 | T007 | `V-T007` |
| DUR-01…DUR-10 | T007/T008/T011 | `V-T007`, `V-T008`, `V-T011` |
| KERN-01…KERN-14 | T008 | `V-T008` |
| LIFE-01…LIFE-08 | T009 | `V-T009` |
| LIMIT-01…LIMIT-11 | T010 | `V-T010` |
| REC-01…REC-11 | T011 | `V-T011` |
| EVAL-01…EVAL-05 | T012 | `V-T012` |

## 19. Open questions

No unresolved technical question is intentionally left for the implementing agent in T007/T008.

This candidate remains under `REPLAN_LOCK` until requester approval. Later separately governed work may revisit durable format evolution, approval-bearing policy, idempotency keys, parallel tools, concrete providers, and real capabilities.
