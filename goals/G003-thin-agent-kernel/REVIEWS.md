# G003 Reviews

No implementation review exists because G003 is proposed and blocked by G002 independent review.

## Review focus when active

- functional Agent loop remains small and Pi-like rather than becoming a session/product monolith;
- provider-specific behavior terminates at adapters;
- finalized messages/tool operations are auditable without persisting hidden reasoning;
- tool replay classification closes crash ambiguity before unsafe tools are exposed;
- common execution and Agent persistence remain logically separate but transactionally coherent where required;
- cancellation/deadline/budget settlement is deterministic around the probabilistic model loop;
- SDK/CLI do not bypass kernel or persistence boundaries;
- no Flow/OWS or WorkWeave Orchestration semantic leakage.
