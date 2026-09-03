# Specification

## Title

- Durable storage evolution and recovery contracts

## State

- draft

## Contract Areas

### Durable format ownership

- Each component owns its schema migrations and durable payload DTOs.
- Shared storage exposes physical transaction and persistence mechanics, not Agent-, Flow-, or Orchestration-specific records.
- Every evolvable durable payload carries an explicit format version with a documented compatibility policy.

### Migration compatibility

- Known-old-state fixtures are committed inputs, not generated from current schemas during the test.
- Upgrade is deterministic, transactional, and idempotent.
- Unknown future schema or payload versions reject before partial mutation.
- Reopen after a successful migration reconstructs the same semantic projection as a fresh current-version store.

### Ambiguous acknowledgement

- Create/ensure accepts a stable caller-owned idempotency identity or equivalent durable key.
- Retrying after the commit succeeds but acknowledgement is lost returns the already-created logical resource.
- Conflicting reuse of an idempotency identity fails explicitly rather than aliasing different requests.

### Failure classification

The storage boundary distinguishes at least:

- retryable availability failure;
- optimistic/concurrency conflict;
- corrupt durable state;
- unsupported future version;
- permanent input or invariant failure.

Concrete SQLite error codes remain adapter details unless needed to produce this stable classification.

### Physical SQLite reuse

- Shared mechanics may include connection configuration, transactional execution, migration coordination, busy handling, and fault injection.
- Component schema definitions, migration contents, row mappings, and durable DTOs remain with the owning adapter.
- Cross-adapter tests prove the common mechanics without coupling the semantic state machines.

## Explicit Exclusions

- Agent provider, tool, loop, lifecycle, cancellation, or limit semantics;
- Flow IR, OWS validation, or interpretation;
- WorkWeave Orchestration concepts;
- SDK, CLI, TUI, filesystem, network, or product surfaces.
