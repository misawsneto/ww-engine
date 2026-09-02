# G003 Tasks

| Task | State | Acceptance |
| --- | --- | --- |
| T001 Accept G002 review and activate G003 ADR/bookkeeping | open | G002 achieved; ADR-0003 accepted; project state points to G003 |
| T002 Implement provider-neutral model/message/stream contracts | open | recorded event streams assemble immutable finalized responses; unsupported/truncated tool-call data fails closed |
| T003 Implement Agent durable model and recovery reducer | open | Agent run/entry/record schema reconstructs deterministic `AgentRecoveryState`; corrupt histories reject |
| T004 Implement recorded provider and provider conformance harness | open | deterministic text-only and tool-call fixtures exercise success, failure, cancellation, usage, and interrupted attempts |
| T005 Implement tool contracts, schema validation, policy, and replay classes | open | `fs.read`, deterministic test tool, and test-only non-replayable fixture pass validation/policy/recovery tests |
| T006 Implement functional model → tool → model kernel | open | recorded provider completes a tool round trip; results preserve provider call order; no session/Flow coupling |
| T007 Integrate common execution lifecycle, cancellation, deadlines, and budgets | open | Agent and common execution settle consistently; cancellation reaches active provider/tool; request/turn/tool/deadline limits are tested |
| T008 Implement crash/restart and ambiguous-effect recovery tests | open | replay-safe incomplete work resumes without duplicate committed results; non-replayable incomplete tool becomes `RequiresIntervention` |
| T009 Implement first concrete OpenAI adapter | open | adapter satisfies provider conformance with recorded transport fixtures; no vendor type leaks; optional live smoke is non-blocking |
| T010 Extend Rust SDK and `ww agent` CLI | open | one bounded run and inspection path use SDK only and support machine-readable output |
| T011 G003 verification and architecture review | open | fmt/boundary/clippy/tests pass; reviewer confirms no Flow dependency/provider leakage/unsafe replay; Goal exit criteria accepted |
