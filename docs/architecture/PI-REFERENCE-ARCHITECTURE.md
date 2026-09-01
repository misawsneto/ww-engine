# Pi Reference Architecture

## Status

- Reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- Claim discipline: distinguish **source-observed**, **derived**, and **proposed WorkWeave adaptation**.
- Pi's current production Agent and Pi's future Harness are separate subjects.

## Architectural thesis

Pi's useful architecture is not one monolithic CLI loop. It separates provider/model transport, a generic agent execution kernel, a coding-agent product/session layer, UI surfaces, and remote protocol/server components.

```text
provider/model APIs
       |
       v
+-------------+
|   pi-ai     |
+------+------+
       |
       v
+-------------+
| pi-agent    |  generic Agent + model/tool loop
+------+------+
       |
       v
+-------------+
| coding-agent|  sessions, resources, tools, extensions, trust, compaction
+------+------+
       |
  +----+----+----------------+
  |         |                |
 CLI/TUI    SDK          protocol/server
```

The implementation lesson is separation of concerns, not package-for-package transplantation.

## C1 — system context

Pi sits between a human/application and model providers plus local/external tools.

Actors and systems:

- user or SDK caller supplies intent and continuation messages;
- model provider emits streamed assistant content and tool calls;
- tools perform local or external effects;
- session persistence records conversational state;
- extensions can intercept resources, tools, provider requests, sessions, UI and events;
- server/protocol exposes durable sessions remotely.

## C2 — containers/packages

### `pi-ai`

Owns provider-neutral model, message, content, usage and stream protocols plus provider adapters.

### `pi-agent`

Owns `Agent`, the generic low-level agent loop, agent messages/events, tool execution coordination, steering/follow-up queues and cancellation.

### `coding-agent`

Owns the product assembly: `AgentSession`, coding tools, skills/resources, project trust, extensions, compaction, retry, model selection and interactive modes.

### TUI / protocol / client / server

Own presentation and remote session access without redefining the low-level loop semantics.

### Session backends

Pi also contains a newer durable session/backend direction, distinct from the current JSONL coding-agent session manager.

## C3 — core components

### Agent façade

`Agent` is stateful. It owns current model/context/tool configuration, queue state, event listeners and one active run. Important dependencies are injected, including `StreamFn`, optional context transform, tool hooks and dynamic API-key lookup.

Reference: `packages/agent/src/agent.ts#L98-L214`.

### Provider stream seam

`StreamFn(model, context, options) -> AssistantMessageEventStream` decouples the loop from concrete providers. Its contract says provider/request/runtime failures should be encoded in the stream/final assistant message instead of escaping as ordinary rejected promises.

Reference: `packages/agent/src/types.ts#L18-L32`.

WorkWeave should preserve this seam concept in Rust with a trait returning a typed event stream.

### Agent loop

Pi exposes prompt-start and continuation entry points over one internal `runLoop`. Conceptually:

```text
seed context
   |
   v
provider stream
   |
   v
assistant message
   |
   +-- no tool calls --> turn end / queue / terminal
   |
   +-- tool calls --> validate -> preflight -> execute -> finalize
                                      |
                                      v
                               tool-result messages
                                      |
                                      +----> next provider request
```

Reference: `packages/agent/src/agent-loop.ts#L32-L102` and `#L156-L360`.

### Tool execution

Pi distinguishes sequential and parallel batches. Before/after hooks allow policy/interception without placing that logic inside each tool. Coding-agent `ToolDefinition` owns model-visible metadata, schema, optional execution mode, execution function and presentation hooks.

References:

- `packages/agent/src/types.ts#L34-L95`.
- `packages/coding-agent/src/core/extensions/types.ts#L451-L500`.

### Queueing

Steering and follow-up messages are explicit queues. Queue behavior supports draining all or one at a time. This is agent-run control, not workflow orchestration.

Reference: `packages/agent/src/types.ts#L44-L50` and `packages/agent/src/agent.ts` queue implementation.

### AgentSession

`AgentSession` is the coding-agent composition root. It mediates the low-level Agent with session state, tools, extensions, resources, compaction, retries, model configuration and user-facing event translation.

Reference: `packages/coding-agent/src/core/agent-session.ts#L311+`.

WorkWeave should keep an equivalent composition layer thinner than Pi's historical product class by splitting policy, audit, persistence and resource concerns into explicit services.

