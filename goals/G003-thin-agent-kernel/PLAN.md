# Plan

## Title

- Prove the durable probabilistic kernel before adding network and product surfaces

## State

- active

## Slicing Strategy

Risk-first + contract-first.

G003 proves the three failure-sensitive contracts independently before integrating them:

```text
provider protocol + assembler ──→ recorded provider ──┐
                                                     │
Agent durable model + reducer ─→ SQLite persistence ─┼─→ kernel loop
                                                     │       ↓
tool contract + replay/policy fixtures ──────────────┘  runtime limits/cancel
                                                             ↓
                                                      fault/restart matrix
                                                             ↓
                                                        final review
```

The concrete OpenAI adapter, bounded `fs.read`, SDK projection, and CLI are deliberately moved to G004. This keeps G003 near G002's implementation/review class and makes recovery safety the acceptance boundary rather than network/product integration.

## Strategy

1. Close G002 independent review and accept ADR-0003 before activating G003.
2. Define normalized provider events and a pure stream assembler first; no HTTP client exists in this Goal.
3. Define immutable context entries, operational attempt records, and a fail-closed recovery reducer before implementing the loop.
4. Prove Agent-owned SQLite append/query/reconstruction first, then prove the narrower common/Agent transaction coordination seam separately.
5. Build a deterministic recorded provider conformance harness that exercises text, tool calls, failure, cancellation, truncation, and interrupted attempts.
6. Define the tool contract with schema validation, policy decision, replay classification, and deterministic/synthetic fixtures before any real filesystem/process effect exists.
7. Implement the smallest functional loop over those ports, sequential tool execution only.
8. Bind the loop to G002 lifecycle/cancellation first; then implement deadline and budget accounting as a separate bounded slice derived from durable state.
9. Run the fault matrix by killing/reopening around model/tool/turn/terminal boundaries.
10. Perform recovery/architecture review only after every required Evaluation has a current passing run.

## High-Risk Proof

The highest-risk proposition is not the model loop. It is:

> After a crash at any effect ambiguity boundary, durable Agent history deterministically identifies whether to retry, continue, terminalize, or require intervention, without creating a duplicate committed logical tool result.

Therefore persistence/reducer/replay semantics precede concrete provider work and public tools.

## Stop Conditions

- Stop if Agent-specific types must enter the shared `ww-store` semantic API to make progress.
- Stop if one creation/settlement transaction cannot be made consistent and no bounded idempotent repair protocol is defined.
- Stop if stream assembly can expose an executable tool call before complete argument validation/finalization.
- Stop if a crash window permits two committed model-visible results for one logical tool call.
- Stop if a non-replayable started effect can be silently re-executed.
- Stop if retry/budget counters depend on process-local state rather than durable history.
- Stop if `ww-agent-core` imports concrete transport, SQLite, filesystem, Flow, or WorkWeave Orchestration types.
- Stop if a Task becomes L-sized under the planning heuristic; split it before implementation.

## Rollback

Revert G003 implementation while preserving ADR-0003, Goal/Evaluation contracts, recorded fixtures, and review findings. G002 remains a valid shared runtime regardless of G003 outcome.
