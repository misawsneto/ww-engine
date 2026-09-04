# G003 Evaluations

- Version: `v2`
- Approval: `approved by requester 2026-09-04 under D021`
- Specification basis: `G003 SPEC v2`
- All EvaluationRuns required for closure must execute on the exact final reviewed commit.

## EvaluationRun record

Append each run directly under its corresponding check and record:

```markdown
### EvaluationRun: <stable-id>
- Check: <check id/name>
- Commit: <full SHA>
- Date: <ISO date/time>
- Mode: deterministic
- Command/fixture: <exact command and fixture>
- Result: pass | fail
- Evidence: <test/log/review location>
- Notes: <only material interpretation>
```

A passing run becomes stale when relevant code or the check contract changes.

## Agent protocol conformance

- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `stream-ordering`

- Covers: normalized provider protocol and mandatory kernel finalization.
- Subjects: `ww-agent-provider` assembler, RecordedProvider, T008 stream consumer.
- Procedure:
  1. run valid text/tool/usage/failure/cancellation fixtures;
  2. run delta-before-start, duplicate terminal, post-terminal, unexpected EOF, invalid JSON, incomplete tool, and length-truncated tool fixtures;
  3. assert no invalid fixture creates an assistant entry or invokes a tool.
- Expected:
  - valid streams finalize exactly once;
  - kernel drains to EOF and calls `finish`;
  - an outer provider-dispatch error and an in-stream failure both create typed attempt failures with no assistant entry or effect;
  - every invalid stream has one typed failure and zero effects.

### `provider-boundary`

- Covers: provider neutrality.
- Subjects: provider/core/tools dependencies and public types.
- Procedure:
  1. run source/dependency boundary checks;
  2. compile RecordedProvider against the same ModelProvider contract used by the kernel;
  3. inspect final crate graph during T012.
- Expected:
  - no vendor transport/request/response type crosses into Agent core;
  - no HTTP/credential dependency exists in G003;
  - tool request/context types contain no Agent-owned operational identity and create no tools→core dependency;
  - recorded and later concrete providers remain substitutable at the normalized seam.

## Tool preparation and policy conformance

- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `schema-policy-ordering`

- Covers: exact arguments, schema, effect/replay, and policy ordering.
- Subjects: `ww-agent-tools` registry/validator/policy/fixtures.
- Procedure:
  1. instrument validation, classification, policy, and execute calls;
  2. exercise valid, invalid, malformed-schema, external-ref, unknown-tool, allow, and deny cases;
  3. inspect counters, stable error output, and final attempt taxonomy.
- Expected:
  - order is validate → classify → policy → execute;
  - invalid/unknown/denied paths execute zero effects;
  - denial and validation errors are model-visible, typed, and deterministic;
  - resolve/validation/classification failures are Rejected and only policy failure is Denied;
  - no argument coercion occurs.

### `replay-metadata`

- Covers: durable preparation identity.
- Subjects: Agent records/reducer and fixture probes.
- Procedure:
  1. prepare Safe and Never calls;
  2. inspect history immediately before effect launch;
  3. reopen SQLite and reduce the same history.
- Expected:
  - source position, provider call ID, logical/attempt/reserved-result identities, pinned tool/version, argument digest, effect, replay, policy, and explicit effect-start state are durable;
  - reopened reduction is identical;
  - changed replay/policy or reserved result is rejected.

## Agent durable recovery safety

- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `recovery-reduction`

- Covers: restart reconstruction and corrupt-history behavior.
- Subjects: entries, records, SQLite store, AgentRecoveryState reducer.
- Procedure:
  1. persist canonical histories for every recovery phase;
  2. reopen in a new process and compare projections;
  3. run all `V-T007` reducer corruption fixtures;
  4. race two versioned append decisions and make the losing driver reload/reduce.
- Expected:
  - valid projections are identical;
  - impossible histories fail closed;
  - an optimistic loser performs no provider/tool work from stale state;
  - no next action depends on process-local state.

### `replay-safety`

- Covers: Safe versus Never effect ambiguity.
- Subjects: `test.echo`, `test.unsafe_once`, attempts, reserved result identity.
- Procedure:
  1. fault after durable `ToolEffectStarted` and before `ToolEffectCompleted` for Safe and Never;
  2. restart twice;
  3. inspect attempt history, effect probe, and model-visible results.
- Expected:
  - Safe creates a distinct retry attempt and one logical result;
  - Never effect count remains one and restart settles RequiresIntervention;
  - second restart creates no additional effect/result.

### `settlement-repair`

- Covers: Agent/common terminal consistency.
- Subjects: Agent terminal result and G002 ExecutionRecord.
- Procedure:
  1. fault after Agent result commit and before common terminalization;
  2. restart twice;
  3. inspect common events and provider/tool counters.
