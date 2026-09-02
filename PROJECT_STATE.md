# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G002 — Shared Runtime Walking Skeleton` (ready for independent review).
- G001 architecture baseline: accepted.
- Architecture thesis: one shared Rust runtime substrate with two sibling execution kernels added in later Goals.
- G002 boundary: shared execution identity, lifecycle, durable audit/events, SQLite, cancellation, artifacts, SDK inspection/event streaming, and CLI only.
- Agent kernel work begins after G002.
- Flow kernel work begins after the thin Agent Goal; the first integrated milestone remains restart-safe `Flow → Agent → Tool → Flow`.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.

## G002 acceptance boundary

Executable acceptance evidence is complete at CI run `33644225518`: synthetic execution lifecycle, real process-boundary restart/inspection, ordered event reduction, cursor reconnect, optimistic conflict rejection, durable cancellation, artifact dedupe, and architecture-boundary checks all pass. G002 remains active only for T010 independent architecture/implementation review.
