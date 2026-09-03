# ADR-0003 — Durable provider-neutral Agent kernel

- Status: accepted
- Goal: G003 — Durable Agent Kernel
- Recorded: 2026-09-02
- Activation condition: satisfied on 2026-09-02 after independent G002 approval.

## Context

G002 proved a semantically neutral execution substrate. The next high-risk dependency is not OpenAI connectivity or CLI UX; it is whether a probabilistic model/tool loop can be made durable and restart-safe without leaking provider, tool, or Flow semantics into the shared runtime.

Pi demonstrates that the valuable execution core is small: normalized model streaming, finalized assistant responses, validation/preflight, ordered tool results, and a loop that returns to the provider until terminal. Pi Harness contributes a separate useful idea: immutable context entries plus operational records reduced into recoverable state. WorkWeave should combine those patterns narrowly, not recreate Pi's larger coding-agent session product layer.

The previous G003 draft also included the first concrete provider, `fs.read`, SDK, and CLI. Planning review classified several tasks as independently large and risk-separable. Those product/network deliverables are moved to G004 so G003 can prove durability before consequential or external capabilities exist.

## Decision

### Kernel boundary

1. One `AgentRun` is one bounded probabilistic execution bound to one G002 common execution.
2. `ww-agent-core` owns the functional loop, Agent recovery model, limits, and settlement; it owns no concrete transport, SQLite, filesystem, CLI, Flow, or WorkWeave Orchestration semantics.
3. G003 has no `ww-flow-*` or OWS dependency.
4. G003 uses a deterministic `RecordedProvider` only. The first concrete network provider moves to G004.

### Physical crates

Create only exercised boundaries:

- `ww-agent-provider` — normalized provider/model/request/stream protocol and deterministic recorded provider fixtures;
- `ww-agent-tools` — tool identity/schema/policy/replay contracts plus deterministic/synthetic test fixtures;
- `ww-agent-core` — Agent entries/records, recovery reducer, functional loop, limits, persistence port, and settlement;
- `ww-agent-store-sqlite` — Agent-specific embedded persistence and backend transaction coordination.

Do not create `ww-agent-openai`, `ww-agent-tools-local`, Agent CLI/TUI/server crates or surfaces in G003.

### Provider protocol

5. Normalize provider output into `Started`, text/tool-call deltas, usage, completion, failure, and cancellation events.
6. Stream assembly is a pure state machine; invalid ordering, duplicate finalization, disconnect-before-finalization, and incomplete/truncated tool calls fail closed.
7. Finalized normalized assistant responses are durable. Transient deltas are live by default and need not be canonical audit state.
8. Vendor-specific request/response types may exist only in later concrete adapters.

### Entry/record durability model

9. Context-bearing entries are immutable: user input, finalized assistant message, and model-visible tool result.
10. Operational records append execution history: model attempt start/interruption/completion, tool attempt start/denial/completion/intervention, turn commit, and Agent result commit.
11. Retries append new attempts and never rewrite prior attempts into success.
12. A pure reducer reconstructs `AgentRecoveryState` and fails closed on impossible references/order/duplicate logical results.

### Tool model

13. Tool arguments must be complete valid JSON and pass JSON Schema validation before policy or execution.
14. A tool has stable identity/version, effect descriptor, replay policy, and async execution contract.
15. G003 uses only `test.echo` (deterministic replay-safe) and `test.unsafe_once` (synthetic non-replayable fault fixture).
16. Tool calls execute sequentially in provider source order. One logical tool call may have multiple attempts but at most one committed model-visible result.
17. Policy denial produces one durable model-visible error result and performs no effect.

### Persistence and recovery

18. Agent data is logically separate from common runtime data even in one physical SQLite database.
19. Agent DTOs do not enter the shared `ww-store` semantic API. Backend-specific transaction coordination may be added only at the SQLite implementation seam.
20. Before provider I/O, model attempt identity/request digest/provider-model pin/budget reservation are durable.
21. Before tool effect execution, logical call/tool-version/arguments digest/replay policy/policy decision/attempt start are durable.
22. A replay-safe incomplete tool attempt may retry as a new audited attempt. `ReplayPolicy::Never` ambiguity is never re-executed and settles to `RequiresIntervention`.
23. Commit Agent terminal result before or atomically with common terminalization; recovery idempotently repairs Agent-terminal/common-nonterminal state without model/tool replay.
24. Fault-injection tests cover creation, model start, model finalization, tool start, tool result, turn commit, and terminal-settlement boundaries.

### Limits and cancellation

25. Enforce deadline, maximum model requests, maximum turns, maximum tool calls, and optional normalized token budgets.
26. Counters derive from durable history rather than process-local mutable counters.
27. Durable G002 cancellation propagates to active provider/tool cancellation tokens and terminal settlement remains explicit and auditable.

### Deferred product/network surface

28. Concrete OpenAI transport, bounded `fs.read`, Agent SDK projection, and `ww agent` CLI move to G004.
29. Sessions, steering/follow-up queues, compaction, parallel tools, MCP, plugins, write/process/network tools, multi-provider routing, server, and TUI remain later.

## Consequences

- G003 becomes approximately one G002-class proof instead of combining durability, network integration, filesystem policy, and product UX.
- Recovery safety is established before real external/model or filesystem effects are exposed.
- G004 can validate the kernel by substitution rather than changing kernel semantics.
- Future Flow invokes the Agent as a bounded execution primitive without importing Agent internals.

## Rejected alternatives

- Keep OpenAI, `fs.read`, SDK, and CLI inside G003.
- Start with a full Pi coding-agent/session abstraction.
- Put provider-specific types into `ww-agent-core`.
- Add bash/write/network tools before replay/intervention semantics are proven.
- Persist every stream delta as canonical state.
- Make Agent execution a Flow/LangGraph graph.

## Evidence basis

- Pinned Pi production Agent loop, tool validation/preflight/result-ordering seams, and provider-neutral `StreamFn` architecture.
- Pinned Pi Harness reducer/entry-record concepts as a future-architecture reference, not production behavior.
- G002 durable lifecycle/cancellation/event/SQLite evidence.
- `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`, especially sections 6, 10, 12, 14–17, 31.4–31.5, and implementation sequence.
