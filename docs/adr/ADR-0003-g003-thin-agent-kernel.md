# ADR-0003 — Durable provider-neutral Agent kernel

- Status: accepted
- Goal: G003 — Thin Agent Kernel
- Recorded: 2026-09-02
- Accepted: 2026-09-02 after independent G002 review acceptance.

## Amendment — 2026-09-03 (D015 scope split)

Sections from **Physical crates** onward were written before the G003/G004 split recorded in `DECISIONS.md` D015. Where they assign the concrete provider adapter (`ww-agent-openai`), bounded `fs.read`, and the Agent SDK/CLI surface to G003, those deliverables now belong to G004 under proposed `ADR-0004`. The Context and Kernel boundary sections below are current; read the later sections subject to this amendment.

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
