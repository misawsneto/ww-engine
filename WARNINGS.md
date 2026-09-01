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
