# Specification — G003 Durable Agent Kernel

- Version: `v2`
- Approval: `pending requester approval`
- Refinement: `D021`
- Governing architecture: `ADR-0003`
- Supersedes: the unversioned G003 specification for remaining open work when approved
- Implementation code basis: `facb2d04d0060417db7a9fa50de68221ea493f33`
- Completed-work boundary: T001–T006 remain governed by their original accepted contracts and evidence

## 0. Document contract

This specification makes the remaining G003 work executable without changing the accepted Goal boundary, architecture, completed Task meanings, or Task identifiers.

Normative words have their usual meaning:

- **MUST / MUST NOT** — required for G003 acceptance.
- **SHOULD / SHOULD NOT** — strong default; deviation requires a recorded rationale in the Task review.
- **MAY** — permitted but not required.

Authority remains:

```text
accepted Decisions and ADR-0003
        ↓
G003 GOAL.md
        ↓
this SPEC
        ↓
PLAN
        ↓
TASKS
        ↓
VERIFICATION / EVALUATIONS
        ↓
implementation
```

This version is prospective for T007–T012. It does not retroactively add requirements to T001–T006.

### 0.1 Refinement method

D021 authorized this precision pass under `ww-refine-goal`.

The refinement method adapts, rather than copies, these pinned sources:

| Source | Pin | Adopted use |
| --- | --- | --- |
| Addy Osmani `spec-driven-development` | `addyosmani/agent-skills@1c760d643497e9da289300e5eb2f5aca861503f7`, file blob `f3f5877c5d6be8f74408c308393bfb45cbcf53c4` | explicit assumptions, boundaries, testable success criteria, human approval before implementation |
| Addy Osmani `planning-and-task-breakdown` | same repository pin, file blob `296249b64334bcfd1aeaefd27b9e3e5494e38ec0` | dependency order, focused work units, acceptance + verification + likely files, checkpoints |
| WorkWeave `ww-refine-goal` | D020 | stable Task identities, completed-work immutability, Goal-scoped `REPLAN_LOCK` |

The existing G003 Task topology already satisfies the greenfield capability-map step. This refinement MUST NOT decompose or renumber G003 again.

### 0.2 Architecture evidence labels

- **Pi observed** — behavior present in pinned Pi production Agent code.
- **Pi Harness observed** — behavior present in pinned future Harness code; useful evidence, not production equivalence.
- **WW adopted** — normative WorkWeave G003 behavior.
- **WW deferred** — useful behavior deliberately outside G003.

## 1. Objective

G003 proves that a bounded probabilistic Agent can:

1. reconstruct its next safe action from durable history;
2. issue provider-neutral model requests through `RecordedProvider`;
3. validate and authorize deterministic/synthetic tool calls;
4. commit every ambiguity-sensitive boundary before crossing it;
5. recover after process restart without duplicating one logical tool result;
6. propagate cancellation and enforce durable limits;
7. terminate with one auditable Agent result consistent with one G002 execution.

Success is one deterministic text-only run and one deterministic:

```text
model → tool → model → terminal Agent result
```

executed through the real kernel, persisted in SQLite, reopened across process boundaries, and verified by the required Evaluations.

## 2. Frozen and open scope

### 2.1 Frozen foundation

T001–T006 are complete and remain unchanged in meaning:

- ADR-0003 accepted and G003 active;
- normalized provider protocol and fail-closed stream assembler;
- immutable Agent context entries and operational records;
- pure recovery reducer;
- Agent-owned SQLite persistence;
- atomic common-execution + Agent-run creation/linkage;
- deterministic `RecordedProvider` conformance fixtures.

Refinement or implementation MUST NOT claim that completed evidence proved new T007–T012 requirements.

### 2.2 Open implementation scope

| Task | Capability |
| --- | --- |
| T007 | tool identity, schema validation, effect/replay classification, policy, deterministic fixtures, and durable preparation metadata |
| T008 | smallest functional recorded-provider model/tool loop |
| T009 | G002 lifecycle binding, cancellation propagation, and terminal repair |
| T010 | durable deadlines and execution budgets |
| T011 | process-restart and ambiguous-effect recovery matrix |
| T012 | exact-code EvaluationRuns and terminal architecture/recovery review |

### 2.3 Explicit exclusions

G003 MUST NOT add:

- OpenAI, Anthropic, or another concrete network provider;
- API keys, credential resolution, HTTP clients, or retries against a live provider;
- public filesystem, process, network, MCP, plugin, A2A, SDK, CLI, TUI, or server capability;
- Flow, OWS, Goal, Task, Decision, Evaluation, Review, epistemic, deontic, or temporal orchestration semantics;
- parallel tool execution;
- sessions, steering/follow-up queues, compaction, branches, subagents, or dynamic tool installation;
- generic schema/payload migration infrastructure or reusable storage-hardening work assigned to proposed G010.

## 3. Assumptions settled by this specification

1. G003 runs in the embedded, single-process profile with SQLite persistence and deterministic fixtures.
2. `main` is the canonical engineering line. A stale G003 branch is never an alternate authority.
3. Model tool arguments reach T007 as a complete parsed `serde_json::Value` produced by the T002 assembler.
4. The parsed value is the sole executable authority. Existing raw JSON may remain as provider provenance but MUST NOT be reparsed for validation, policy, digesting, or execution.
5. Tool argument validation is non-coercing: validation either accepts the exact parsed value or rejects it.
6. Tool schemas use JSON Schema Draft 2020-12 and are self-contained.
7. G003 policy has only `Allow` and `Deny`. Approval workflows remain later.
8. G003 replay policy has only `Safe` and `Never`. Idempotency-key semantics remain later.
9. Every tool call is handled sequentially in provider source order.
10. Provider-declared failure does not trigger an automatic transient-retry policy in G003. Crash recovery may create a new audited attempt when the recovery matrix permits.
11. A text response finalized with `CompletionReason::Length` is durable for audit but does not count as a successful Agent result.
12. G003 has no released durable-data compatibility promise. T007–T011 may add Agent record variants and update fixtures needed by the accepted proof; same-code reopen/restart remains mandatory, while known-old schema/payload migration belongs to proposed G010.
13. No open architectural question blocks this candidate. Requester approval of the complete packet settles these refinements.

## 4. Reference-derived architecture

### 4.1 Pi production Agent: adopted seams

