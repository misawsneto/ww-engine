# Warnings

1. Do not port Pi package-for-package; extract contracts and runtime behavior instead.
2. Do not describe Pi Harness scaffold behavior as current Pi production behavior.
3. Do not rebuild LangGraph's graph DSL beneath OWS.
4. Do not let a shared runtime become a shared Agent/Flow state machine.
5. Do not treat workflow context, model context, or execution audit as WorkWeave semantic Domain truth.
6. Do not turn every model/tool event into a Flow transition or orchestration event.
7. Do not make OpenTelemetry the only durable audit record.
8. Do not prematurely build remote multi-agent, plugin, sandbox and provider breadth before the local execution contracts are proven.
9. Do not duplicate the canonical WorkWeave Orchestration v0.5 model in this repository as mutable authority.
10. Do not infer Rust types from reference implementation names without validating the required invariants and ownership boundary.
11. Do not let G003 Agent persistence solve atomic coordination by pushing Agent message/provider/tool types into the shared `ww-store` API.
12. Do not expose unsafe write/process/network tools until replay classification and `RequiresIntervention` recovery are proven with fault injection.
13. Do not pull G004 OpenAI/fs.read/SDK/CLI work back into G003 for convenience; doing so recreates the oversized Goal that the planning review deliberately split.
14. Do not verify a task branch against a weaker gate than its merge target. A temporary workflow that omits one `main` check will record Tasks as verified that `main` would reject; a G003 verifier omitting `cargo fmt --all -- --check` did exactly that across T002-T004.
15. Do not infer Goal or architecture acceptance from branch placement. Work on `main` means the engineering gate passed, not that the Task, Goal, or ADR is accepted.
