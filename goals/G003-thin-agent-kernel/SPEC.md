# Specification

## Title

- Durable provider-neutral WorkWeave Agent kernel

## State

- active

## Scope

G003 proves the probabilistic worker itself. It does not prove a production network adapter or user-facing Agent product surface. All model behavior is driven by deterministic recorded provider fixtures and all tool behavior by deterministic/synthetic fixtures.

## Exercised crates

### `ww-agent-provider`

Owns the stable provider protocol and deterministic provider conformance fixtures.

Core contracts:

```rust
pub trait ModelProvider: Send + Sync {
    fn id(&self) -> ProviderId;
    fn capabilities(&self, model: &ModelId) -> ModelCapabilities;
    async fn stream(
        &self,
        request: ModelRequest,
        context: ProviderContext,
    ) -> Result<ModelEventStream, ProviderError>;
}
```

Normalized stream vocabulary for G003:

```text
Started
TextDelta
ToolCallStarted
ToolCallArgumentsDelta
ToolCallCompleted
Usage
Completed
Failed
Aborted
```

The adapter protocol must not expose vendor request/response structures to `ww-agent-core`.

### `ww-agent-tools`

Owns tool identity, schema, replay semantics, policy input, and execution contract.

Minimum concepts:

```text
ToolId / ToolVersion
ToolSpec
ToolCallId
EffectDescriptor
ReplayPolicy::Safe | ReplayPolicy::Never
PolicyDecision::Allow | PolicyDecision::Deny
ToolAttemptId
ToolResult
```

G003 implementations are test fixtures only:

- `test.echo` — deterministic replay-safe structured result;
- `test.unsafe_once` — synthetic non-replayable fixture used only for crash/intervention proof.

The contract may describe future execution modes, but G003 launches calls sequentially only.

### `ww-agent-core`

Owns:

- `AgentRunRequest`;
- immutable provider-neutral context entries;
- operational attempt records;
- `AgentRecoveryState` reducer;
- the functional model → tool → model loop;
- limits and stop policy;
- Agent terminal result mapping;
- Agent persistence port.

It does not own transport, SQLite, filesystem tools, CLI, Flow, or WorkWeave Orchestration semantics.

### `ww-agent-store-sqlite`

Owns Agent-specific embedded persistence and backend coordination with the G002 SQLite store.

It may physically share `runtime.db`, but Agent tables and types remain Agent-owned. Any backend-specific transaction coordination must not add Agent DTOs or provider/tool concepts to `ww-store`.

## Context-bearing entries versus operational records

Borrow the useful distinction from Pi Harness without importing its unfinished public façade.

### Entries

Entries are immutable model-facing context artifacts:

```text
user_input
assistant_message
model_visible_tool_result
```

Each entry has stable identity, run identity, sequence/order metadata, immutable normalized payload, and provenance to the attempt/call that produced it.

### Operational records

Records describe how execution proceeded:

```text
model_attempt_started
model_attempt_interrupted
model_attempt_completed
tool_attempt_started
tool_attempt_denied
tool_attempt_completed
tool_attempt_intervention
turn_committed
agent_result_committed
```

Retries append new attempts. They never mutate prior attempts into success.

## Agent recovery state

`AgentRecoveryState` is a deterministic projection over the durable Agent run, entries, records, and current G002 execution record.

It must derive at least:

```text
current phase
ordered context entries
next model-request ordinal
current/last model attempt
pending logical tool calls
completed logical tool results
attempt history per logical tool call
usage summary
budget counters
cancel observation
terminal Agent result, if any
corruption/intervention state
```

The reducer must fail closed on:

- non-contiguous or duplicate run-local sequencing when sequencing is required;
- record references to unknown entries/calls/attempts;
- more than one finalized response for one model attempt;
- more than one committed model-visible result for one logical tool call;
- tool attempt start before a finalized assistant tool call exists;
- terminal Agent result followed by new execution records;
- impossible common-execution/Agent-terminal combinations that lack a defined repair path.

## Model stream assembly

The assembler is a pure state machine separate from provider transport.

```text
AwaitingStart
    ↓ Started
Streaming
    ├─ TextDelta*
    ├─ ToolCallStarted / ArgumentsDelta* / ToolCallCompleted
    ├─ Usage*
    └─ Completed | Failed | Aborted
```

Rules:

- no delta before `Started`;
- one terminal event only;
- tool argument fragments must form exactly one valid JSON value before `ToolCallCompleted` is accepted;
- a request ending before tool-call completion is a failed/incomplete model attempt, not an executable assistant message;
- a response stopped for provider length/truncation must not execute any possibly incomplete tool call;
- tool call IDs are unique within the response;
- normalized usage may be partial while streaming but is immutable when finalized;
- raw chain-of-thought is not a required or persisted type.

## Tool preparation and execution

For every finalized tool call, in provider source order:

```text
resolve pinned tool
→ parse complete JSON arguments
→ validate against JSON Schema
→ derive effect descriptor
→ obtain policy decision
→ persist logical call + attempt start/denial
→ execute only when allowed
→ persist finalized result/error
→ append exactly one model-visible result
```