At the pinned Pi revision, the production loop separates provider streaming from the Agent loop, resolves a tool by name, prepares/validates arguments, runs a preflight policy hook, executes allowed tools, normalizes failures into model-visible results, and preserves source ordering for sequential calls.

Exact evidence:

- `packages/agent/src/types.ts#L18-L42` defines the injected provider stream seam and the sequential/parallel ordering contract;
- `packages/agent/src/types.ts#L258-L293` places the pre-tool hook after validation and before execution;
- `packages/agent/src/agent-loop.ts#L156-L273` keeps the low-level model/tool loop separate from the stateful product façade;
- `packages/agent/src/agent-loop.ts#L409-L552` prepares calls before execution and preserves assistant source order in emitted tool-result messages;
- `packages/agent/src/agent-loop.ts#L598-L788` resolves, validates, preflights, executes, normalizes failures, and constructs one model-visible result.

All paths are relative to the immutable Pi prefix in `docs/architecture/SOURCE-REGISTER.md`.

**WW adopted:**

```text
resolve
→ validate exact arguments
→ derive effect/replay
→ evaluate centralized policy
→ persist pre-effect state
→ execute or deny
→ normalize one model-visible result
```

### 4.2 Pi production Agent: deliberate adaptations

| Pi behavior | G003 treatment |
| --- | --- |
| argument preparation may transform/coerce raw arguments | reject; G003 validation is non-coercing |
| tools may run in parallel | reject; sequential only |
| hooks can patch tool calls/results | defer |
| session queues and continuation modes | defer |
| tool updates/progress | not required for G003 |
| coding-agent filesystem/process tools | defer to later Goals |

### 4.3 Pi Harness: adopted durability evidence

At the pinned Harness revision:

- entries and operational records are separate;
- a tool-start record includes effective arguments, source position, replay classification, and a reserved result-entry identity;
- the reducer derives unresolved tool batches in source order;
- open operations are reconstructed and impossible histories fail closed.

**WW adopted:**

- stable logical call, attempt, and reserved result identities;
- durable effective-argument digest, tool pin, effect, policy, and replay classification before execution;
- append-only attempts;
- pure recovery reduction;
- one model-visible result per logical call.

**WW deferred:** Harness lanes, queues, navigation, compaction, hook system, filesystem environment, and session product façade.

Exact evidence:

- `packages/agent/src/harness/reducer.ts#L131-L270` validates attempt continuity, reserved result identity, assistant/source-index linkage, and duplicate tool invocation;
- `packages/agent/src/harness/reducer.ts#L311-L382` validates the record log before reduction and rejects unknown or post-finish records;
- `packages/agent/src/harness/reducer.ts#L447-L507` reconstructs the unresolved source-ordered tool batch from entries plus records;
- `packages/agent/src/harness/agent-harness.ts` still contains unavailable/not-implemented façade operations at the pin, so G003 adopts reducer evidence rather than claiming Harness lifecycle parity.

### 4.4 WorkWeave architecture dossier: governing direction

The primary Engine dossier establishes:

- stable tool identity/version;
- tool-owned schema and async execution contract;
- centralized policy over an effect descriptor;
- replay classification before effect execution;
- durable model and tool boundaries;
- no silent replay of an unsafe started effect;
- `jsonschema` for dynamic provider/tool-boundary validation.

This specification narrows those general contracts to the G003 proof.

### 4.5 Reference-to-Task design lineage

| Task | Observed reference evidence | WorkWeave adoption | Deliberate rejection or adaptation |
| --- | --- | --- | --- |
| T007 | Pi validates before `beforeToolCall`, converts blocked/invalid calls to tool results, and preserves source order; Harness binds tool start to assistant entry, source index, and reserved result identity | provider-independent registry; exact non-coercing validation; centralized policy; stable source/call/result identities; pure corruption-checking reducer | no `prepareArguments` coercion, postflight mutation, parallel execution, product hooks, lanes, or Harness façade |
| T008 | Pi injects `StreamFn` into a small `runLoop` and alternates finalized assistant messages with ordered tool results | injected `ModelProvider`; one production EOF/finalization path; small functional durable driver | no stateful mega-session, queue draining, follow-ups, compaction, or concrete transport |
| T009 | Pi propagates one abort signal through provider and tool hooks; G002 already separates durable cancel request from live token delivery | one common execution root token, child tokens, durable intent before signal, explicit terminal repair | cancellation is not proof that a started Never effect did not occur; no Pi session lifecycle is imported |
| T010 | the WorkWeave dossier requires reserve-before-work budget accounting; Pi's turn-stop callback shows a useful loop boundary but is process-local | durable counters and pure pre-launch decisions reconstructed from Agent history | no callback-local counters, pricing policy, provider retry budget, or general resource scheduler |
| T011 | Harness reconstructs open work and rejects contradictions; LangGraph checkpoints retain pending writes and its interrupt contract may re-execute a node from its start | explicit F1–F8 durable states, bounded repair actions, distinct-process restart, second-restart idempotency | no generic graph/checkpoint runtime and no assumption that replaying an interrupted unit is safe for a Never effect |
| T012 | Addy Osmani's spec/planning skills require explicit assumptions, dependency order, acceptance, and verification; the dossier requires behavioral reference-parity tests rather than implementation copying | exact-code EvaluationRuns and requirement-to-evidence review | reference projects are evidence, not compatibility targets or architecture authority |

LangGraph and OWS remain negative boundary evidence for G003: LangGraph's checkpoint mechanics inform restart discipline, while OWS remains the future Flow-definition authority. Neither contributes a graph, command, interrupt, task, or workflow type to the Agent kernel.

## 5. Container and dependency architecture

```text
                         ┌───────────────────────┐
                         │ RecordedProvider      │
                         │ ww-agent-provider     │
                         └───────────┬───────────┘
                                     │ normalized stream
                                     ▼
┌───────────────────────┐   ┌────────────────────────┐
│ ToolRegistry/Policy   │◄──│ AgentKernel            │
│ ww-agent-tools        │   │ ww-agent-core          │
└───────────┬───────────┘   └───────────┬────────────┘
            │ fixture effect             │ AgentStore port
            ▼                            ▼
┌───────────────────────┐   ┌────────────────────────┐
│ test.echo /           │   │ SqliteAgentStore +     │
│ test.unsafe_once      │   │ coordinator            │
└───────────────────────┘   │ ww-agent-store-sqlite  │
                            └───────────┬────────────┘
                                        │ common lifecycle
                                        ▼
                            ┌────────────────────────┐
                            │ G002 runtime/store     │
                            └────────────────────────┘
```

