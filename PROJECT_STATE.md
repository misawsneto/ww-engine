# Project State

## Current

- Product: WorkWeave Engine.
- Language target: Rust.
- Active Goal: `G001 — WorkWeave Execution Architecture`.
- Architecture thesis: one shared Rust runtime substrate with two sibling execution kernels.
- Agent kernel: probabilistic LLM/tool loop inspired by Pi.
- Flow kernel: deterministic durable OWS runtime informed by LangGraph execution mechanics.
- Orchestration: separate layer above the engine; canonical semantics remain owned by `misawsneto/ww-orchestration` v0.5.

## Current evidence pins

- Pi reference revision: `6c87d9a026677b601e8278030dcf1ad97fe0bd86`.
- WorkWeave Orchestration reference revision: `21aac374d28e6ad39944214866780a74b39f8e24`.
- OWS specification revision: `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.
- LangGraph reference revision: `11ee185999b86bfea2d8c0e69cef9a5e37acf686`.

## Next acceptance boundary

G001 is ready when architecture review agrees on:

1. sibling Agent/Flow semantics;
2. shared runtime ownership;
3. OWS definition authority;
4. Flow-to-Agent execution contract;
5. Rust crate topology;
6. persistence and audit boundaries;
7. SDK/CLI/TUI surface strategy;
8. a bounded G002 spike capable of falsifying the architecture.
