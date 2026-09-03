# G004 Verification

## Required checks

- [ ] ADR-0004 accepted before G004 activation.
- [ ] `cargo fmt --all -- --check`
- [ ] provider/tool/SDK/CLI dependency-boundary checks
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`

### OpenAI adapter

- [ ] text-only SSE fixture conforms
- [ ] fragmented tool-call JSON fixture conforms
- [ ] sequential multiple tool calls conform
- [ ] normalized usage conforms
- [ ] provider error and disconnect fail closed
- [ ] cancellation stops active transport
- [ ] truncated/incomplete tool call cannot execute
- [ ] unsupported event shape returns typed provider error
- [ ] credentials/authorization headers absent from durable audit and test snapshots

### `fs.read`

- [ ] canonical in-root file reads
- [ ] `..`/absolute outside-root access rejects
- [ ] symlink escape rejects
- [ ] directory rejects
- [ ] non-UTF-8 rejects
- [ ] line/range and total-byte caps apply deterministically
- [ ] cancellation before/during read returns cancelled result without write/effect

### SDK/CLI

- [ ] SDK starts, cancels, inspects, and streams one Agent run without caller DB access
- [ ] Agent inspection reconstructs committed transcript/tool/usage/result projection
- [ ] CLI source has no `ww-store`, `ww-store-sqlite`, or rusqlite dependency
- [ ] local mock OpenAI server drives separate-process CLI `model → fs.read → model → terminal`
- [ ] new CLI process inspects the same durable run
- [ ] event cursor resumes without duplicates
- [ ] JSON/JSONL output contains no secrets/raw auth headers

### Architecture boundaries

- [ ] G003 kernel crates remain unchanged in provider-specific semantics
- [ ] no Flow/OWS or WorkWeave Orchestration semantic dependency is introduced
- [ ] no write/process/arbitrary-network/MCP/plugin tool exists
- [ ] no TUI/server/session/compaction work enters G004

## Required Evaluations

All checks in `EVALUATIONS.md` required for G004 closure must have current passing EvaluationRuns on the reviewed final code basis.

## Evidence

- To be recorded during G004.
