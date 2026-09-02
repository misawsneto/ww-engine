# G003 — Thin Agent Kernel

## Statement

Prove that WorkWeave Agent can execute one bounded, restart-safe provider → tool → provider run on the G002 substrate while remaining provider-neutral, tool-modular, auditable, cancellable, resource-bounded, and independent of WorkWeave Flow.

## State

- proposed

## Architecture Decision Records

- `docs/adr/ADR-0003-g003-thin-agent-kernel.md` — proposed; must be accepted before G003 activation.

## Boundaries

- Implement one bounded Agent Run, not a general conversational coding-agent product.
- No Flow/OWS dependency or Flow state.
- No WorkWeave Goal/Task/Evaluation/epistemic/deontic/temporal semantics.
- One concrete provider adapter plus a deterministic recorded provider for CI.
- Initial user-visible tools are read-only/deterministic: `fs.read` and one structured test tool.
- No bash, workspace writes, network tools, MCP, plugins, subagents, remote A2A, or public extension ABI.
- No Agent sessions, steering/follow-up queues, branching, or compaction.
- SDK and CLI are in scope; TUI and server/API are deferred.
- Sequential tool execution only in G003; preserve call/result ordering and leave parallel-safe execution for later.

## Success Criteria

- Provider-neutral message/request/stream types can represent one complete text/tool round trip without vendor types leaking into `ww-agent-core`.
- A deterministic recorded provider completes `model → tool → model → terminal result` through the real Agent kernel.
- One concrete OpenAI adapter satisfies the same provider contract through recorded HTTP/stream fixtures; mandatory CI requires no external credential.
- Tool arguments are schema-validated before execution and policy evaluation cannot be bypassed.
- Durable Agent entries/records reconstruct one `AgentRecoveryState` after process restart.
- Finalized model responses, tool requests/results, usage, turn settlement, and terminal result are auditable; transient deltas need not be durable.
- Replay-safe incomplete work can resume without duplicate committed tool results.
- An incomplete non-replayable tool attempt is never silently re-executed and settles to `RequiresIntervention`.
- Durable cancellation propagates to an active provider/tool cancellation token and terminal settlement remains auditable.
- Deadline, maximum model requests, maximum turns, and maximum tool calls are enforced; token budget is enforced when normalized usage is available.
- Agent terminal state maps cleanly onto the G002 common execution lifecycle.
- `ww agent run` and SDK APIs execute and inspect the run without direct database access from CLI code.
- No crate in the Agent implementation depends on `ww-flow-*` or OWS types.

## Requirements

- Preserve G002's state/event consistency checks and optimistic concurrency behavior.
- Keep Agent-specific persistence logically separate from common runtime persistence even when the embedded profile uses the same SQLite file.
- Fail closed on corrupt Agent history, invalid tool schemas, unsupported provider events, and ambiguous non-replayable effects.
- Keep raw provider payload retention off by default; durable normalized audit is sufficient for G003.
- Record exact provider/model/tool/configuration pins needed to audit a run.
- Add fault-injection tests at model and tool durability boundaries before the Goal can complete.

## Dependencies

- G002 — Shared Runtime Walking Skeleton, including independent review acceptance.