### 5.1 Required dependency direction

- `ww-agent-provider` MUST NOT depend on Agent core, tools, runtime, persistence, Flow, or transport.
- `ww-agent-tools` MAY depend on utility crates such as `serde`, `serde_json`, `sha2`, `async-trait`, `tokio-util`, and `jsonschema`. It MUST NOT depend on Agent core, runtime/store, SQLite, filesystem/process/network libraries, Flow, or Orchestration.
- `ww-agent-core` MAY depend on `ww-agent-provider`, `ww-agent-tools`, and the generic G002 runtime API. It MUST NOT depend on SQLite, concrete provider transport, filesystem/process/network capability, Flow, CLI, or Orchestration.
- `ww-agent-store-sqlite` implements Agent persistence and the bounded common/Agent SQLite coordination seam. Agent DTOs MUST NOT enter `ww-store`.
- G003 MUST add no generic `ww-policy`, `ww-agent-openai`, or `ww-agent-tools-local` crate.

Agent operational identities (`AgentRunId`, `LogicalToolCallId`, `ToolAttemptId`, and `AgentEntryId`) remain owned by `ww-agent-core`, as established by T003. The `ww-agent-tools` public API MUST NOT mention those types or depend on core to obtain them. Core correlates a generic tool invocation with its durable Agent attempt outside the tool trait.

## 6. Tool subsystem contract — T007

### 6.1 Required concepts

The implementation MUST expose WorkWeave-owned equivalents of:

```rust
pub struct ToolId(String);          // model-visible stable name, e.g. "test.echo"
pub struct ToolVersion(String);     // opaque non-empty version, fixture value "1"

pub struct ToolIdentity {
    pub id: ToolId,
    pub version: ToolVersion,
    pub implementation_digest: Option<String>,
}

pub struct ToolSpec {
    pub identity: ToolIdentity,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub enum ReplayPolicy {
    Safe,
    Never,
}

pub enum EffectDescriptor {
    Pure { kind: String },
    Synthetic { kind: String, attributes: serde_json::Value },
}

pub enum PolicyDecision {
    Allow,
    Deny { code: String, message: String },
}

pub struct ToolOutput {
    pub content: serde_json::Value,
}

pub struct ToolExecutionError {
    pub code: String,
    pub message: String,
}

pub struct ToolRequest {
    pub identity: ToolIdentity,
    pub arguments: serde_json::Value,
}

pub struct ToolContext {
    pub cancellation: tokio_util::sync::CancellationToken,
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn spec(&self) -> ToolSpec;
    fn effect(&self, arguments: &serde_json::Value)
        -> Result<EffectDescriptor, ToolExecutionError>;
    fn replay_policy(&self, arguments: &serde_json::Value) -> ReplayPolicy;
    async fn execute(
        &self,
        request: ToolRequest,
        context: ToolContext,
    ) -> Result<ToolOutput, ToolExecutionError>;
}
```

Equivalent names or module placement are allowed only when the ownership, data, and ordering semantics remain identical.

`ToolRequest` deliberately contains no Agent run/call/attempt/entry identity. Those values belong to core's durable execution protocol and are not required to perform a G003 fixture effect. Later observability or idempotency work may introduce a tools-owned opaque invocation token through separate governance; T007 MUST NOT create a dependency cycle to expose core IDs.

### 6.2 Identity rules

- `ToolId` MUST be non-empty and MUST equal the name exposed to the model.
- G003 fixture IDs are exactly `test.echo` and `test.unsafe_once`.
- `ToolVersion` MUST be non-empty; both initial fixtures use version `1`.
- One registry MUST reject duplicate `ToolId` values.
- A run MUST resolve the ordered tool identities pinned in its configuration; it MUST NOT silently substitute another version with the same name.
- `implementation_digest` MAY be absent for G003 fixtures, but the field belongs to the identity contract for later pinning.

### 6.3 Registry rules

`ToolRegistry` MUST:

1. accept an explicit set of tools;
2. validate and compile each schema at construction/registration;
3. reject duplicate IDs before a run starts;
4. expose deterministic lookup by exact `ToolId`;
5. emit model-visible specs in the order pinned by the run configuration;
6. retain compiled validators for reuse;
7. return WorkWeave-owned errors rather than `jsonschema` types.

The registry MUST be immutable for one `AgentRun`.

### 6.4 JSON Schema profile

T007 MUST add:

```toml
jsonschema = { version = "0.52.1", default-features = false }
```

to the workspace dependency set and commit the resulting lockfile.

Selection basis: the official `jsonschema` 0.52.1 documentation reports an MSRV of Rust 1.85 and enables HTTP/file reference resolution by default. WorkWeave pins Rust 1.98 and therefore disables all default resolver features for this offline G003 profile.

Normative profile:

- Draft: JSON Schema 2020-12, selected explicitly.
- Schema root: object schema for G003 tools.
- External `$ref`: forbidden.
- Local fragment references such as `#/$defs/...`: allowed.
- HTTP and file retrieval features: disabled.
- Schema document validity: checked before registration succeeds.
- Instance validation: performed with one compiled reusable validator.
- Coercion/default injection: forbidden.
- `format` assertion: disabled in G003.
- Error ownership: `ww-agent-tools` converts validation failures into stable records containing at least an instance path and message.
- Error ordering: deterministic, sorted by instance path and then message.
- `schemars`: not added in G003 because schemas are explicit fixture data.

A malformed or externally resolving schema is a tool-definition error and prevents registry construction. Invalid invocation arguments are a call error and do not invoke policy or the tool.

### 6.5 Canonical arguments and digest

- `ToolCall.arguments: Value` is authoritative.
- `arguments_json` is diagnostic provenance only.
- Validation, effect derivation, policy, digesting, and execution MUST receive the same parsed value.
- No stage may mutate that value.
- `arguments_digest` MUST be SHA-256 over a compact deterministic serialization of the parsed value with object keys recursively sorted.
- Two objects differing only in insertion/key order MUST produce the same digest.
- A semantically different value MUST produce a different digest.
- This digest is an internal G003 audit/recovery identity, not yet a cross-language public canonicalization standard.

