# Architecture Decision Records

ADRs capture Goal-level architecture choices that must remain reviewable independently of implementation detail.

## Policy

- Every Goal must reference at least one ADR before the Goal becomes active.
- Create the ADR while the Goal is proposed.
- Change the governing ADR to `accepted` when the Goal is activated.
- If implementation discovers a material architecture change, amend the accepted ADR only for clarifying detail that preserves the same decision. Otherwise create a new ADR that supersedes it before relying on the new direction.
- Goal/Task bookkeeping does not replace ADRs; ADRs do not replace Goal success criteria or Task acceptance criteria.
- G001 and G002 were backfilled when this policy was introduced.

## Index

| ADR | Goal | Status | Decision |
| --- | --- | --- | --- |
| [ADR-0001](ADR-0001-g001-execution-architecture.md) | G001 | accepted | One Rust execution platform with sibling Agent and Flow kernels over a shared substrate |
| [ADR-0002](ADR-0002-g002-shared-runtime.md) | G002 | accepted | Semantically neutral SQLite-first shared runtime walking skeleton |
| [ADR-0003](ADR-0003-g003-thin-agent-kernel.md) | G003 | accepted | Durable provider-neutral Agent kernel proved with recorded provider/tool fixtures |
| [ADR-0004](ADR-0004-g004-agent-provider-surface.md) | G004 | proposed | First OpenAI-protocol adapter, bounded `fs.read`, and Agent SDK/CLI surface |
