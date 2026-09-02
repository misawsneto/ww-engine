# Plan

## Title

- Prove the bounded probabilistic worker

## State

- draft

## Strategy

1. Accept ADR-0003 only after G002 independent review closes.
2. Add provider-neutral and tool-neutral contracts before any concrete provider code.
3. Implement deterministic recorded provider and deterministic tools first so the model/tool state machine is executable without external credentials.
4. Add Agent-specific SQLite persistence/reducer and fault injection before adding the live provider adapter.
5. Implement the functional model → tool → model kernel with explicit persistence boundaries modeled after Pi's small `runAgentLoop` shape rather than its larger coding-agent session layer.
6. Bind one Agent Run to the G002 common execution lifecycle and cancellation path.
7. Add the OpenAI adapter behind the normalized provider contract and prove it with recorded transport fixtures.
8. Add SDK/CLI surfaces only after kernel/recovery contracts pass.
9. Perform architecture and recovery review before terminal acceptance.

## Stop Conditions

- Stop if Agent implementation requires Flow/OWS state or WorkWeave Orchestration semantics.
- Stop if provider-specific types leak into `ww-agent-core`.
- Stop if Agent-specific persistence forces Agent DTOs into the shared `ww-store` port.
- Stop if recovery cannot distinguish replay-safe from ambiguous non-replayable effects.
- Stop if a crash window can create two committed tool results for one logical tool call.
- Stop if CLI needs direct database access instead of SDK/kernel projection.

## Rollback

- Revert G003 implementation while retaining ADR-0003, the Goal packet, recorded provider fixtures, and review evidence documenting the rejected architecture.