### 6.6 Effect and replay classification

Classification occurs only after schema validation.

- `test.echo` returns `EffectDescriptor::Pure { kind: "test.echo" }` and `ReplayPolicy::Safe`.
- `test.unsafe_once` returns `EffectDescriptor::Synthetic { kind: "test.unsafe_once", ... }` and `ReplayPolicy::Never`.
- A tool cannot authorize itself. Replay classification is tool metadata; policy remains a separate decision.
- G003 MUST NOT include filesystem, process, network, MCP, or secret-bearing effect variants in exercised code.
- Classification failure becomes a model-visible no-effect error.

### 6.7 Policy contract

The G003 policy seam is deterministic and synchronous:

```rust
pub trait ToolPolicy: Send + Sync {
    fn evaluate(&self, input: &ToolPolicyInput) -> PolicyDecision;
}
```

`ToolPolicyInput` contains the tool identity, the immutable validated arguments, their canonical digest, the effect descriptor, and the replay policy. It contains no WorkWeave Goal/Task semantics.

Rules:

- policy executes only after successful validation and effect/replay classification;
- `Allow` permits preparation for execution;
- `Deny` performs no effect;
- denial produces one model-visible error result:
  `{"code":"policy_denied","message":<reason>}`;
- policy is called exactly once per preparation attempt;
- `RequireApproval` is outside G003.

### 6.8 Fixture behavior

`test.echo`:

- schema requires exactly one property named `value`;
- additional properties are rejected;
- returns `{"value": <input value>}`;
- has no external side effect;
- returns byte-equivalent normalized output for the same input.

`test.unsafe_once`:

- schema requires a non-empty string property named `key`;
- additional properties are rejected;
- invokes an injected test-only effect probe;
- returns `{"applied":true,"key":<key>}` when the effect and result both complete;
- is always `ReplayPolicy::Never`;
- supports a test failpoint after the probe observes the effect and before the Agent commits the result;
- exposes no public filesystem/process/network capability.

Unit tests MAY use an in-memory probe. T011 process-restart tests MAY use a test-only durable probe outside the public tool surface to prove that restart does not repeat the effect.

## 7. Durable tool preparation and result model

### 7.1 Required identities

For each provider tool call, the Agent allocates once and persists:

- `LogicalToolCallId` — stable across retries;
- `ToolAttemptId` — unique per handling/execution attempt;
- reserved `AgentEntryId` for the eventual model-visible result;
- assistant entry ID and zero-based provider source index;
- provider call ID and requested tool name.

The logical ID is generated once when the finalized assistant entry is created. Recovery reads it from that entry; it is never regenerated from provider output.

### 7.2 Required durable information and records

Before any allowed effect starts, durable history MUST contain:

- logical call and attempt IDs;
- source assistant entry and source index;
- provider call ID and requested tool name;
- exact tool identity/version;
- reserved result-entry ID;
- canonical arguments digest;
- effect descriptor;
- replay policy;
- policy decision;
- an explicit effect-start marker.

T007 MUST add these Agent-owned record shapes, with equivalent Rust field types already owned by `ww-agent-core` or `ww-agent-tools`:

```rust
pub enum ToolPreparationStage {
    Resolve,
    Validate,
    Classify,
    Policy,
}

pub enum ToolPreparationDisposition {
    Executable {
        identity: ToolIdentity,
        arguments_digest: String,
        effect: EffectDescriptor,
        replay: ReplayPolicy,
        policy: PolicyDecision, // Allow only in this variant
    },
    NoEffect {
        failed_at: ToolPreparationStage,
        code: String,
        message: String,
        identity: Option<ToolIdentity>,
        arguments_digest: Option<String>,
        effect: Option<EffectDescriptor>,
        replay: Option<ReplayPolicy>,
        policy: Option<PolicyDecision>,
    },
}

pub enum ToolEffectResult {
    Output { content: serde_json::Value },
    Error { code: String, message: String },
}

pub enum AgentRecordData {
    // existing variants remain
    ToolCallPrepared {
        attempt_id: ToolAttemptId,
        logical_call_id: LogicalToolCallId,
        assistant_entry_id: AgentEntryId,
        source_index: u32,
        provider_call_id: ToolCallId,
        requested_tool_name: String,
        result_entry_id: AgentEntryId,
        disposition: ToolPreparationDisposition,
    },
    ToolEffectStarted { attempt_id: ToolAttemptId },
    ToolEffectCompleted {
        attempt_id: ToolAttemptId,
        result: ToolEffectResult,
    },
    ToolAttemptRejected {
        attempt_id: ToolAttemptId,
        result_entry_id: AgentEntryId,
        failed_at: ToolPreparationStage,
    },
    ToolAttemptInterrupted {
        attempt_id: ToolAttemptId,
        reason: String,
    },
}
```

`ToolAttemptStarted` retains its existing meaning as the beginning of one durable handling attempt. It is **not**, by itself, evidence that the effect boundary was crossed. `ToolEffectStarted` is the ambiguity marker used by cancellation and restart recovery.

The exact enum/module layout may follow existing Rust conventions, but the variants, fields, and distinction between handling start and effect start are normative for G003.

### 7.3 No-effect settlements

Unknown tool, invalid arguments, classification failure, and policy denial:

- MUST invoke no effect;
- MUST allocate one handling attempt and use the already-reserved result entry;
- MUST atomically append, in this order: `ToolAttemptStarted`, `ToolCallPrepared::NoEffect`, the model-visible error entry, and one final no-effect attempt record;
- MUST NOT append `ToolEffectStarted` or `ToolEffectCompleted`;
- MUST append exactly one model-visible error result;
- MUST preserve source order;
- MUST be distinguishable by stable error code:
  `tool_not_found`, `invalid_arguments`, `classification_failed`, or `policy_denied`.

The final no-effect attempt record preserves failure taxonomy:

- resolve, validation, or classification failure appends `ToolAttemptRejected` with the matching `failed_at` stage;
- policy denial appends the existing `ToolAttemptDenied` and MUST have `failed_at: Policy` plus a durable `PolicyDecision::Deny`;
- `ToolAttemptDenied` MUST NOT be reused as a generic preparation-failure record.

For an invalid/unknown call, unavailable classification fields remain `None` rather than fabricated. The `failed_at` field records the preparation stage that failed.

