# Plan

## Title

- Establish the smallest durable execution substrate

## State

- active

## Strategy

- Implement lifecycle semantics as typed Rust code above a transactional store contract.
- Persist current execution rows and immutable execution events in the same SQLite transaction.
- Reconstruct lifecycle projections from committed events and compare them with current state in tests.
- Add content-addressed filesystem artifacts with SQLite metadata after durable content creation.
- Expose only the SDK to CLI callers; never let CLI code mutate SQLite directly.
- Use GitHub Actions with the pinned Rust toolchain because the current sandbox lacks a local Rust compiler.

## Stop Conditions

- Stop if shared runtime types require Agent- or Flow-specific state to satisfy G002.
- Stop if SQLite transaction boundaries cannot provide atomic state + event commits.
- Stop if restart recovery depends on in-memory state that cannot be reconstructed from durable records.

## Rollback

- Revert G002 implementation commits while retaining G001 architecture and this Goal packet as evidence of the rejected approach.
