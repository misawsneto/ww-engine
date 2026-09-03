# G009 Reviews

No implementation review exists because G009 is proposed only. Activation order must be decided after G003 from concrete evidence.

## Planned review focus

- component ownership of schemas, migrations, and durable payload DTOs;
- compatibility and fail-closed behavior across known-old and future versions;
- idempotent create/ensure behavior after uncertain acknowledgement;
- stable recovery-oriented error classification;
- physical SQLite reuse without semantic ownership leakage;
- cross-adapter conformance and fault injection;
- absence of Agent, Flow, or WorkWeave Orchestration semantic changes;
- absence of provider, tool, SDK, CLI, TUI, filesystem, or network scope.
