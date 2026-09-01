# WorkWeave Orchestration reference

This repository consumes WorkWeave Orchestration concepts but does not own their canonical model.

Canonical authority remains in `misawsneto/ww-orchestration`.

Pinned baseline used by G001:

- repository: `https://github.com/misawsneto/ww-orchestration`
- commit: `21aac374d28e6ad39944214866780a74b39f8e24`
- Domain model: `docs/orchestration/domain/model.yaml`
- Flow model: `docs/orchestration/flow/model.yaml`
- OWS profile: `docs/orchestration/ows/profile.yaml`
- design dossier: `docs/orchestration/WORKWEAVE-ORCHESTRATION-DOSSIER.md`

Relevant v0.5 boundary:

```text
OWS workflow definition
        |
        v
WorkWeave Flow runtime
        |
        v
guarded Domain services/commands
        |
        v
WorkWeave Domain truth
```

WorkWeave Engine owns the future execution architecture beneath and beside that model: Agent runs, Flow runtime implementation, workers, persistence, timers, signal correlation, external execution adapters, audit and deployment.
