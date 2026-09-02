# ADR-0003 — Thin WorkWeave Agent kernel

- Status: proposed
- Goal: G003 — Thin Agent Kernel
- Recorded: 2026-09-02
- Activation condition: G002 independent review accepted and this ADR changed to `accepted`.

## Context

G002 proved the semantically neutral execution substrate. The next dependency is the leaf probabilistic worker that will later be callable from WorkWeave Flow. Pi shows that the valuable Agent core is a small provider-neutral model → tool → model loop, with persistence, policy, recovery, and product surfaces around that loop rather than inside one monolithic session object.

## Decision

### Kernel boundary

1. Implement a bounded `AgentRun` as one probabilistic execution primitive backed by one common `ExecutionRecord(kind = agent)`.
2. Keep the functional model/tool loop independent of CLI, TUI, remote server, WorkWeave Orchestration, and Flow.
3. G003 has no `ww-flow-*` dependency and no OWS types.

### Physical crates

Create only boundaries exercised by this Goal:

- `ww-agent-provider` — normalized provider/model request and streaming contracts plus recorded-provider fixtures.
- `ww-agent-tools` — tool contracts, registry, JSON-schema validation, effect/replay metadata, and bounded execution.
- `ww-agent-core` — Agent Run state/recovery model, functional loop, persistence port, limits, and settlement.
- `ww-agent-store-sqlite` — Agent-specific durable tables/reducer data in the embedded SQLite database.
- `ww-agent-openai` — first concrete provider adapter.
- extend `ww-sdk` and `ww-cli`; do not add TUI/server yet.

### Provider model

4. Normalize provider output into a stable stream: request started, text deltas, tool-call construction, usage, completion, failure, and cancellation.
5. Stream deltas are live by default; finalized normalized assistant responses are durable.
6. Vendor-specific request/response types stop at provider adapters.
7. CI uses a deterministic recorded provider; the concrete OpenAI adapter is verified with contract fixtures and may have an opt-in live smoke test outside mandatory CI.

### Tool model

8. A Tool exposes stable identity, JSON input schema, effect classification, execution mode, replay policy, and `execute`.
9. Validate arguments before policy or execution.
10. Initial user-visible tools are `fs.read` and one deterministic structured test tool. A synthetic non-replayable test tool exists only for recovery/fault tests.
11. G003 executes tool calls sequentially. The contract may represent future parallel-safe execution, but parallel scheduling is deferred until ordering/recovery semantics are proven.
12. Model-visible tool results preserve provider tool-call order.

### Durability and recovery

13. Persist Agent-specific state separately from common execution state even when both use one physical SQLite database.
14. Agent durable state contains stable context entries and operational records. Model deltas are not required for replay; finalized responses, tool requests/results, usage, and turn settlement are.
15. Any commit that must atomically change common execution state and Agent state uses one SQLite transaction through a backend coordination seam; do not put Agent types into `ww-store`.
16. Recovery reduces durable Agent records into an `AgentRecoveryState` and fails closed on impossible history.
17. An incomplete replay-safe tool may be retried as a new attempt. An incomplete `ReplayPolicy::Never` tool becomes `RequiresIntervention` and is never silently re-executed.
18. An interrupted model request may create a new attempt when cancellation/deadline/budget policy permits; the interrupted attempt remains auditable.

### Limits and settlement

19. Enforce deadline, maximum model requests, maximum turns, and maximum tool calls in G003. Normalize token usage when the provider supplies it and stop before the next request when a configured token budget is exhausted.
20. Cancellation propagates from the G002 durable cancel request to provider and tool cancellation tokens; settlement remains explicit and durable.
21. A terminal Agent result is committed before the common execution is terminalized, so recovery can idempotently finish a partially settled run.

### Product surface

22. Add Rust SDK and CLI support: `ww agent run`, `ww agent inspect`, and transcript/audit inspection sufficient to debug one bounded run.
23. Sessions, steering/follow-up queues, compaction, plugins/MCP, write/shell/network tools, multi-provider routing, server APIs, and TUI are explicitly deferred.

## Consequences

- G003 proves the probabilistic worker without becoming a coding-agent product clone.
- Flow can later invoke Agent through an adapter without importing Agent internals.
- Recovery semantics are designed before unsafe tools exist in the public tool set.
- The first provider adapter does not define core Agent types.
- Shared SQLite storage is operationally simple while preserving separate logical models.

## Rejected alternatives

- Start G003 with a full Pi-style coding-agent session layer.
- Put provider-specific types into the Agent kernel.
- Add bash/write/network tools before replay and intervention behavior is proven.
- Persist every stream delta as canonical state.
- Make an Agent itself a LangGraph/Flow graph.

## Evidence basis

- Pi functional Agent loop and `StreamFn` seams at the pinned Pi revision.
- Pi tool validation/preflight/result ordering and future Harness entry/record distinction.
- G002 durable lifecycle, cancellation, event journal, artifact, SDK, and SQLite evidence.
- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`, especially sections 10, 12, 15, 31.4–31.5, and 34.2.
