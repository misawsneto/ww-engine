# G009 — Durable Storage Evolution and Recovery Hardening

## Statement

Prove that WorkWeave's durable execution data and embedded persistence can evolve across schema/payload versions and ambiguous storage boundaries without changing Agent, Flow, or Orchestration semantics.

## State

- proposed

## Architecture Decision Records

- None yet. A governing ADR must be proposed and accepted before activation.

## Success Criteria

- Durable payloads have explicit, component-owned version evolution and known-old-state upgrade fixtures.
- Unknown future schema or payload versions fail closed without partial mutation.
- Create/ensure operations are idempotent after committed-but-unacknowledged outcomes.
- Store failures expose a recovery-oriented classification without leaking backend mechanics into semantic kernels.
- Physical SQLite mechanics are reusable by engine persistence adapters while schema and durable DTO ownership remain component-local.
- Cross-adapter conformance tests prove the storage contracts against migration, retry, reopen, and ambiguous acknowledgement cases.
- Agent, Flow, and WorkWeave Orchestration semantics remain unchanged.

## Boundaries

In scope:

- versioned durable payload evolution;
- component-owned schema migrations;
- known-old-state upgrade fixtures;
- future-version fail-closed behavior;
- idempotent create/ensure after uncertain acknowledgement;
- recovery-oriented store error classification;
- physical SQLite mechanics reusable by engine adapters;
- storage contract and conformance tests.

Out of scope:

- Agent tool semantics and provider protocol;
- Agent functional loop;
- Flow interpretation and OWS;
- SDK, CLI, or TUI;
- filesystem or network capabilities;
- WorkWeave Orchestration semantics.

## Required Evaluations

- `Durable format evolution`.
- `Ambiguous acknowledgement recovery`.
- `Storage adapter conformance`.

See `EVALUATIONS.md`.

## Dependencies

- None while proposed. Activation and execution order must be decided after G003 from concrete evidence.
- G009 is not automatically a prerequisite for G004 or any already reserved Goal.
