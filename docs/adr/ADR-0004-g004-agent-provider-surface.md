# ADR-0004 — First concrete Agent provider and SDK/CLI surface

- Status: proposed
- Goal: G004 — Agent Provider and Surface
- Recorded: 2026-09-02
- Activation condition: G003 terminal review accepted and this ADR changed to `accepted`.

## Context

G003 is intentionally narrowed to prove the durable probabilistic kernel using deterministic provider/tool fixtures. The next proof is substitution: attach one real network protocol adapter and one bounded local effect without changing kernel durability/recovery semantics, then expose the result through the first public Rust SDK and CLI surfaces.

The first concrete adapter is OpenAI-compatible because it exercises streamed text, fragmented tool-call arguments, usage, errors, cancellation, authentication, and endpoint configuration while remaining testable against a local mock HTTP/SSE server. The first local tool is read-only `fs.read`; write/process/network capabilities remain deferred until later policy/sandbox work.

## Decision

1. Create `ww-agent-openai` as an adapter to the accepted G003 `ModelProvider` contract; vendor types do not cross the adapter boundary.
2. Mandatory conformance uses deterministic local HTTP/SSE fixtures. Live credentialed OpenAI smoke is optional and non-blocking.
3. Resolve credentials at request time; never persist API keys or authorization headers in normalized Agent state/audit/CLI output.
4. Create `ww-agent-tools-local` with only bounded `fs.read` in G004.
5. `fs.read` canonicalizes workspace root and target, rejects path/symlink escape before content return, rejects directories/non-UTF-8 in G004, and enforces line and byte bounds.
6. Extend `ww-sdk` with Agent start/cancel/inspect/event projection methods; callers receive normalized projections, not raw database/provider payloads.
7. Extend `ww-cli` with `ww agent run`, `ww agent inspect`, and `ww agent events`; CLI composes SDK and has no direct store/SQLite dependency.
8. Prove CLI end-to-end in CI with a local mock OpenAI-compatible server and separate OS processes over the same durable SQLite database.
9. Do not change G003 replay/recovery rules in G004. Any required semantic change stops the Goal and amends/supersedes ADR-0003 first.
10. Defer second providers, fallback/routing, write/patch/bash/process/arbitrary-network/MCP/plugin tools, TUI/server, sessions, steering, and compaction.

## Consequences

- G003 remains a deterministic durability proof; G004 becomes the network/security/product-surface proof.
- Provider correctness and filesystem containment can be reviewed independently from core recovery logic.
- CLI/SDK become real first-class Agent surfaces before Flow work begins, without requiring a live external credential in CI.
- Future provider/tool expansion reuses accepted normalized seams.

## Rejected alternatives

- Keep concrete provider and CLI in G003.
- Require a live OpenAI key for acceptance.
- Add read/write/bash together as the first local tool set.
- Let CLI query SQLite directly for convenience.
- Add provider fallback/routing before one adapter is proven.

## Evidence basis

- Accepted/proposed G003 normalized provider/tool/recovery contracts.
- Pi provider abstraction and coding-agent model runtime as source references, without importing its larger session/product layer.
- G002 SDK/CLI store-boundary and process-restart evidence.
