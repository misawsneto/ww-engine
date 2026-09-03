# G003 Evaluations

## Agent protocol conformance
- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### Checks

#### stream-ordering
- Covers: `Provider-neutral stream protocol`
- Subjects: `ww-agent-provider normalized event types and stream assembler`
- Criteria: `Valid recorded streams finalize exactly once and illegal ordering, duplicate finalization, disconnect-before-finalization, and truncated/incomplete tool calls fail closed.`
- Procedure: `Run recorded provider stream fixtures and malformed-stream property/table tests against the pure assembler.`
- Expected: `All valid fixtures finalize to the expected normalized response; every invalid fixture returns a typed protocol/assembly failure and exposes no executable tool call.`

#### provider-boundary
- Covers: `Provider neutrality`
- Subjects: `ww-agent-provider and ww-agent-core public types/dependencies`
- Criteria: `Vendor transport/request/response types do not cross into ww-agent-core.`
- Procedure: `Compile contract fixtures and run dependency/source boundary checks.`
- Expected: `Only normalized WorkWeave provider types are visible to the kernel.`

## Agent durable recovery safety
- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### Checks

#### recovery-reduction
- Covers: `Restart reconstruction`
- Subjects: `Agent entries, operational records, SQLite persistence, AgentRecoveryState reducer`
- Criteria: `The same durable history reconstructs the same AgentRecoveryState after reopen/process restart, while impossible history fails closed.`
- Procedure: `Persist canonical histories, reopen in new processes, compare projections, and run corruption fixtures for invalid references/order/finalization.`
- Expected: `Valid projections are identical; corrupt histories return typed corruption failures.`

#### replay-safety
- Covers: `Tool effect recovery`
- Subjects: `ReplayPolicy, tool attempts, model-visible tool-result entries`
- Criteria: `Replay-safe ambiguity creates a distinct retry attempt with one logical result; non-replayable ambiguity never executes again and requires intervention.`
- Procedure: `Fault after tool-attempt start for safe and non-replayable fixtures, restart, and inspect attempts/effects/results.`
- Expected: `No logical call has more than one committed model-visible result; non-replayable fixture effect count does not increase after restart.`

#### settlement-repair
- Covers: `Agent/common terminal consistency`
- Subjects: `Agent terminal result and G002 ExecutionRecord`
- Criteria: `A durable Agent terminal result with non-terminal common execution is repaired idempotently without provider/tool replay.`
- Procedure: `Fault between Agent result commit and common terminalization, restart twice, and inspect state/event history.`
- Expected: `Common terminal state is applied once and no model/tool attempt is duplicated.`

## Agent kernel execution conformance
- State: `active`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G003 — Durable Agent Kernel`

### Checks

#### model-tool-model
- Covers: `Probabilistic worker loop under recorded inputs`
- Subjects: `ww-agent-core kernel, RecordedProvider, deterministic tool fixture`
- Criteria: `One recorded provider run requests one tool, receives one ordered model-visible result, and produces the expected terminal Agent result.`
- Procedure: `Execute the real kernel against pinned recorded provider/tool fixtures.`
- Expected: `One logical tool call/result exists; final context and result match the fixture contract.`

#### cancellation-limits
- Covers: `Bounded execution`
- Subjects: `G002 cancellation seam and AgentLimits`
- Criteria: `Cancellation, deadline, model-request, turn, tool-call, and normalized token limits terminate at defined boundaries and remain auditable.`
- Procedure: `Run deterministic fixtures that block provider/tool work or exceed each configured limit.`
- Expected: `Each case reaches its defined terminal disposition without launching an operation beyond the limit/cancel boundary.`
