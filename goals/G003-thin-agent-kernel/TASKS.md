# G003 Tasks

| Task | State | Acceptance | Dependencies |
| --- | --- | --- | --- |
| T001 Accept G002 review and activate G003 | complete | G002 achieved; ADR-0003 accepted; G003 state active; project state points to G003 | G002 T010 |
| T002 Define provider-neutral protocol and stream assembler | complete | immutable provider/model/message/tool-call types exist; pure assembler passes valid text/tool streams and fails closed on malformed/truncated/duplicate terminal streams | T001 |
| T003 Define Agent entries, operational records, and recovery reducer | complete | immutable context-entry/attempt vocabulary reconstructs deterministic `AgentRecoveryState`; impossible references/order reject with typed corruption errors | T001 |
| T004 Implement Agent SQLite persistence and reconstruction | complete | Agent schema remains Agent-owned; append/query/reopen/process-restart reconstruction works; stale Agent writers reject without partial Agent mutation | T003 |
| T005 Prove common/Agent SQLite transaction coordination | complete | common execution + Agent creation/link commit atomically without Agent DTOs in shared `ww-store`; injected failure leaves neither half committed; terminal repair remains T009 | T004 |
| T006 Implement recorded provider and provider conformance fixtures | open | deterministic fixtures cover text, tool calls, usage, failure, cancellation, truncation, and interrupted attempts through the normalized provider contract | T002 |
| T007 Implement tool contract, schema validation, policy, and replay fixtures | open | deterministic replay-safe tool and synthetic non-replayable fixture validate before execution; denial is model-visible without effect; replay policy is durable | T001 |
| T008 Implement functional recorded-provider model → tool → model kernel | open | real kernel completes text-only and one-tool round trips; provider call order equals model-visible tool-result order; loop imports no transport/SQLite/Flow types | T005, T006, T007 |
| T009 Integrate G002 lifecycle and durable cancellation | open | one Agent run maps to one common execution; start/terminal repair is idempotent; durable cancel reaches active provider/tool token and settles consistently | T008 |
| T010 Implement durable deadlines and execution budgets | open | deadline/model-request/turn/tool-call/token limits derive from durable state, reserve before attempts, and terminate deterministically without launching work beyond the limit | T009 |
| T011 Prove crash/restart and ambiguous-effect recovery matrix | open | process-restart tests cover every specified fault boundary; replay-safe retry creates a new audited attempt without duplicate logical result; non-replayable ambiguity yields `RequiresIntervention` | T010 |
| T012 Record required EvaluationRuns and perform G003 recovery/architecture review | open | required Evaluations pass on exact code state; fmt/boundary/clippy/tests pass; review finds no blocking provider leakage, unsafe replay, Agent/store ownership leak, or Flow/Orchestration coupling | T011 |
