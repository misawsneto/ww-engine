# Specification

## Title

- Thin WorkWeave Agent Kernel

## State

- draft

## Interfaces

### `ww-agent-provider`

- `ModelProvider` — streams normalized `ModelEvent` values for a normalized `ModelRequest`.
- `ProviderRegistry` — resolves `ProviderId` + `ModelId` and exposes capability metadata.
- `RecordedProvider` — deterministic fixture provider used by kernel/recovery tests.
- Normalized events must cover request start, text, tool-call construction/completion, usage, terminal completion, failure, and cancellation.

### `ww-agent-tools`

- `Tool` — identity, description, JSON input schema, effect class, replay policy, execution mode, and async execution.
- `ToolRegistry` — resolves exact tool identity/version exposed to a run.
- `ToolPolicy`/policy adapter — allow/deny decision over one exact requested effect.
- `fs.read` — bounded workspace read.
- deterministic structured test tool — pure known result for loop/conformance tests.
- synthetic non-replayable fixture — test-only effect used to prove intervention behavior.

### `ww-agent-core`

- `AgentRunRequest` — prompt/input context, provider/model, tool set, limits, configuration digest, and execution metadata.
- `AgentKernel` — functional bounded model/tool loop.
- `AgentRecoveryState` — state reconstructed from durable Agent records.
- `AgentPersistence` — transaction boundaries for input, model request/finalization, tool start/result, turn commit, and run result.
- `AgentLimits` — deadline, model-request, turn, tool-call, and optional token budgets.

### `ww-agent-store-sqlite`

- migrations for `agent_runs`, `agent_entries`, `agent_records`, model attempts, and tool attempts as needed by the chosen normalized model;
- reconstruction queries/reducer inputs;
- one physical embedded SQLite database may contain common and Agent tables, but Agent schema/types stay out of `ww-store`;
- commits spanning common execution state and Agent terminal/initial state must share one SQLite transaction through a backend coordination seam.

### `ww-agent-openai`

- first concrete `ModelProvider` adapter;
- vendor protocol converted at the adapter boundary only;
- recorded transport fixtures are mandatory; live smoke is optional and credential-gated.

### `ww-sdk`

- start bounded Agent run;
- inspect Agent state/transcript/audit summary;
- request cancellation through the common runtime seam;
- consume committed execution events.

### `ww-cli`

- `ww agent run`;
- `ww agent inspect`;
- optional machine-readable transcript/audit command if inspection output would otherwise become overloaded.

## Agent loop

```text
commit input
repeat
    check cancellation/deadline/budgets
    commit model-attempt start
    stream normalized provider events
    finalize + commit assistant response and usage

    if response failed/cancelled
        settle run
    else if no tool calls
        commit terminal Agent result
        terminalize common execution
        return
    else
        for each tool call in provider order
            validate schema
            obtain policy decision
            commit tool attempt start
            execute or deny
            commit finalized tool result
        append model-visible tool results in provider call order
        commit turn/checkpoint
until limit/terminal condition
```

## Recovery rules

- finalized assistant response + no tool attempt: continue from tool dispatch;
- replay-safe `tool_started` without result: start a new attempt for the same logical call and preserve prior attempt audit;
- non-replayable `tool_started` without result: do not execute; settle common/Agent run as `RequiresIntervention`;
- finalized tool results + no next model attempt: continue with the next model request;
- interrupted model attempt without finalized response: mark attempt interrupted and retry only if cancellation/deadline/budget policy permits;
- durable Agent result + non-terminal common execution: idempotently terminalize the common execution;
- impossible ordering/reference history: fail closed as corrupt history.

## Durable records

At minimum persist:

- Agent run configuration snapshot/digest;
- user input/context entry;
- model attempt started/interrupted/completed;
- finalized assistant message;
- normalized provider/model identity and provider request ID when available;
- normalized usage;
- tool call identity/name/arguments digest;
- policy decision;
- tool attempt start/result/error/intervention;
- model-visible tool result;
- turn/checkpoint settlement;
- terminal Agent result.

Transient text/tool-argument deltas may be streamed live and omitted from canonical persistence.

## Boundaries

- No Flow crates, OWS documents, scheduler, waits, branches, or tokens.
- No TUI/server/remote protocol.
- No general-purpose coding tools beyond `fs.read`.
- No public Agent session/branch/compaction model.
- No provider routing or fallback policy.
- No hidden chain-of-thought persistence requirement.
