# G003 — Durable Agent Kernel

## Statement

Prove that WorkWeave Agent has a durable, provider-neutral probabilistic execution kernel that can execute and recover one recorded model → tool → model run on the G002 substrate without network-provider, local-filesystem, CLI, SDK-product, or Flow concerns contaminating the kernel.

## State

- active

## Architecture Decision Records

- `docs/adr/ADR-0003-g003-thin-agent-kernel.md` — accepted.

## Success Criteria

- A deterministic recorded provider completes one text-only run and one model → tool → model run through the real Agent kernel.
- Provider-neutral stream assembly rejects malformed, duplicate-finalized, truncated, or incomplete tool-call output before an effect can execute.
- Durable Agent entries and operational records reconstruct an identical `AgentRecoveryState` after process restart.
- One logical tool call can never produce two committed model-visible results, including across crash/restart.
- An incomplete replay-safe tool attempt can resume as a distinct audited attempt; an incomplete non-replayable attempt is never silently re-executed and causes `RequiresIntervention`.
- Durable cancellation propagates to an active provider or tool cancellation token and settles consistently with the G002 execution lifecycle.
- Deadline, model-request, turn, and tool-call budgets terminate deterministically; token usage is enforced before the next model request when normalized usage exists.
- Agent terminal result, usage, finalized assistant messages, tool requests/results, and recovery-relevant attempts are auditable from durable records.
- G003 introduces no concrete network provider, local filesystem tool, `ww agent` CLI, TUI, server, Flow/OWS type, or WorkWeave Orchestration semantic type.

## Requirements

- Preserve G002 state/event consistency, optimistic concurrency, cancellation, and audit invariants.
- Keep provider-specific types outside `ww-agent-core` and `ww-agent-tools`.
- Keep Agent-specific persistence out of the shared `ww-store` semantic API even when embedded storage shares one SQLite file.
- Commit any start-time state that must be consistent across common execution and Agent state atomically or prove an explicit idempotent recovery path before relying on partial state.
- Keep finalized context-bearing entries immutable; operational retries create new attempt records rather than rewriting prior attempts.
- Validate complete tool arguments before policy and execution.
- Execute tool calls sequentially in G003 and preserve provider call order in model-visible results.
- Fail closed on corrupt record ordering, unknown references, unsupported provider events, invalid tool schemas, and ambiguous non-replayable effects.
- Persist normalized provider/model/tool/configuration pins needed to audit the run; raw provider payload retention remains off by default.
- Add process-kill/fault-injection proof at model and tool durability boundaries before Goal completion.

## Boundaries

- Recorded provider only. The first concrete OpenAI adapter moves to G004.
- Test tools only: one deterministic replay-safe structured tool plus one synthetic non-replayable fault fixture. Bounded `fs.read` moves to G004.
- No public SDK or CLI Agent surface; G003 is exercised through crate-level/integration harnesses. Public Agent SDK/CLI moves to G004.
- No sessions, steering/follow-up queues, branching, compaction, subagents, MCP, plugins, remote A2A, shell, writes, or network tools.
- No parallel tool scheduling.
- No Flow/OWS dependency and no WorkWeave Goal/Task/Evaluation/epistemic/deontic/temporal semantics.

## Required Evaluations

- `Agent protocol conformance` — provider stream and tool-call assembly rules.
- `Agent durable recovery safety` — restart, corruption, replay, and ambiguous-effect behavior.
- `Agent kernel execution conformance` — model/tool loop, ordering, common lifecycle, cancellation, and limits.

See `EVALUATIONS.md`.

## Dependencies

- G002 — Shared Runtime Walking Skeleton, including independent review acceptance.
