# Proposed Rust Contracts

These contracts are architectural sketches, not frozen APIs.

## Workspace

```text
crates/
  ww-runtime/          shared execution identity, lifecycle, cancellation, events
  ww-audit/            durable ordered audit journal and inspection
  ww-store/            storage ports and transaction abstractions
  ww-store-sqlite/     embedded persistence
  ww-policy/           capability and execution policy
  ww-artifacts/        durable artifact references/content

  ww-agent-core/       Agent domain types + loop
  ww-agent-provider/   provider traits and normalized model protocol
  ww-agent-openai/     first concrete provider
  ww-agent-tools/      tool traits/registry and initial local tools

  ww-flow-core/        Flow runtime types and deterministic transition engine
  ww-flow-ows/         OWS parsing/profile validation/runtime plan
  ww-flow-scheduler/   waits, timers, leases and ready-work scheduling

  ww-sdk/              in-process public Rust API
  ww-server/           remote API/protocol
  ww-cli/              `ww` CLI
  ww-tui/              Agent and Flow terminal UI
```

Crate names can change. Dependency direction is more important:

```text
ww-agent-core -------> ww-runtime <------- ww-flow-core
      |                    ^                    |
      v                    |                    v
provider/tools          audit/store           OWS/scheduler
```

`ww-agent-core` must not depend on `ww-flow-core`, and `ww-flow-core` must not depend on Agent internals. Cross-engine execution goes through shared execution ports/adapters.

## Shared execution identity

```rust
pub struct ExecutionId(Uuid);
pub struct ParentExecutionId(pub ExecutionId);
pub struct CorrelationId(Uuid);

pub enum ExecutionKind {
    Agent,
    Flow,
    Job,
    Tool,
    External,
}

pub enum ExecutionState {
    Pending,
    Running,
    Waiting,
    Succeeded,
    Failed,
    Cancelled,
}
```

Agent and Flow may define stricter internal states; shared states exist only for operational inspection.

## Execution context

```rust
pub struct ExecutionContext {
    pub execution_id: ExecutionId,
    pub parent: Option<ExecutionId>,
    pub correlation_id: CorrelationId,
    pub deadline: Option<DateTime<Utc>>,
    pub budget: ResourceBudget,
    pub cancellation: CancellationToken,
    pub policy: PolicyContext,
}
```

## Thin executor port

```rust
#[async_trait]
pub trait Executor<Rq, Rs>: Send + Sync {
    async fn execute(
        &self,
        request: Rq,
        ctx: ExecutionContext,
    ) -> Result<ExecutionHandle<Rs>, ExecutionError>;
}
```

Do not add a universal `next()` or universal payload/state abstraction. Agent and Flow have different execution semantics.

## Durable event envelope

```rust
pub struct AuditEvent<T> {
    pub execution_id: ExecutionId,
    pub sequence: u64,
    pub occurred_at: DateTime<Utc>,
    pub kind: &'static str,
    pub payload: T,
}
```

Required properties:

- monotonic sequence per execution;
- stable event type;
- enough information for audit without storing hidden chain-of-thought;
- references to large payload/artifacts rather than forcing them inline;
- exportable trace correlation.

## Agent contracts

### Agent run request

```rust
pub struct AgentRunRequest {
    pub definition: AgentDefinitionRef,
    pub input: Vec<AgentMessage>,
    pub model: ModelRef,
    pub tools: Vec<ToolRef>,
    pub limits: AgentLimits,
    pub metadata: BTreeMap<String, Value>,
}
```

### Agent events

```rust
pub enum AgentEvent {
    RunStarted,
    ModelRequestStarted { attempt: u32 },
    ModelDelta(ModelDelta),
    ModelResponseCompleted(ModelResponse),
    ToolCallRequested(ToolCall),
    ToolPolicyEvaluated(PolicyDecision),
    ToolExecutionStarted(ToolExecutionId),
    ToolExecutionUpdated(ToolUpdate),
    ToolExecutionCompleted(ToolResult),
    TurnCompleted,
    RunCompleted(AgentRunResult),
}
```

### Provider

```rust
#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn stream(
        &self,
        request: ModelRequest,
        ctx: ProviderContext,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ModelEvent, ProviderError>> + Send>>, ProviderError>;
}
```

Provider adapters normalize protocol differences. The Agent loop consumes one model protocol.