### SessionManager

The current coding-agent session manager persists append-oriented JSONL session entries and maintains an in-memory tree/index. It delays initial file flushing until an assistant message exists and then appends subsequent entries.

Reference: `packages/coding-agent/src/core/session-manager.ts#L856-L1040`.

WorkWeave should borrow append-oriented durability and inspectability but use transactional storage where Agent/Flow execution state requires atomic multi-record changes.

### Server session seam

`PiSessionRuntime` exposes `snapshot`, phase, `prompt`, `steer`, `abort`, model/thinking changes, subscription and disposal. `PiServerService` lists/creates/opens durable sessions. Conflicting operations must reject rather than silently queue at that boundary.

Reference: `packages/server/src/types.ts#L41-L60`.

This is a strong reference for a WorkWeave server contract.

## C4 — code-level seams worth translating

### Model provider

```rust
#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn stream(
        &self,
        request: ModelRequest,
        ctx: ProviderContext,
    ) -> Result<ModelEventStream, ProviderError>;
}
```

### Tool

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn spec(&self) -> ToolSpec;

    async fn execute(
        &self,
        request: ToolRequest,
        ctx: ToolContext,
        cancel: CancellationToken,
    ) -> ToolResult;
}
```

### Agent loop

```rust
pub async fn run_agent(
    request: AgentRunRequest,
    services: &AgentServices,
) -> AgentRunResult;
```

The loop should remain conceptually small even if observability, budgeting, retries and policy produce many events around it.

## Agent domain model

```text
AgentDefinition
  - instructions/system policy
  - provider/model defaults
  - tool set
  - limits

AgentRun
  - run id
  - definition/config snapshot
  - input
  - lifecycle
  - usage
  - terminal result

AgentContext
  - ordered messages
  - model-facing derived context

ModelRequest / ModelResponse
  - provider/model
  - context snapshot/reference
  - streamed events
  - usage

ToolCall
  - call id
  - tool name
  - arguments

ToolExecution
  - policy decision
  - start/end
  - result/error

AgentEvent
  - ordered run-scoped occurrence
```

This is execution data. Goal/Task/Question/Evaluation semantics remain above it.

## State ownership

| State | Owner |
| --- | --- |
| current model, tools, messages, run queues | Agent runtime |
| provider protocol mapping | provider adapter |
| tool-specific execution | tool implementation |
| capability decision | policy layer |
| durable AgentRun audit | shared runtime/audit store |
| coding workspace | execution environment/tool layer |
| orchestration meaning | WorkWeave Orchestration, not Agent |

## Failure model

A production WorkWeave Agent should make terminal state explicit:

- `succeeded`;
- `failed`;
- `cancelled`;
- `timed_out`;
- `budget_exhausted`;
- `policy_denied`.

Provider failure, malformed tool input, tool execution failure and agent cancellation are different events even when the model ultimately sees a normalized error result.

## Pi future Harness

Pi contains a separate future architecture under `packages/agent/src/harness/`.

Source-observed concepts include:

- lanes;
- suspended operations;
- lane/session snapshots;
- queued items;
- action inspection;
- durable-looking operation records;
- a reducer capable of reconstructing lane execution state and detecting corrupt sequences.

References:

- `agent-harness.ts#L134-L198` — snapshots/actions;
- `agent-harness.ts#L305+` — façade;
- `reducer.ts#L79-L126` — lane state;
- `reducer.ts#L506+` — state reduction.

Important qualification: public Harness operations are incomplete in this reference revision. WorkWeave should borrow the durable-operation/reducer pattern only after distinguishing implemented contracts from scaffolded methods.

## What to preserve from Pi

- provider-neutral streaming seam;
- small low-level model/tool loop;
- typed tool schemas and pre/post execution interception;
- explicit steering/follow-up queue semantics;
- cancellation as first-class run control;
- event streaming to presentation layers;
- separation between generic Agent and coding-agent product assembly;
- project trust and capability-policy concepts;
- server/session abstraction;
- future Harness idea of reconstructable execution state.

## What to simplify or reject

- do not port all providers initially;
- do not reproduce a large all-owning AgentSession class;
- do not couple tool presentation to core execution traits;
- do not expose TypeScript dynamic-extension semantics as a Rust ABI prematurely;
- do not make JSONL conversation history the universal persistence model;
- do not embed WorkWeave semantic orchestration concepts in the agent loop.