### 7.4 Allowed execution settlements

For an allowed call:

1. atomically append, in this order: `ToolAttemptStarted`, `ToolCallPrepared::Executable`, and `ToolEffectStarted`;
2. commit that append;
3. execute once with a cancellation token;
4. append and commit `ToolEffectCompleted` with normalized output/error;
5. atomically append the one reserved model-visible result and `ToolAttemptCompleted`;
6. commit before processing the next call or issuing another model request.

This creates the required T011 repair boundary: `ToolEffectCompleted` may be durable while the model-visible result entry is absent.

A tool execution error is model-visible and normally returns control to the model. It does not automatically fail the Agent.

### 7.5 Attempt interruption

The durable vocabulary MUST represent an interrupted tool attempt separately from denial, completion, and intervention.

- `ToolAttemptStarted` without `ToolEffectStarted` means no effect boundary was crossed; recovery may settle/re-run preparation according to current durable state.
- `ToolEffectStarted` without `ToolEffectCompleted` is effect ambiguity.
- Safe ambiguous effect: append `ToolAttemptInterrupted`, then a new attempt may execute.
- Never ambiguous effect: append `ToolAttemptIntervention` and terminalize `RequiresIntervention`.
- Retries never mutate a prior attempt into success.

### 7.6 Reducer invariants

The reducer MUST reject:

- preparation for an unknown logical call or non-current assistant entry;
- duplicate preparation of one attempt or preparation data that conflicts across attempts for the same logical call;
- `ToolEffectStarted` before an executable preparation or after a no-effect disposition;
- result entry ID different from the reserved ID;
- effect completion without effect start, or for a denied/rejected call;
- attempt completion without a durable effect result or no-effect disposition;
- `ToolAttemptDenied` after `ToolEffectStarted`;
- `ToolAttemptRejected` for a Policy failure or `ToolAttemptDenied` for a Resolve/Validate/Classify failure;
- more than one model-visible result per logical call;
- tool attempts/results outside provider source order;
- tool identity/version, arguments digest, effect, replay, or Allow/Deny decision changes across attempts for one logical call;
- records after terminal Agent result.

## 8. Functional Agent kernel — T008

### 8.1 Shape

`ww-agent-core` owns a small functional driver. It is not a session object and does not own transport, database construction, or product UI.

Its injected dependencies are:

- `Arc<dyn ModelProvider>`;
- immutable tool registry;
- `Arc<dyn ToolPolicy>`;
- `Arc<dyn AgentStore>`;
- clock/ID sources where deterministic tests require them;
- cancellation token;
- typed Agent configuration.

### 8.2 Typed run configuration

The kernel MUST operate from a typed configuration equivalent to:

```rust
pub struct AgentRunConfiguration {
    pub provider: ProviderId,
    pub model: ModelId,
    pub system_prompt: Option<String>,
    pub tools: Vec<ToolIdentity>,
    pub limits: AgentLimits,
}
```

Until G010, this may be serialized inside the existing configuration JSON field. The kernel MUST fail before provider/tool work if the stored configuration cannot be decoded or a pinned tool is unavailable. T008 may introduce `AgentLimits` with permissive/unbounded defaults so the configuration shape is stable; T010 owns limit validation and enforcement.

### 8.3 Model request construction

The model request is derived only from:

- typed run configuration;
- ordered durable context entries;
- ordered pinned tool specs.

Entry mapping:

- `UserInput` → user message;
- `AssistantMessage` → assistant message;
- `ModelVisibleToolResult` → tool result carrying the original provider call ID and tool name.

Tool results MUST appear in provider source order.

### 8.4 Model attempt boundary

Before provider I/O, durable history MUST contain an attempt record with:

- attempt ID;
- request ordinal;
- provider ID;
- model ID;
- canonical request digest.

A recommended additive record is `ModelRequestPrepared`. It and `ModelAttemptStarted` may be appended atomically.

The request digest is SHA-256 over the same recursively key-sorted compact JSON canonicalization used for tool arguments, applied to the normalized `ModelRequest`.

`ModelProvider::stream` may fail before yielding a stream even though Pi's `StreamFn` encodes request failures inside its stream contract. G003 preserves the already-completed T002 Rust port and normalizes either form at the kernel boundary: an outer `ProviderError` appends one typed interrupted/failed model-attempt outcome, creates no assistant entry, launches no tool, and is not automatically retried.

### 8.5 Mandatory stream finalization

The production consumption path MUST:

```rust
while let Some(event) = stream.next().await {
    assembler.push(event?)?;
}
assembler.finish()
```

It MUST drain through EOF even after a terminal event so post-terminal events are detected.

Outcomes:

- valid completion → immutable assistant entry;
- provider `Failed` → interrupted/failed model attempt and terminal Agent failure;
- provider `Aborted` due to durable cancellation → interrupted attempt and Agent cancellation;
- stream item error or unexpected EOF → interrupted attempt, never a partial assistant entry;
- malformed/truncated tool calls → protocol failure, no tool effect.

### 8.6 Assistant persistence

A finalized assistant response and `ModelAttemptCompleted` record MUST commit before any requested tool executes.

For every tool call, allocate stable logical IDs in provider source order before serializing the assistant entry.

`CompletionReason` behavior:

- `Stop` with no tools → commit a successful terminal result;
- `ToolUse` with completed calls → process those calls;
- `Length` → preserve the assistant entry for audit but fail the Agent with code `model_length`;
- inconsistent completion/tool structure is already rejected by the assembler.

### 8.7 Tool loop

Process each logical call sequentially:

```text
recover current call position
→ classify/validate/policy
→ persist pre-effect state
→ execute or produce no-effect result
→ persist effect output
→ append one model-visible result
→ finalize attempt
→ continue to next source-order call
```

After all results are model-visible, append `TurnCommitted` with result IDs in source order and issue the next model request.

### 8.8 Terminal behavior

Text-only success commits `AgentResultCommitted::Succeeded` after the assistant entry is durable.

The T008 kernel does not yet own common G002 terminalization; T009 adds that binding.

No automatic provider retry/backoff, parallel tools, follow-up queue, or compaction is added.

### 8.9 Mutation ownership and optimistic conflicts

At most one kernel invocation may successfully own each mutation transition for one Agent run. Competing drivers may read concurrently, but every decision cycle MUST:

