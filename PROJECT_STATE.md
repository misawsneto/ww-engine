# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G002 — Shared Runtime Walking Skeleton` (executable acceptance complete; independent review pending).
- G001 architecture baseline: accepted.
- Architecture thesis: one shared Rust runtime substrate with two sibling execution kernels added in later Goals.
- G002 boundary: shared execution identity, lifecycle, durable audit/events, SQLite, cancellation, artifacts, SDK inspection/event streaming, and CLI only.
- Agent kernel work begins after G002 terminal acceptance.
- Flow kernel work begins after the thin Agent Goal; the first integrated milestone remains restart-safe `Flow → Agent → Tool → Flow`.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.

## G002 acceptance boundary

Executable acceptance is complete. Final reviewed implementation code head `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` is covered by CI run `33646651848` through evidence commit `bb2cb831fe42342afcfc93cf7e8757a9206c1947`. Format, architecture boundaries, clippy with warnings denied, full tests, real process-boundary lifecycle, cursor reconnect, optimistic conflict rejection, two-phase durable cancellation, projection consistency, and artifact dedupe all pass.

The architecture review found and resolved a cancellation/audit consistency defect before Agent work began. G002 remains active only for T010 independent architecture/implementation review. Do not begin G003 until T010 is accepted or that governance requirement is explicitly changed.
