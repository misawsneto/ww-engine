# G003 Engineering Handoff — 2026-09-03

## Purpose

This record separates engineering continuation from strategic architecture discussion. The engineering agent may implement within the active G003 Goal and accepted ADR-0003. Material architecture changes, scope changes, or new durable decisions must be raised for strategic review and recorded through the Goal/ADR bookkeeping before implementation relies on them.

## Authoritative engineering branch

- Branch: `g003-engineering`
- Verified engineering basis: `69f4ab7ecbed731d40a695dafcf487d62645b695`
- G002: achieved and owner-approved.
- G003: active under accepted ADR-0003.
- T001–T005: complete.
- Next tasks: T006 and T007; T008 depends on T005 + T006 + T007.
- G004: proposed only; ADR-0004 remains proposed. Do not implement G004 capabilities inside G003.

## Verified implementation boundary

The consolidation basis passed on Rust 1.98.0:

```text
cargo fmt --all -- --check                  pass
five architecture-boundary checks           pass
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings   pass
cargo test --workspace --all-features --locked                                  44/44 pass
```

Permanent CI must remain at least as strong as this gate. The earlier temporary verifier omitted rustfmt and did not use `--locked`; that weaker gate is retired.

## G003 architectural constraints

- `ww-agent-core` owns no concrete HTTP/provider transport, SQLite implementation, filesystem capability, CLI/TUI/server, Flow/OWS, or WorkWeave Orchestration semantics.
- Provider contracts remain normalized and provider-neutral.
- Agent persistence remains logically separate from common runtime persistence even when one SQLite file is used.
- Agent DTOs do not enter the shared `ww-store` semantic API.
- G003 uses RecordedProvider and test-only tools; OpenAI, bounded `fs.read`, Agent SDK, and `ww agent` CLI belong to G004.
- Tool/recovery semantics must prevent duplicate committed logical results and must never silently replay ambiguous non-replayable effects.
- Flow/OWS work does not enter G003.

## Next engineering sequence

### T006 — RecordedProvider conformance

Implement deterministic provider fixtures over the normalized T002 protocol covering text, tool calls, usage, provider failure, cancellation, truncation, and interrupted attempts. No network client.

### T007 — Tool contract, validation, policy, replay fixtures

Implement `test.echo` and synthetic `test.unsafe_once`; complete JSON/schema validation before policy/effect execution; persist replay classification and effect-relevant digests. No filesystem/process/network tool.

### T008 — Functional kernel integration

Only after T006 and T007 pass independently, integrate the real sequential model → tool → model loop. Preserve provider source order in model-visible tool results.

## Escalate for strategic review before changing

- accepted ADR-0003 boundaries;
- G003/G004 split;
- replay/intervention model;
- shared-store semantic ownership;
- sequential-vs-parallel G003 tool semantics;
- provider/network/filesystem scope;
- Flow/OWS or Orchestration coupling;
- any new crate/public API that materially changes the C3/C4 architecture.

## Merge discipline

Do not merge G003 to `main` merely because intermediate tasks pass. G003 closes only after T012 required EvaluationRuns and terminal recovery/architecture review. Intermediate engineering commits should remain on the G003 engineering branch and keep bookkeeping current.
