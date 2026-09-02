# G001 — WorkWeave Execution Architecture

## Goal

Establish an implementable architecture for one Rust WorkWeave execution platform containing a probabilistic Agent kernel and deterministic OWS Flow kernel on a shared operational substrate.

## State

- achieved

## Architecture Decision Records

- `docs/adr/ADR-0001-g001-execution-architecture.md` — accepted.

## Success criteria

- The architecture defines C1–C4 boundaries for the engine.
- Pi Agent and Pi future Harness are analyzed separately and source-pinned.
- OWS remains Flow-definition authority.
- LangGraph contributes runtime mechanics without introducing another canonical workflow language.
- Agent and Flow share infrastructure but not one execution state machine.
- Flow-to-Agent execution uses an explicit local/remote-compatible A2A seam.
- The architecture defines persistence, audit, policy, deployment, SDK, CLI and TUI boundaries.
- A bounded G002 falsification spike is defined.

## Boundaries

- No production Rust implementation in G001.
- No new WorkWeave Orchestration Domain/Flow semantics.
- No provider, plugin, sandbox or OWS breadth work beyond what is needed to define architecture.