1. read one `AgentHistorySnapshot` and its version;
2. reduce that exact snapshot;
3. choose one bounded next durable append or one external operation already authorized by committed state;
4. append with the snapshot's expected version;
5. cross an external provider/effect boundary only after the authorizing append commits.

If append returns an optimistic conflict, the stale decision is discarded. The kernel MUST perform no external operation from that stale decision, reload/reduce, and either continue from the new state or return a typed ownership/conflict outcome. A conflict after another owner has already committed effect output/result is resolved from durable state and MUST NOT cause re-execution.

This adapts the Harness single-writer record protocol while retaining G002/T004 optimistic concurrency rather than introducing a new lease subsystem in G003.

## 9. Common lifecycle and cancellation — T009

### 9.1 One-to-one identity

One Agent run maps to exactly one G002 common execution through the existing coordinator link.

The binding MUST reject:

- missing link;
- mismatched link;
- common execution kind other than `agent`;
- conflicting terminal states without a defined repair.

### 9.2 Start behavior

- Pending and not cancel-requested → atomically/optimistically transition common execution to Running, then run the Agent.
- Running/Waiting after restart → reconstruct Agent state and continue.
- Cancel requested before provider/tool start → commit Agent cancellation and settle common execution Cancelled without launching work.
- Common terminal + matching Agent terminal → return the durable result without model/tool work.

### 9.3 Cancellation token behavior

The common runtime owns the root token for an execution.

- repeated local registrations MUST observe the same root cancellation;
- consumers SHOULD receive child tokens so a consumer cannot cancel sibling work;
- durable `request_cancel` commits before the root token is signaled;
- provider and tool calls receive child tokens;
- the kernel checks durable cancellation before every provider/tool launch;
- after validation/classification/policy, the kernel checks cancellation again immediately before committing `ToolEffectStarted`; an already-durable request prevents that marker and prevents invocation;
- once `ToolEffectStarted` commits, cancellation cannot erase ambiguity and the recorded replay policy governs settlement;
- terminal settlement unregisters the root.

### 9.4 Cancellation during ambiguity

- provider cancellation before final response → no assistant entry; Agent Cancelled.
- safe tool cancellation with no result → record interruption; because caller cancellation is terminal, do not retry.
- never-replayable tool cancellation after start with no durable result → `RequiresIntervention`, because cancellation does not prove the effect did not occur.
- a completed durable result is repaired/returned rather than discarded merely because cancellation arrived afterward.

### 9.5 Terminal mapping and repair

| Agent terminal result | Common execution |
| --- | --- |
| Succeeded | Succeeded |
| Failed | Failed |
| Cancelled | Cancelled |
| TimedOut | TimedOut |
| BudgetExhausted | BudgetExhausted |
| RequiresIntervention | RequiresIntervention |

Agent result is authoritative for Agent semantics. If the Agent result is durable and common execution is non-terminal, repair common terminal state idempotently without provider/tool replay.

Generic G002 lifecycle methods/events needed for statuses already present in `ExecutionStatus` MAY be added, but Agent-specific DTOs MUST NOT enter shared runtime contracts.

### 9.6 Deterministic recovery precedence

When more than one condition is observable, the reducer/driver applies this order before launching work:

1. corrupt or contradictory durable history fails closed without guessed repair;
2. an existing Agent terminal result is returned and its common terminal state is repaired if needed;
3. a started Never effect without durable completion settles `RequiresIntervention`;
4. a durable cancellation request settles `Cancelled` when no higher condition applies;
5. an expired effective deadline settles `TimedOut`;
6. an exhausted count/token budget settles `BudgetExhausted`;
7. otherwise the one recovery/next action derived from history may proceed.

This order is fixed so restart does not choose a different disposition from the same durable state. In particular, cancellation, timeout, or budget exhaustion MUST NOT mask Never-effect ambiguity.

## 10. Deadlines and execution budgets — T010

### 10.1 Limits model

```rust
pub struct AgentLimits {
    pub deadline: Option<DateTime<Utc>>,
    pub max_model_requests: u64,
    pub max_turns: u64,
    pub max_tool_calls: u64,
    pub max_input_tokens: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub max_total_tokens: Option<u64>,
}
```

Count limits MUST be positive. The effective deadline is the earlier of Agent configuration deadline and common execution deadline.

### 10.2 Durable counting semantics

- `model_requests` = number of durable model-attempt starts, including restart/retry attempts.
- the `max_turns` budget = number of durable `ModelAttemptCompleted` records, including terminal assistant responses. Add a distinct derived `completed_model_turn_count` (or equivalently named field); do not repurpose the existing `AgentRecoveryState.turn_count`, which remains the number of durable `TurnCommitted` records established by T003.
- `tool_calls` = number of durable `ToolAttemptStarted` handling attempts, including safe replay attempts and no-effect denied/rejected attempts.
- input/output/total tokens = sum of finalized normalized provider usage; cached-token fields do not reduce total-token accounting.

Counters are reconstructed from durable history, never process-local mutable counters.

### 10.3 Reservation and enforcement points

Before model attempt start:

- check cancellation/deadline;
- verify the next model request fits `max_model_requests`;
- verify the next turn fits `max_turns`;
- persist/reserve the attempt before provider I/O.

Before tool attempt start:

- check cancellation/deadline;
- verify the attempt fits `max_tool_calls`;
- persist/reserve the attempt before effect execution.

After finalized usage:

- add usage durably;
- when a token limit is reached or exceeded, do not issue another model request;
- the already-finalized response remains durable.

### 10.4 Boundary definitions

- `now >= deadline` means expired.
- Reaching a count limit is allowed for the operation just durably reserved; launching operation `limit + 1` is forbidden.
- Token limits cannot predict unknown provider output; enforcement occurs before the next model request.
- Deadline during active provider/tool work cancels its child token and settles TimedOut unless never-replayable ambiguity requires intervention.
- Budget exhaustion produces a typed Agent result and common `BudgetExhausted`, not a provider/tool failure.

## 11. Recovery and fault matrix — T011

The test harness MUST support deterministic interruption after these boundaries:

