# Orchestration Starter Adoption

`ww-engine` adopts the **bookkeeping and development philosophy** of `misawsneto/ww-orchestration-starter`, not its stale product identity or every copied semantic assumption.

## Adopted

- Goal packets with `GOAL`, `SPEC`, `PLAN`, `TASKS`, `VERIFICATION`, and `REVIEWS`.
- Root project records for state, decisions, questions, learnings, and warnings.
- The simplest-path Git/sandbox operating philosophy.
- Markdown authoring templates for Goal/Plan/Task/Decision/Question/Evaluation/Review bookkeeping.
- Evidence-first architecture research and explicit verification before terminal completion.

## Adapted

- Workspace identity is `/mnt/data/ww-engine`.
- Product scope is WorkWeave Runtime + Agent + Flow in Rust.
- Canonical orchestration reference is WorkWeave v0.5, not the starter's copied v0.4 index.
- OWS is Flow-definition authority.

## Held back deliberately

The starter's `skills/ww-*` snapshot is **not copied yet**. Several skills still contain v0.4/FEEL-era Flow terminology such as `FEEL_validate_*`, v0.4 command wording, and pre-OWS Flow assumptions.

Those procedures may still be useful, but they must be requalified against WorkWeave Orchestration v0.5 before becoming active project guidance. Importing them unchanged would create negative drift.

## Authority

- Engine architecture: this repository.
- Orchestration semantic authority: `misawsneto/ww-orchestration`.
- Starter repository: procedural/bookkeeping reference only.
