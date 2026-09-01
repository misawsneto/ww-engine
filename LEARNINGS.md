# Learnings

1. Pi's valuable core is a small provider-neutral model/tool loop surrounded by much larger product/session/UI machinery.
2. Pi separates provider streaming from the agent loop through `StreamFn`, which is a strong substitution seam.
3. Pi's future Harness adds durable operations, lanes, records and reduction; these are useful runtime ideas but the public Harness is incomplete.
4. WorkWeave v0.5 already defines OWS as Flow-definition authority and explicitly leaves workers, schedulers, persistence and Agent adapters to Architecture.
5. LangGraph demonstrates that durability, interrupts and streaming can be first-class runtime mechanics without making them semantic work concepts.
6. Agent and Flow share operational concerns but have fundamentally different notions of next action, state, resume and completion.
7. A local Flow-to-Agent call can use the same logical contract as a remote A2A call, preserving composition without coupling state machines.
8. Auditability requires a durable ordered execution record in addition to exportable traces and metrics.
9. CLI/TUI/SDK should expose both engines directly; Agent is not merely a node hidden inside Flow and Flow is not merely an Agent tool.
10. The first Rust slice should falsify boundaries before expanding provider, workflow, plugin or sandbox breadth.
