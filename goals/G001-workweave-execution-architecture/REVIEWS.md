# G001 Reviews

## Architecture self-review

### Passes

- The design does not embed WorkWeave Goal/Task/Evaluation semantics into the Agent loop.
- The design does not replace OWS with a proprietary workflow graph.
- Shared runtime abstractions stop at operational concerns.
- Agent and Flow terminal state have distinct engine meaning.
- Flow-to-Agent integration uses an explicit adapter boundary.
- Pi Harness is qualified as incomplete rather than represented as production behavior.

### Open review points

- strict jq runtime choice;
- exact storage transaction/outbox design;
- first public remote protocol boundary;
- minimum sandbox policy for G002;
- whether the common `Executor` abstraction is useful in code or should remain only conceptual until the spike.
