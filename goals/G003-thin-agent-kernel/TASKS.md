# G003 Tasks

| Task | State | Acceptance |
| --- | --- | --- |
| T001 Accept G002 review and activate G003 ADR/bookkeeping | complete | G002 achieved; ADR-0003 accepted; project state points to G003 |
| T002 Implement provider-neutral model/message/stream contracts | complete | recorded event streams assemble immutable finalized responses; unsupported/truncated tool-call data fails closed |
| T003 Implement Agent durable history model and recovery reducer | complete | Agent run/entry/record schema reconstructs deterministic `AgentRecoveryState`; corrupt histories reject |
| T004 Implement Agent-owned SQLite persistence | complete | append-only entries/records, optimistic version checks, rollback, SQLite reopen and OS process-restart reconstruction |
| T005 Implement common/Agent SQLite transaction seam | complete | common execution, Agent run, and durable link commit in one transaction; injected mid-write failure rolls back both models |
| T006 Implement recorded provider and provider conformance harness | open | deterministic text-only and tool-call fixtures exercise success, failure, cancellation, usage, and interrupted attempts |
| T007 Implement tool contracts, schema validation, policy, and replay classes | open | deterministic test tool and test-only non-replayable fixture pass validation/policy/recovery tests |
| T008 Implement functional model → tool → model kernel | open | recorded provider completes a tool round trip; results preserve provider call order; no session/Flow coupling |
| T009 Integrate common execution lifecycle and cancellation | open | Agent and common execution settle consistently; cancellation reaches active provider/tool work |
| T010 Implement durable Agent limits | open | deadline, maximum model requests, turns, and tool calls are enforced; token budget enforced when normalized usage is available |
| T011 Implement crash/restart and ambiguous-effect recovery matrix | open | replay-safe incomplete work resumes without duplicate committed results; non-replayable incomplete tool becomes `RequiresIntervention` |
| T012 G003 evaluations and terminal architecture review | open | fmt/boundary/clippy/tests pass; reviewer confirms no Flow dependency/provider leakage/unsafe replay; Goal exit criteria accepted |
