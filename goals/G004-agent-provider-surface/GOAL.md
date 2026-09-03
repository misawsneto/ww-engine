# G004 — Agent Provider and Surface

## Statement

Expose the proven G003 Agent kernel through one concrete OpenAI-protocol provider adapter, one bounded read-only local tool, and first-class Rust SDK/CLI surfaces without changing kernel durability/recovery semantics.

## State

- proposed

## Architecture Decision Records

- `docs/adr/ADR-0004-g004-agent-provider-surface.md` — proposed; must be accepted before G004 activation.

## Success Criteria

- The OpenAI adapter passes the same normalized provider contract using deterministic recorded/local HTTP streaming fixtures and leaks no vendor type into the kernel.
- One bounded `fs.read` tool enforces workspace-root, canonical-path/symlink, UTF-8, and size/range policy before reading model-visible content.
- Rust SDK can start, cancel, inspect, and stream one Agent run through Agent/kernel projections without direct database access by callers.
- `ww agent run`, `ww agent inspect`, and `ww agent events` operate through the SDK and support machine-readable output.
- A local mock OpenAI-compatible streaming server can drive CLI `model → fs.read → model → terminal` end-to-end in mandatory CI without external credentials.
- Provider credentials and sensitive headers never enter durable normalized audit or CLI output.
- G004 adds no Flow/OWS semantics, write/process/network tools, sessions, compaction, multi-provider routing, TUI, or server control plane.

## Requirements

- G004 may extend provider/tool adapters and projections but must not weaken or fork G003 recovery rules.
- OpenAI transport must be cancellable and convert protocol events at the adapter boundary only.
- Mandatory CI must be credential-free; live OpenAI smoke is opt-in/non-blocking.
- `fs.read` must resolve/canonicalize the target and prove it remains within the configured workspace root before opening content.
- SDK/CLI inspection must expose normalized committed transcript/audit state, not raw chain-of-thought or secrets.
- CLI must not depend directly on `ww-store`, `ww-store-sqlite`, or rusqlite.

## Boundaries

- One OpenAI-compatible adapter only.
- One user-visible tool: `fs.read`.
- SDK and CLI only; TUI/server remain later.
- No write, patch, shell, process, arbitrary network, MCP, plugin, subagent, or A2A tools.
- No Flow/OWS or WorkWeave Orchestration semantics.

## Required Evaluations

- `OpenAI adapter conformance`.
- `Bounded filesystem read safety`.
- `Agent SDK and CLI surface conformance`.

See `EVALUATIONS.md`.

## Dependencies

- G003 — Durable Agent Kernel, including terminal review acceptance.
