# Flow Reference Architecture

## Sources

- WorkWeave Orchestration v0.5 at `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS schema/profile source at `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph at `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.

## WorkWeave Flow boundary

The canonical v0.5 model already establishes the essential separation:

```text
OWS owns
  workflow syntax
  task/control-flow semantics
  expressions and data movement
  waits/events/concurrency/retries

WorkWeave Flow owns
  FlowInstance
  FlowToken
  exact WorkflowRef pin
  persisted workflow context
  wait correlation
  execution lineage
  meaningful movement

Architecture owns
  parsing/normalization
  scheduler/workers
  locks/transactions
  timers/signals
  external execution adapters
  databases/indexes

Observability owns
  low-level model/tool calls
  traces/logs/metrics/usage
```

The engine repository implements the Architecture side without replacing the canonical Flow model.

## Canonical Flow runtime model

### FlowInstance

One durable resumable execution of one exact accepted OWS workflow definition.

Important invariant: the workflow reference is immutable after start.

### FlowToken

One durable resumable position inside a FlowInstance. Concurrent branches or iterations retain independent identity and lineage.

### WorkflowRef

Pins OWS DSL version, namespace/name/version, source digest and WorkWeave profile. A running instance never silently adopts changed source.

### WorkflowPosition

Unambiguous logical location in the pinned definition. The Rust architecture owns the physical locator representation.

### WorkflowContextState

Durable procedural OWS context for deterministic resume. It has no WorkWeave Domain authority.

### WaitState

Durable event/time/nested-workflow/external-execution correlation state. A wait resumes only from a matching cause.

## OWS integration surface

The frozen WorkWeave profile currently maps:

| Purpose | OWS mechanism |
| --- | --- |
| Agent | native `call:a2a` |
| Tool | native `call:mcp` |
| Service/domain call | `call:function` |
| Nested workflow | `run.workflow` |
| Event wait | `listen` |
| Procedural state | input/output/export/set |

This is important for WorkWeave Engine: a local Agent implementation is an A2A executor adapter, not a proprietary Flow node kind.

## LangGraph runtime lessons

LangGraph is useful as a runtime reference but not as definition authority.

### Pregel plan/execute/update

At the pinned revision, `Pregel` documents three superstep phases:

1. **Plan** which actors execute based on channel triggers;
2. **Execution** of selected actors, with writes not visible until the next step;
3. **Update** channel values from completed writes.

Reference: `libs/langgraph/langgraph/pregel/main.py` around the `Pregel` class at the pinned revision.

WorkWeave should borrow the clarity of explicit deterministic stepping, but OWS task/control-flow semantics determine what a Flow step means.

### Checkpoints

LangGraph's `BaseCheckpointSaver` persists graph state keyed by a `thread_id`; checkpoints enable persistence, resume from interrupts and time-travel/debugging. The saver contract exposes get/list/put/write operations and both synchronous and asynchronous implementations.

Reference: `libs/checkpoint/langgraph/checkpoint/base/__init__.py` at the pinned revision.

WorkWeave adaptation:

- checkpoint exact FlowInstance/FlowToken/context state at transaction boundaries;
- use stable run/instance identity rather than conflating workflow identity and conversational thread identity;
- retain ordered audit events separately from checkpoint snapshots;
- make replay/recovery deterministic against the pinned OWS definition.

### Interrupts and commands

LangGraph exposes explicit interrupt/resume and command constructs. The useful idea is first-class suspension and external resumption, not adopting its graph vocabulary.

### Streaming

LangGraph supports multiple stream modes and scoped subgraph streaming. WorkWeave should expose a typed engine event stream for Flow progression and a distinct Agent event stream, then provide a common envelope at the SDK/server boundary.

### Subgraphs

Nested execution can retain namespace/scope. WorkWeave already has native `run.workflow`; nested Flow execution should preserve parent/child identity and cancellation semantics without inventing a second subgraph model.

## Deterministic Flow stepping

A useful conceptual runtime is:

```text
load pinned definition + instance + ready tokens
                |
                v
        determine next task(s)
                |
                v
       evaluate deterministic input
                |
        +-------+---------+
        |                 |
        v                 v
   internal step     external execution
 set/switch/for      function/MCP/A2A/run
        |                 |
        v                 v
 persist movement     durable wait/result
        |                 |
        +-------+---------+
                |
                v
        next token state
```

The worker may execute multiple independent tokens concurrently, but each transition must remain deterministic given the pinned definition, current durable context and completed external results.

## Flow-to-Agent boundary

```text
OWS call:a2a
    |
    v
A2A Call Resolver
    |
    +--> LocalWorkWeaveAgentAdapter
    |       |
    |       v
    |    AgentExecutor
    |
    +--> RemoteA2AAdapter
            |
            v
        remote agent
```

The Flow kernel knows only the A2A call contract and durable external-execution identity. It does not manipulate Agent turns, messages or tool calls.

## What to borrow from LangGraph

- explicit deterministic execution loop;
- durable checkpoints and resume;
- first-class interrupts;
- streaming execution state;
- retry/cancellation boundaries;
- scoped nested execution;
- separation between graph construction and execution engine.

## What not to borrow

- another canonical graph definition language;
- framework-specific node semantics where OWS already defines behavior;
- state channels as WorkWeave Domain truth;
- Agent-specific workflow node types;
- conflation of checkpoint state with audit history.