### Tool

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn spec(&self) -> ToolSpec;
    fn execution_mode(&self) -> ToolExecutionMode { ToolExecutionMode::Sequential }

    async fn execute(
        &self,
        request: ToolRequest,
        ctx: ToolContext,
    ) -> ToolResult;
}
```

Policy wraps tool execution. Tools do not decide their own authorization.

## Flow contracts

### Workflow source

```rust
pub struct WorkflowRef {
    pub dsl_version: String,
    pub namespace: String,
    pub name: String,
    pub version: String,
    pub source_digest: Sha256Digest,
    pub profile: String,
}
```

This mirrors the WorkWeave v0.5 conceptual value without dictating its canonical persistence representation.

### Flow instance/token

```rust
pub struct FlowInstance {
    pub id: FlowInstanceId,
    pub workflow: WorkflowRef,
    pub state: FlowInstanceState,
    pub context: JsonValue,
}

pub struct FlowToken {
    pub id: FlowTokenId,
    pub instance: FlowInstanceId,
    pub position: WorkflowPosition,
    pub state: FlowTokenState,
    pub parent: Option<FlowTokenId>,
    pub lineage: Option<ExecutionLineage>,
    pub wait: Option<WaitState>,
}
```

### Deterministic transition

```rust
pub trait FlowInterpreter {
    fn step(
        &self,
        definition: &AcceptedWorkflow,
        snapshot: FlowSnapshot,
    ) -> Result<FlowDecision, FlowError>;
}
```

`FlowDecision` describes deterministic mutations and external work requirements; the transaction layer applies them atomically.

```rust
pub enum FlowDecision {
    Advance(TransitionSet),
    Spawn(Vec<TransitionSet>),
    Wait(WaitSpec),
    Invoke(ExternalExecutionSpec),
    Complete(FlowOutput),
    Fail(FlowFailure),
}
```

## External execution contract

```rust
pub enum ExternalExecutionTarget {
    Agent(A2aTarget),
    Mcp(McpTarget),
    Function(FunctionTarget),
    Workflow(WorkflowRef),
}

pub struct ExternalExecutionSpec {
    pub idempotency_key: String,
    pub target: ExternalExecutionTarget,
    pub input: JsonValue,
    pub retry: RetryPolicy,
    pub timeout: Option<Duration>,
}
```

The Flow transaction records a durable wait/external execution before dispatch. Completion correlates back to that exact identity.

## Local Agent adapter

```rust
pub struct LocalAgentA2aAdapter {
    agent: Arc<dyn Executor<AgentRunRequest, AgentRunResult>>,
}

impl A2aExecutor for LocalAgentA2aAdapter {
    // map A2A request -> AgentRunRequest
    // execute locally
    // map AgentRunResult -> A2A result
}
```

The same Flow code can later select a remote A2A transport without understanding Agent internals.

## Storage ports

Minimum operations:

```rust
#[async_trait]
pub trait RuntimeStore {
    async fn transaction<T>(&self, f: impl RuntimeTransaction<T>) -> Result<T, StoreError>;
}

pub trait RuntimeTransaction {
    fn append_audit(&mut self, event: NewAuditEvent) -> Result<AuditPosition, StoreError>;
    fn put_execution(&mut self, execution: &ExecutionRecord) -> Result<(), StoreError>;
    fn put_agent_run(&mut self, run: &AgentRunRecord) -> Result<(), StoreError>;
    fn put_flow_instance(&mut self, instance: &FlowInstanceRecord) -> Result<(), StoreError>;
    fn put_flow_token(&mut self, token: &FlowTokenRecord) -> Result<(), StoreError>;
}
```

Exact SQL/table layout is an implementation concern.

## Audit versus telemetry

Audit is durable and queryable:

```text
what ran
with what configuration
which model/provider was called
which tools/actions were requested
which policy allowed/denied them
which outputs/errors occurred
what artifacts changed
how the execution terminated
```

Telemetry adds operational signals:

- OpenTelemetry spans;
- metrics;
- sampled logs;
- exporter-specific traces.

Hidden chain-of-thought is not an audit requirement.

## SDK

```rust
let engine = WorkWeave::builder()
    .sqlite("ww.db")
    .provider(openai)
    .tool(fs_read)
    .build()
    .await?;

let run = engine.agent().run(request).await?;
let flow = engine.flow().start(workflow, input).await?;
```

## CLI

```text
ww agent run ...
ww agent inspect <run>
ww agent cancel <run>

ww flow validate workflow.yaml
ww flow run workflow.yaml
ww flow inspect <instance>
ww flow signal <instance> ...

ww run inspect <execution>
ww run events <execution>
```

## TUI

One application shell can expose two first-class execution views:

- Agent: transcript, model/tool activity, workspace effects, usage, cancellation, policy.
- Flow: current tokens, task positions, branches, waits, child executions, context, signals, retries.

Shared navigation can show parent/child executions and correlated audit events across both engines.
