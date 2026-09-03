# Plan

## Title

- Prove the durable probabilistic kernel before adding network and product surfaces

## Version

- `v2`
- Effective: 2026-09-03
- Governing sequencing decision: D018
- v2 preserves completed T001–T006 and inserts a durability/hygiene gate before the previously planned tool-contract T007.
- Historical records written before v2 keep their original task numbering. In v2, former open T007–T012 become T012–T017.

## State

- active

## Slicing Strategy

Risk-first + contract-first + durability-cleanup-first.

G003 proves failure-sensitive contracts independently, closes implementation debt that could become part of the durable format, and only then integrates tools and the functional loop:

```text
provider protocol + assembler ──→ recorded provider ───────────────┐
                                                                   │
Agent durable model + reducer ─→ SQLite persistence/coordination ──┼─→ durability/hygiene gate
                                                                   │          ↓
                                                                   │   tool policy/replay contract
                                                                   │          ↓
                                                                   └────→ functional kernel
                                                                              ↓
                                                                    lifecycle + limits/cancel
                                                                              ↓
                                                                       fault/restart matrix
                                                                              ↓
                                                                         final review
```

The concrete OpenAI adapter, bounded `fs.read`, SDK projection, and CLI remain in G004. D018 does not reopen the accepted architecture; it strengthens ADR-0003 by making schema evolution, idempotency, durable ownership, and recovery-relevant error boundaries explicit before additional durable semantics are introduced.

## Strategy

1. Close G002 independent review and accept ADR-0003 before activating G003. **Complete.**
2. Define normalized provider events and a pure stream assembler first; no HTTP client exists in this Goal. **Complete.**
3. Define immutable context entries, operational attempt records, and a fail-closed recovery reducer before implementing the loop. **Complete.**
4. Prove Agent-owned SQLite append/query/reconstruction first, then prove the narrower common/Agent transaction coordination seam separately. **Complete.**
5. Build a deterministic recorded provider conformance harness that exercises text, tool calls, failure, cancellation, truncation, and interrupted attempts. **Complete.**
6. Execute the D018 durability/hygiene gate before tool semantics expand the durable model:
   1. add component-owned SQLite migration/version tracking and explicit durable payload/schema versions;
   2. make coordinated creation idempotently retryable after a committed-but-unacknowledged success and introduce recovery-relevant typed store errors;
   3. move provider-to-durable conversion behind an explicit core-owned boundary and establish one authoritative tool-argument representation before validation/hash/replay logic;
   4. centralize provider stream consumption/finalization so every EOF/terminal path has one typed interpretation;
   5. consolidate only physical SQLite plumbing, strengthen structural dependency checks, isolate test-only surfaces, and synchronize canonical current-state documents.
7. Define the tool contract with schema validation, policy decision, replay classification, and deterministic/synthetic fixtures only after the cleanup gate is complete.
8. Implement the smallest functional loop over those ports, sequential tool execution only.
9. Bind the loop to G002 lifecycle/cancellation first; then implement deadline and budget accounting as a separate bounded slice derived from durable state.
10. Run the fault matrix by killing/reopening around creation/model/tool/turn/terminal boundaries, including retry after committed-but-unacknowledged creation.
11. Perform recovery/architecture review only after every required Evaluation has a current passing run on the final code state.

## Plan v2 Cleanup Gate

All five cleanup tasks T007–T011 must be complete before T012 begins.

### T007 — durable schema and migration evolution

- Introduce component-owned migration tracking suitable for one physical SQLite database shared by common runtime, Agent, and future Flow adapters.
- Add explicit version information to Agent durable entry/record payloads and define version-aware decode behavior.
- Define the version contract for persisted Agent configuration snapshots.
- Add upgrade/reopen tests from the existing v1 schema/state.

### T008 — idempotent coordinated creation and typed storage failures

- Make create/ensure semantics safe when the transaction committed but the caller did not receive the success response.
- Retry with the same execution/run identity must return the matching committed aggregate/link rather than create duplicates or surface an ambiguous uniqueness error.
- Distinguish invalid command/input, already-existing semantic conflict, optimistic conflict, persisted corruption, transient backend failure, and permanent backend failure where recovery behavior differs.
- Add fault/retry tests around the post-commit acknowledgement boundary.

### T009 — durable ownership and canonical tool arguments

- `ww-agent-core` owns the serialized durable Agent vocabulary; normalized provider types are converted explicitly at the boundary instead of becoming the disk-format owner by re-export/import convenience.
- Establish one authoritative executable tool-argument value. Preserve raw provider JSON only when it has an explicit audit/diagnostic purpose.
- Prevent raw/parsed argument divergence before T012 adds validation, policy, digests, and replay semantics.

### T010 — one provider-stream finalization path

- Add one production stream-consumption/finalization path used by the future kernel and the recorded-provider conformance tests.
- Normal completion, provider failure, cancellation, malformed streams, and EOF without a terminal event must each produce one typed interpretation.
- Callers must not be able to accidentally treat interrupted EOF as successful absence of a response.

### T011 — physical-backend and repository hygiene

- Consolidate repeated SQLite connection/configuration mechanics behind a physical backend utility without moving Agent DTOs into shared semantic store contracts.
- Add structural crate/dependency checks (for example via Cargo metadata) alongside the existing semantic grep guards.
- Make process-restart fixtures and RecordedProvider/testkit exposure explicitly test/development oriented rather than accidental product surface.
- Synchronize `PROJECT_STATE.md`, README/current guidance, and Goal records so completed-task state is canonical and dated recall documents remain non-authoritative history.

## High-Risk Proof

The highest-risk proposition remains:

> After a crash at any effect ambiguity boundary, durable Agent history deterministically identifies whether to retry, continue, terminalize, or require intervention, without creating a duplicate committed logical tool result.

Plan v2 adds one prerequisite to that proof: the durable format and creation boundary themselves must have explicit evolution and idempotency semantics before tool/replay behavior relies on them.

## Stop Conditions

- Stop if Agent-specific types must enter the shared `ww-store` semantic API to make progress.
- Stop if durable Agent payloads are changed without an explicit version/migration path.
- Stop if one creation/settlement transaction cannot be made consistent and no bounded idempotent repair protocol is defined.
- Stop if the same coordinated create identity can produce an ambiguous success/duplicate outcome after retry.
- Stop if raw and parsed tool arguments can diverge at a durable or executable boundary.
- Stop if stream assembly can expose an executable tool call before complete argument validation/finalization.
- Stop if a caller can drain a provider stream without receiving a typed terminal/interrupted outcome.
- Stop if a crash window permits two committed model-visible results for one logical tool call.
- Stop if a non-replayable started effect can be silently re-executed.
- Stop if retry/budget counters depend on process-local state rather than durable history.
- Stop if `ww-agent-core` imports concrete transport, SQLite, filesystem, Flow, or WorkWeave Orchestration types.
- Stop if a Task becomes L-sized under the planning heuristic; split it before implementation.

## Rollback

Revert G003 implementation while preserving ADR-0003, D018, Goal/Evaluation contracts, recorded fixtures, and review findings. G002 remains a valid shared runtime regardless of G003 outcome.
