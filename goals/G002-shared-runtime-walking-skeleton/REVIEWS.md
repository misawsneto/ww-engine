# G002 Reviews

## Implementation evidence review — 2026-09-02

State: ready for independent review after reviewed lifecycle fix passes CI.

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

Previous evidence: CI run `33644225518` at code head `c19c4c6a6c071190ddbcef23299fd10aecabb0a4` passed format, architecture boundaries, clippy with warnings denied, and the complete workspace test suite.

## Architecture review finding — cancellation and projection consistency

A separate architecture review pass found one blocking lifecycle defect before G003:

1. `settle_cancelled` could terminalize an execution without a prior durable `request_cancel` event.
2. The cancellation reason supplied at settlement could diverge from the reason persisted by the durable cancellation request.
3. The event reducer did not project terminal `result_artifact` and `error`, so `inspect()` could miss disagreement between the durable row and event history for terminal payloads.

Fix applied:

- cancellation is explicitly two-phase: `request_cancel` persists intent/reason, then `settle_cancelled` may terminalize only when `cancel_requested == true`;
- the terminal cancellation event reuses the persisted cancellation reason rather than accepting a second reason;
- the reducer now projects `result_artifact` and `error`, and `inspect()` compares them with the current execution row;
- a regression test rejects cancellation settlement without a prior durable request;
- the cancellation reducer property test now generates at least one request before a terminal cancellation event.

Reviewed code head before rustfmt: `05196726b0972fb3506894a9c1b24118e8326d4e`; GitHub Actions applied rustfmt as `16fb32f43769a72b96e513c8892b615b79e57d18`. A sandbox-transfer typo in the cancellation return path was then corrected as `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc`. Permanent CI on this reviewed code is the remaining executable check for the finding.

## Terminal review status

T010 remains open. This implementation/architecture review is not represented as an independent review. An independent reviewer must still inspect lifecycle/transaction correctness, restart reconstruction, cancellation, artifact durability, SDK/CLI boundaries, and the size/semantic neutrality of the shared substrate before G002 is marked complete.