A policy denial is a durable model-visible error result and performs no effect.

G003 is sequential. Later parallel scheduling may execute safe calls concurrently, but must preserve model-visible source order and the same durable logical-call identities.

## Kernel algorithm

```text
reconstruct AgentRecoveryState
repeat
    check common durable cancellation
    check deadline and budgets

    if an incomplete recoverable operation exists
        repair/resume according to recovery matrix
        continue

    commit model attempt start
    stream recorded provider events
    finalize and commit assistant entry + usage

    if model attempt failed/aborted
        settle according to stop/cancel policy
        break

    if no tool calls
        commit Agent terminal result
        terminalize common execution idempotently
        break

    for each tool call in source order
        validate + policy
        commit attempt boundary
        execute/deny
        commit result

    commit turn boundary
until terminal
```

The functional control loop receives ports for provider, tools, persistence, clock, cancellation, and limits. It does not open databases or construct concrete transports.

## Persistence boundaries

Durability is required before crossing an ambiguity boundary.

### Agent creation

Before any provider call:

- common `ExecutionRecord(kind = agent)` exists;
- Agent run configuration snapshot exists;
- initial input entry exists;
- creation audit exists.

Prefer one SQLite transaction. If the backend seam cannot support that without contaminating shared semantics, stop and design a bounded transaction coordinator or a fully specified idempotent repair protocol before proceeding.

### Model attempt

Before provider I/O:

- attempt identity, request ordinal, provider/model pin, normalized request digest, budget reservation, and start record are durable.

After provider finalization and before any requested tool executes:

- finalized assistant entry;
- normalized stop reason;
- complete tool calls;
- provider request ID when available;
- normalized usage;
- completion/failure record

are durable.

Transient deltas need not be durable.

### Tool attempt

Before effect execution:

- logical tool call identity;
- tool/version pin;
- arguments digest;
- replay policy;
- policy decision;
- attempt identity/start

are durable.

After effect completion:

- finalized result/error;
- result digest/artifact references;
- model-visible tool-result entry

are committed before the next model request.

### Terminal settlement

Commit the Agent terminal result before or atomically with common terminalization. Recovery must idempotently repair `Agent result committed + common execution still non-terminal` without re-contacting provider or tool.

## Recovery matrix

| Durable state at restart | Required action |
| --- | --- |
| run created, no model attempt | begin next model attempt |
| model attempt started, no finalized response | mark attempt interrupted; retry only if cancel/deadline/budget permit |
| finalized assistant with no tools | commit/repair Agent terminal result, then common terminal state |
| finalized assistant with tool calls, no tool attempt | prepare first pending tool |
| replay-safe tool attempt started, no result | append interrupted attempt; start new attempt for same logical call |
| non-replayable tool attempt started, no result | do not execute; commit intervention and settle common execution `RequiresIntervention` |
| tool result durable, model-visible result absent | idempotently append the one missing model-visible result |
| all tool results durable, turn not committed | commit turn/checkpoint |
| turn committed, no next model attempt | begin next model attempt |
| Agent terminal result durable, common execution non-terminal | idempotently terminalize common execution |
| impossible references/order | fail closed as corrupt history |

## Limits

`AgentLimits` in G003:

```text
deadline
max_model_requests
max_turns
max_tool_calls
optional_max_input_tokens
optional_max_output_tokens
optional_max_total_tokens
```

Rules:

- counters derive from durable attempts/results, not process-local mutable counters;
- check deadline/cancellation before each external/fixture operation;
- reserve request/tool budget before starting the attempt;
- reconcile provider usage when the provider supplies it;
- when token usage reaches a configured limit, stop before issuing the next model request;
- budget exhaustion is terminal and auditable, not a generic provider failure.

## Common runtime integration

G003 binds one Agent run to one G002 common execution.

The common runtime owns generic lifecycle, cancellation request, event cursor, artifacts, and execution identity. Agent owns messages, model/tool attempts, usage, recovery phase, and Agent result.

Agent must not push its DTOs into `ww-store` to obtain atomicity.

## Fault-injection points

Tests must be able to stop execution after each of these boundaries and restart from the same SQLite database:

1. common/Agent creation commit;
2. model-attempt start commit;
3. model finalization commit;
4. tool-attempt start commit;
5. tool-result commit before model-visible entry, when physically separable;
6. all tool results before turn commit;
7. Agent terminal result before common terminalization.

Each fault case must define the one valid recovery action and prove no duplicate committed logical result.

## Explicit exclusions

- concrete OpenAI/Anthropic/network adapter;
- filesystem read/write, shell, network, MCP, A2A tools;
- SDK/CLI/TUI/server Agent product surface;
- steering, follow-up, sessions, branching, compaction;
- parallel tool scheduling;
- provider routing/fallback;
- hidden chain-of-thought persistence;
- Flow/OWS and WorkWeave Orchestration semantics.
