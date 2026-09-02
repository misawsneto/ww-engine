# G002 Reviews

## Implementation evidence review — 2026-09-02

State: ready for independent review.

The implementation evidence covers the Goal's executable boundary:

- lifecycle state and immutable event history are committed transactionally;
- invalid lifecycle transitions do not append events;
- stale optimistic writers are rejected without partial state or event mutation;
- execution projection survives SQLite reopen;
- the `ww` CLI proves durability across separate operating-system processes using the same database;
- cursor-based event inspection reconnects without duplicates;
- durable cancellation signals a registered local `CancellationToken`;
- artifact content is SHA-256 addressed and deduplicated;
- CLI source has no direct SQLite/store dependency and operates through `ww-sdk`;
- CI rejects Agent/provider/tool-loop and Flow/OWS/token concepts in the common runtime crates.

## Architecture review finding — cancellation and projection consistency

A separate architecture review pass found one blocking lifecycle defect before G003:

1. `settle_cancelled` could terminalize an execution without a prior durable `request_cancel` event.
2. The cancellation reason supplied at settlement could diverge from the reason persisted by the durable cancellation request.
3. The event reducer did not project terminal `result_artifact` and `error`, so `inspect()` could miss disagreement between the durable row and event history for terminal payloads.

Fix applied and verified:

- cancellation is explicitly two-phase: `request_cancel` persists intent/reason, then `settle_cancelled` may terminalize only when `cancel_requested == true`;
- the terminal cancellation event reuses the persisted cancellation reason rather than accepting a second reason;
- the reducer now projects `result_artifact` and `error`, and `inspect()` compares them with the current execution row;
- a regression test rejects cancellation settlement without a prior durable request;
- the cancellation reducer property test requires at least one request before a terminal cancellation event.

The reviewed implementation code is `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc`. Permanent CI run `33646651848`, on evidence head `bb2cb831fe42342afcfc93cf7e8757a9206c1947`, passed format, architecture-boundary checks, clippy with warnings denied, and the complete workspace test suite.

## Review conclusion

No additional blocking implementation issue was identified by this review pass. The common substrate remains semantically neutral: it knows execution identity/lifecycle, durable events, storage, cancellation and artifacts, but not Agent/provider/tool-loop or Flow/OWS/token concepts.

## Terminal review status

T010 remains open. This review was performed in the same assistant/workstream and is therefore not represented as an independent review. An independent reviewer must still inspect lifecycle/transaction correctness, restart reconstruction, cancellation, artifact durability, SDK/CLI boundaries, and the size/semantic neutrality of the shared substrate before G002 is marked complete.
