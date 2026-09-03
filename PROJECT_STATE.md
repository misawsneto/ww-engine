# Project State

## Current

- Product: WorkWeave Engine.
- Language: Rust.
- Active Goal: `G003 — Thin Agent Kernel`.
- G001 architecture baseline: achieved and accepted.
- Architecture thesis: one shared Rust runtime substrate with two sibling execution kernels added in later Goals.
- G002 is achieved after independent review acceptance on 2026-09-02; its governing ADR is `ADR-0002`.
- G003 is active and narrowed to the durable provider-neutral kernel/recovery proof under accepted `ADR-0003`.
- Proposed following Goal: `G004 — Agent Provider and Surface`; it adds the first OpenAI-protocol adapter, bounded `fs.read`, and Agent SDK/CLI under proposed `ADR-0004`.
- G004 activation requires G003 terminal acceptance and ADR-0004 transition from `proposed` to `accepted`.
- Deterministic OWS Flow kernel work follows as G005; restart-safe Flow → Agent integration follows as G006.

## Goal ADR rule

Every Goal must reference at least one ADR before activation. G001/G002 ADRs were backfilled when this rule was introduced; future Goals must create their ADRs while still proposed.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.
- Engine architecture baseline: `docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`.

## G002 achieved boundary

Executable acceptance is complete. Final reviewed implementation code head `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` is covered by CI run `33646651848` through evidence commit `bb2cb831fe42342afcfc93cf7e8757a9206c1947`. Format, architecture boundaries, clippy with warnings denied, full tests, real process-boundary lifecycle, cursor reconnect, optimistic conflict rejection, two-phase durable cancellation, projection consistency, and artifact dedupe all pass.

The architecture review found and resolved a cancellation/audit consistency defect before Agent work began. The project owner/user independently reviewed and accepted G002 on 2026-09-02. G002 is achieved.

## Planned implementation sequence

```text
G002 Shared Runtime                 achieved
  ↓
G003 Durable Agent Kernel           ACTIVE — recorded provider/tools; recovery first
  ↓
G004 Agent Provider and Surface     OpenAI protocol + fs.read + SDK/CLI
  ↓
G005 Deterministic OWS Flow Kernel
  ↓
G006 Flow → Agent integration
  ↓
G007 Full frozen OWS profile
  ↓
G008 Local product experience
```

## G003 progress

T001 activation/bookkeeping, T002 provider protocol/assembler, T003 durable history and recovery reducer, and T004 Agent SQLite persistence are complete and verified. T005 common/Agent transaction coordination is complete. T006 onward remain open.
