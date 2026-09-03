# G009 Evaluations

All Evaluations are draft while G009 remains proposed.

## Durable format evolution
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G009 — Durable Storage Evolution and Recovery Hardening`

### Checks

#### compatibility-matrix
- Covers: `schema and payload evolution`
- Subjects: `component-owned durable adapters and committed fixtures`
- Criteria: `Known-old states upgrade transactionally and reconstruct the current semantic projection; future versions fail closed without partial mutation.`
- Procedure: `Open committed version fixtures, migrate/reopen them, and inject unsupported schema and payload versions at each owned boundary.`
- Expected: `Supported upgrades are deterministic and idempotent; unsupported versions make no mutation.`

## Ambiguous acknowledgement recovery
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G009 — Durable Storage Evolution and Recovery Hardening`

### Checks

#### create-ensure-retry
- Covers: `committed-but-unacknowledged creation`
- Subjects: `component create/ensure APIs`
- Criteria: `A retry after acknowledgement loss returns one logical durable resource and rejects conflicting identity reuse.`
- Procedure: `Fault after commit but before acknowledgement, reopen where applicable, and retry the same and a conflicting request.`
- Expected: `The same request converges on one identity; conflicting reuse fails explicitly.`

## Storage adapter conformance
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G009 — Durable Storage Evolution and Recovery Hardening`

### Checks

#### shared-mechanics-boundary
- Covers: `physical SQLite reuse and failure taxonomy`
- Subjects: `shared mechanics plus component-owned adapters`
- Criteria: `Adapters share only physical mechanics, expose stable recovery classes, and retain ownership of schemas and durable DTOs.`
- Procedure: `Run migration, retry, reopen, concurrency, injected-failure, and dependency-boundary fixtures across every participating adapter.`
- Expected: `All adapters satisfy one mechanics contract without semantic type leakage or partial durable mutation.`
