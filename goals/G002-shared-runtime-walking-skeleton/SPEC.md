# Specification

## Title

- Shared Runtime Walking Skeleton

## State

- active

## Interfaces

- `ww-types` — stable shared identifiers, lifecycle/event DTOs, digest and artifact references.
- `ww-store` — persistence contract for execution state, ordered events, and artifact metadata.
- `ww-store-sqlite` — SQLite implementation and migrations.
- `ww-runtime` — lifecycle rules, cancellation registry, reducer, and artifact service composition.
- `ww-sdk` — in-process façade for mutations, inspection, and committed-event streaming.
- `ww-cli` — `ww run ...` and `ww artifact ...` commands.

## Boundaries

- No Agent provider/model/tool contracts.
- No Flow workflow-definition, token, wait, branch, or expression contracts.
- No distributed scheduler or PostgreSQL implementation.
- No remote server/API or TUI in this Goal.
- No attempt at exactly-once external effects.