| Fault | Durable state | Required restart action |
| --- | --- | --- |
| F1 creation commit | common execution + Agent run + link + input exist | continue once from known IDs; do not create a second run |
| F2 model start commit | model attempt started; no final response; subcases cover no event and transient partial deltas because deltas are not canonical durable entries | mark interrupted; create a new attempt only if cancellation/deadline/budget permit |
| F3 model finalization commit | assistant entry and completion record exist | do not contact provider; process pending tools or terminalize |
| F4 safe effect-start before durable result | replay `Safe`, `ToolEffectStarted`, no `ToolEffectCompleted` | append interruption; execute a new attempt; one logical result |
| F5 Never effect-start/effect ambiguity before durable result | replay `Never`, `ToolEffectStarted`, no `ToolEffectCompleted` | do not execute; append intervention; settle `RequiresIntervention` |
| F6 tool effect output durable, model-visible entry absent | effect output exists with reserved result ID | append exactly the missing entry and completion; do not execute |
| F7 all model-visible tool results durable, turn absent | ordered results exist | append one `TurnCommitted`; do not execute/provider-call |
| F8 Agent result durable, common execution non-terminal | Agent terminal result exists | terminalize common execution once; no provider/tool work |

Tests MUST restart in a distinct OS process against the same SQLite database for at least F1–F8.

A second restart after each repair MUST be a no-op with respect to external effects, logical results, and terminal events.

### 11.1 Corrupt versus repairable

Repair only states explicitly listed in the matrix.

Unknown references, mismatched reserved IDs, duplicate logical results, source-order violations, policy/replay changes, or incompatible common/Agent terminal states MUST fail closed as corruption or intervention; they MUST NOT be guessed into a repaired state.

## 12. Evaluations and terminal review — T012

T012 MUST record current EvaluationRuns for:

1. Agent protocol conformance;
2. Agent durable recovery safety;
3. Agent kernel execution conformance.

Each run is appended under the corresponding check in `EVALUATIONS.md` and records:

- evaluation/check name;
- exact reviewed commit;
- command/fixture;
- result;
- evidence location;
- evaluator mode;
- date.

The terminal review MUST verify:

- every G003 requirement maps to passing evidence;
- no concrete transport/filesystem/product/Flow/Orchestration scope entered;
- no unsafe replay path exists;
- Agent and common lifecycle repair is idempotent;
- all permanent gates pass on the exact reviewed commit;
- no open G003 Stop Condition remains triggered.

Goal acceptance remains a separate requester action.

## 13. Project structure

Expected implementation shape:

```text
crates/
  ww-agent-tools/
    Cargo.toml
    src/
      lib.rs
      identity.rs
      schema.rs
      registry.rs
      policy.rs
      fixtures.rs
    tests/
      tool_contract.rs

  ww-agent-core/
    src/
      history.rs          durable entry/record vocabulary
      reducer.rs          pure recovery projection
      kernel.rs           functional provider/tool loop
      limits.rs           durable limit decisions
      lifecycle.rs        common execution binding/repair
      lib.rs
    tests/
      recovery.rs
      kernel.rs
      lifecycle.rs
      limits.rs

  ww-agent-store-sqlite/
    src/bin/
      agent-kernel-fixture.rs     test-only process driver
    tests/
      recovery_matrix.rs
```

Exact file names MAY vary. Module responsibilities and dependency direction MUST not.

The implementation SHOULD keep each internal work unit near five implementation files. More files are allowed when one coherent acceptance boundary would be weakened by an artificial split; record the rationale in the Task review.

## 14. Commands

Permanent gate:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Focused commands expected during implementation:

```bash
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
cargo test -p ww-agent-core --test kernel --locked
cargo test -p ww-agent-core --test lifecycle --locked
cargo test -p ww-agent-core --test limits --locked
cargo test -p ww-agent-store-sqlite --test recovery_matrix --locked
```

Package/test target names MUST match final files; update Task verification commands if an approved file name changes.

## 15. Testing strategy

- Unit tests: identities, schema profile, deterministic digest, policy, fixture outputs, limit decisions.
- Contract tests: registry/validation/policy ordering and zero-effect denial.
- Reducer tests: valid transitions and every new corrupt-history case.
- Kernel integration tests: text-only and model→tool→model.
- Runtime integration tests: lifecycle/cancellation/terminal repair.
- OS-process tests: F1–F8 restart matrix.
- Full workspace gate: required before each Task is marked complete and for T012 exact-code review.

Tests MUST assert both positive outcomes and prohibited side effects/provider calls.

## 16. Boundaries

### Always

- reconstruct from durable history before deciding the next action;
- validate exact parsed arguments before policy or execution;
- persist ambiguity-sensitive identity/classification/start state before effect/provider work;
- preserve provider source order;
- use typed WorkWeave-owned errors at crate boundaries;
- run focused tests and the permanent gate;
- update Task and Verification evidence only after the code basis passes.

### Ask first / escalate under governance

- change Goal or ADR-0003 boundaries;
- renumber/redefine used Tasks;
- add a concrete provider, real capability, new product surface, Flow/OWS, or Orchestration semantics;
- add a new prerequisite Goal/Task;
- replace Draft 2020-12, coercion policy, or replay model;
- change the durable repair matrix beyond a clarification required by existing acceptance;
- introduce a new crate other than the already specified `ww-agent-tools`.

### Never

- execute invalid or denied calls;
- silently replay `ReplayPolicy::Never`;
- produce two model-visible results for one logical call;
- treat EOF without finalization as success;
- rely on process-local counters for recovery or limits;
- mutate completed Task meanings/evidence;
- persist secrets or hidden chain-of-thought;
- bypass the AgentStore/common runtime seams with direct database access from the kernel.

## 17. Normative requirement index

### Tool requirements

| ID | Requirement |
| --- | --- |
| TOOL-01 | Stable non-empty tool identity/version and exact run pinning |
| TOOL-02 | Immutable duplicate-rejecting registry and configured model-spec order |
| TOOL-03 | Explicit self-contained Draft 2020-12 schema compiled at registration |
| TOOL-04 | No HTTP/file external reference resolution |
| TOOL-05 | Exact non-coercing argument validation with WorkWeave-owned errors |
| TOOL-06 | Parsed `Value` is the sole executable argument authority |
| TOOL-07 | Deterministic recursively key-sorted SHA-256 argument digest |
| TOOL-08 | Validate before effect/replay classification; classify before policy |
| TOOL-09 | Centralized deterministic Allow/Deny policy; tools do not authorize themselves |
| TOOL-10 | Invalid/unknown/denied calls perform zero effect and create one ordered error result |
| TOOL-11 | `test.echo` is deterministic, pure, and replay-safe |
| TOOL-12 | `test.unsafe_once` is synthetic, probe-observable, and never replayable |
| TOOL-13 | Tool public contracts contain no Agent-owned operational identity or core dependency |

