# G001 Specification

## Required architecture decisions

1. WorkWeave Agent is a probabilistic bounded execution primitive.
2. WorkWeave Flow is a deterministic durable OWS execution primitive.
3. WorkWeave Orchestration is a layer above both.
4. Agent and Flow share runtime infrastructure but own separate internal state machines.
5. OWS 1.0.3 + the frozen WorkWeave profile is the initial Flow definition target.
6. Local WorkWeave Agent invocation from Flow preserves the logical A2A boundary.
7. Durable audit is separate from exportable observability.
8. Both engines receive SDK, CLI and TUI surfaces.

## Required deliverables

- integrated engine dossier;
- Pi reference architecture;
- OWS/LangGraph Flow reference architecture;
- source register;
- Rust interface sketches;
- implementation slice proposal;
- review record.
