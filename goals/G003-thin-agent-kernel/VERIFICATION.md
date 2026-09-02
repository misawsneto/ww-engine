# G003 Verification

## Required checks

- [ ] ADR-0003 is accepted before implementation is represented as active.
- [ ] `cargo fmt --all -- --check`
- [ ] Agent/common architecture-boundary checks
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] provider-neutral stream assembler finalizes text-only response
- [ ] provider-neutral stream assembler finalizes complete tool calls
- [ ] truncated/incomplete tool arguments cannot execute
- [ ] recorded provider completes model → tool → model round trip
- [ ] tool arguments are JSON-schema validated before policy/execution
- [ ] denied tool becomes a model-visible durable result without executing effect
- [ ] provider tool-call order equals model-visible tool-result order
- [ ] finalized assistant/tool/turn records reconstruct identical state after process restart
- [ ] stale/conflicting Agent writer fails without partial Agent/common audit mutation
- [ ] durable cancel request reaches an active provider stream cancellation token
- [ ] durable cancel request reaches an active tool cancellation token
- [ ] deadline, model-request, turn, and tool-call limits terminate deterministically
- [ ] token budget stops before the next model request when normalized usage crosses the configured limit
- [ ] replay-safe interrupted tool can resume without duplicate committed result
- [ ] non-replayable interrupted tool is not re-executed and yields `RequiresIntervention`
- [ ] interrupted model attempt remains auditable and retry creates a distinct attempt
- [ ] OpenAI adapter passes recorded provider-contract fixtures
- [ ] optional live provider smoke is not required for CI
- [ ] CLI uses SDK only; no direct SQLite/store dependency
- [ ] Agent crates have no Flow/OWS dependency
- [ ] WorkWeave Orchestration Goal/Task/Evaluation semantics do not appear in Agent kernel types

## Evidence

- To be recorded during G003.