### Durability requirements

| ID | Requirement |
| --- | --- |
| DUR-01 | Logical call, attempt, source position, provider call, and reserved result identities are stable |
| DUR-02 | Tool/version, digest, effect, replay, policy, and explicit effect-start marker are durable before allowed effect invocation |
| DUR-03 | No-effect dispositions are atomic, distinguish their failed preparation stage, and contain no effect-start record |
| DUR-04 | Effect output/error is durable before the model-visible result repair boundary |
| DUR-05 | Safe interruption and Never intervention are distinct append-only attempt outcomes |
| DUR-06 | Retries create new attempts and never rewrite previous attempts |
| DUR-07 | Reducer fails closed on unknown/mismatched/duplicate/out-of-order tool history |
| DUR-08 | One logical call has at most one reserved/committed model-visible result in source order |
| DUR-09 | Reject/reload optimistic conflicts before external work; stale decisions never authorize provider/tool execution |
| DUR-10 | Preparation rejection and policy denial use distinct terminal attempt records |

### Kernel requirements

| ID | Requirement |
| --- | --- |
| KERN-01 | Typed stored configuration and exact pinned tools are validated before work |
| KERN-02 | Model requests derive only from ordered durable entries and configuration |
| KERN-03 | Provider/model/request digest attempt state is durable before provider I/O |
| KERN-04 | Provider streams drain through EOF and finalize exactly once |
| KERN-05 | Finalized assistant entry/usage commits before tool handling |
| KERN-06 | Tool calls execute sequentially and return ordered model-visible results |
| KERN-07 | Turn commit follows all durable results and precedes the next provider request |
| KERN-08 | Text success, tool round trip, failures, cancellation, and Length have explicit dispositions |
| KERN-09 | Kernel owns no concrete transport, SQLite, capability, Flow, or product surface |
| KERN-10 | Outer provider dispatch errors normalize to one durable failed/interrupted attempt with no assistant/effect |
| KERN-11 | Each mutation cycle derives from one versioned snapshot and discards stale decisions on conflict |

### Lifecycle requirements

| ID | Requirement |
| --- | --- |
| LIFE-01 | One Agent run links to one common execution of kind `agent` |
| LIFE-02 | Pending starts once; Running/Waiting resumes; matching terminal performs no work |
| LIFE-03 | Durable cancellation request precedes live root-token signal |
| LIFE-04 | Provider/tool consumers receive cancellation children from one execution root |
| LIFE-05 | Cancellation launches no new work and preserves already durable results |
| LIFE-06 | Never-replayable ambiguity maps to RequiresIntervention |
| LIFE-07 | Agent/common terminal mapping and repair are idempotent and semantically separated |
| LIFE-08 | Corruption, durable terminal state, Never ambiguity, cancellation, deadline, and budget use fixed recovery precedence |

### Limit requirements

| ID | Requirement |
| --- | --- |
| LIMIT-01 | Positive typed limits and deterministic effective deadline |
| LIMIT-02 | Model/turn/tool counters derive from specified durable records |
| LIMIT-03 | Provider/tool capacity is checked and reserved before launch |
| LIMIT-04 | Operation `limit + 1` is never launched |
| LIMIT-05 | Finalized provider usage accumulates durably |
| LIMIT-06 | Token limit stops before the next provider request |
| LIMIT-07 | `now >= deadline` expires; active expiry cancels child work |
| LIMIT-08 | BudgetExhausted/TimedOut are explicit unless Never ambiguity requires intervention |

### Recovery requirements

| ID | Requirement |
| --- | --- |
| REC-01 | F1 creation restart continues the existing run once |
| REC-02 | F2 started model attempt becomes an interrupted/new audited attempt when allowed |
| REC-03 | F3 finalized model response is never re-requested before pending handling |
| REC-04 | F4 Safe effect-start/no-result retries as a new attempt with one logical result |
| REC-05 | F5 Never effect-start/no-result never re-executes and requires intervention |
| REC-06 | F6 durable effect output repairs exactly the reserved result without execution |
| REC-07 | F7 missing turn commit repairs once without provider/tool work |
| REC-08 | F8 Agent terminal/common nonterminal repairs once without provider/tool work |
| REC-09 | F1–F8 execute across real OS-process restart and a second restart is idempotent |
| REC-10 | States outside the explicit matrix fail closed rather than guessed repair |
| REC-11 | F2 covers both pre-event and transient-partial-stream process loss without treating partial deltas as durable assistant state |

### Evaluation requirements

| ID | Requirement |
| --- | --- |
| EVAL-01 | Current EvaluationRuns pin exact commit, command/fixture, mode, result, date, and evidence |
| EVAL-02 | Every normative requirement maps to passing Verification/Evaluation evidence |
| EVAL-03 | Exact reviewed commit passes the permanent local and hosted gate |
| EVAL-04 | Independent review confirms architecture, scope, durability, and replay boundaries |
| EVAL-05 | Goal acceptance is explicit requester action, not inferred from branch placement |

## 18. Requirement traceability

| Requirement family | Primary Task | Verification section |
| --- | --- | --- |
| TOOL-01…TOOL-13 | T007 | `V-T007` |
| DUR-01…DUR-10 | T007/T008/T011 | `V-T007`, `V-T008`, `V-T011` |
| KERN-01…KERN-11 | T008 | `V-T008` |
| LIFE-01…LIFE-08 | T009 | `V-T009` |
| LIMIT-01…LIMIT-08 | T010 | `V-T010` |
| REC-01…REC-11 | T011 | `V-T011` |
| EVAL-01…EVAL-05 | T012 | `V-T012` |

The detailed check identifiers live in `VERIFICATION.md`.

## 19. Open questions

No unresolved question blocks approval of this specification.

Later, separately governed work may revisit:

- durable schema/payload evolution and cross-language canonicalization in proposed G010;
- approval-bearing policy decisions;
- idempotency-key replay policy;
- parallel tools;
- concrete providers and bounded filesystem capability in G004.