- Expected:
  - common terminal state is applied once;
  - provider/tool are not invoked;
  - Agent/common dispositions match.

## Agent kernel execution conformance

- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `text-only`

- Covers: minimum functional terminal run.
- Subjects: real kernel, RecordedProvider, AgentStore.
- Procedure: execute one user input against a recorded Stop response, reopen, and inspect.
- Expected:
  - one model request/assistant entry/result;
  - successful Agent terminal result;
  - no tool attempt;
  - identical reopened state.

### `model-tool-model`

- Covers: primary G003 walking skeleton.
- Subjects: real kernel, RecordedProvider, `test.echo`, policy, persistence.
- Procedure:
  1. first response requests one echo call;
  2. execute/commit result;
  3. assert second request contains the ordered tool result;
  4. final response stops;
  5. reopen and inspect.
- Expected:
  - one logical call and one model-visible result;
  - two model requests in order;
  - expected final output/result;
  - all durable boundaries present.

### `denial-and-tool-error`

- Covers: model-visible no-effect/error behavior and error taxonomy.
- Subjects: policy denial, invalid args, unknown tool, ordinary returned `ToolExecutionError`, cancellation, invariant failure.
- Procedure:
  1. run invalid, unknown, and policy-denied calls through the real kernel;
  2. run a fixture returning an ordinary typed `ToolExecutionError`;
  3. run a cancellable fixture and assert cancellation follows the cancellation path;
  4. exercise a test-only impossible invariant/contract failure and assert it is not normalized into an ordinary tool result.
- Expected:
  - zero effect for invalid/unknown/denied;
  - exactly one error result per invalid/unknown/denied call;
  - an ordinary returned execution error produces exactly one durable model-visible `is_error=true` result and the next provider request may observe it;
  - cancellation is not converted into ordinary tool error;
  - panic/impossible invariant failure is not presented to the model as a normal tool failure; recovery starts from the last durable boundary.

### `cancellation-limits`

- Covers: bounded execution.
- Subjects: common cancellation, canonical deadline, model/turn/logical-tool/token limits, provider usage capability.
- Procedure:
  1. block provider and Safe/Never tools;
  2. cancel or expire the common `ExecutionRecord.deadline`;
  3. run exact model/turn/logical-tool boundaries and `limit + 1` cases;
  4. run multi-call assistant batches exactly fitting and exceeding remaining logical tool-call capacity;
  5. configure token limits with usage-capable and usage-incapable provider/model fixtures, plus a fixture that declares usage capability but omits finalized usage;
  6. combine cancel, deadline, budget exhaustion, and Never ambiguity in the same reduced states;
  7. reopen terminal histories.
- Expected:
  - active child token receives cancellation;
  - common `ExecutionRecord.deadline` is authoritative and a mismatching Agent deadline snapshot fails closed before work;
  - no provider request beyond its limit launches;
  - an over-budget logical tool-call batch executes zero tools; an exactly fitting batch is admitted as a whole in source order;
  - token-limit configuration rejects when usage capability is unavailable;
  - declared usage capability with omitted finalized usage fails closed before another provider request;
  - token limit stops before next provider call;
  - Never ambiguity requires intervention;
  - simultaneous conditions select the same SPEC §9.6 disposition before and after reopen;
  - terminal state is durable and common/Agent-consistent.

## Recovery fault matrix

- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `fault-boundaries-F1-F8`

- Covers: every ambiguity-sensitive commit/effect boundary in SPEC §11.
- Subjects: test-only process fixture, SQLite state, RecordedProvider journal, unsafe-effect probe.
- Procedure:
  1. execute F1–F8 in distinct process restarts, including F2 before the first provider event and after transient partial deltas;
  2. capture before/after history and counters;
  3. run a competing-resumer case against one versioned snapshot;
  4. restart a second time;
  5. compare with the matrix expected action.
- Expected:
  - every state follows exactly its specified repair/intervention action;
  - no duplicate logical result;
  - no Never replay;
  - no partial stream becomes a durable assistant entry and no stale resumer launches external work;
  - second restart is effect/result/terminal-event idempotent.

## Terminal architecture review

- State: `active`
- Mode: `review`
- Evaluator Mode: `deterministic evidence + independent review`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### `architecture-and-scope`

- Covers: ADR-0003 ownership and exclusions.
- Procedure:
  1. inspect Cargo graph and source imports;
  2. inspect public APIs and durable records;
  3. trace T007–T012 to SPEC §4's observed evidence, adopted WorkWeave behavior, and explicit deviations;
  4. map SPEC requirement families to Verification/Evaluation evidence;
  5. run permanent gate on exact commit.
- Expected:
  - Agent/Flow state machines remain independent;
  - common runtime is semantically neutral;
  - no concrete transport/capability/product/Orchestration scope entered;
  - no blocking Stop Condition or unverified normative requirement remains.
