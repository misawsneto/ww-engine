# Architecture Decision Records

Architecture Decision Records capture durable engine-design decisions that authorize or materially constrain Goal implementation.

Rules:

- Every Goal must reference at least one ADR before the Goal becomes `active`.
- Create the ADR while the Goal is `proposed`; change the ADR to `accepted` when the Goal is activated.
- If implementation changes an accepted architectural decision, supersede or amend the ADR before relying on the new direction.
- ADRs explain architecture and tradeoffs. Goal, Task, Question, Decision, Evaluation, Review, and Outcome semantics remain in their canonical bookkeeping records.
- `DECISIONS.md` is the compact workspace decision index; ADRs carry the implementation-grade rationale and consequences.

## Index

| ADR | Goal | Status | Decision |
| --- | --- | --- | --- |
| [ADR-0001](ADR-0001-g001-execution-architecture.md) | G001 | accepted | One Rust platform, sibling Agent/Flow kernels, Orchestration above |
| [ADR-0002](ADR-0002-g002-shared-runtime.md) | G002 | accepted | Semantically neutral durable runtime substrate on SQLite |
| [ADR-0003](ADR-0003-g003-thin-agent-kernel.md) | G003 | accepted | Durable provider-neutral Agent kernel on the shared runtime |
