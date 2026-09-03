# G004 Tasks

| Task | State | Acceptance | Dependencies |
| --- | --- | --- | --- |
| T001 Accept G003 review and activate G004 | open | G003 achieved; ADR-0004 accepted; G004 active; project state points to G004 | G003 T012 |
| T002 Implement OpenAI adapter and transport conformance fixtures | open | OpenAI HTTP/SSE translation passes text/tool/usage/error/disconnect/cancel/truncation fixtures; no vendor types leak; mandatory tests need no credential | T001 |
| T003 Implement bounded `fs.read` local tool | open | canonical workspace containment, symlink escape rejection, UTF-8/range/byte bounds, cancellation, and normalized read result are tested | T001 |
| T004 Add Agent projector and Rust SDK surface | open | SDK starts/cancels/inspects/streams runs using normalized Agent projections and no raw DB/provider payload exposure | T002 |
| T005 Add `ww agent` CLI and process-boundary mock-provider E2E | open | CLI uses SDK only; local mock OpenAI server drives `model → fs.read → model`; a new process inspects durable run/events with stable JSON/JSONL output | T003, T004 |
| T006 Record required EvaluationRuns and perform G004 architecture/security review | open | provider/fs/SDK/CLI Evaluations pass; fmt/boundary/clippy/tests pass; review finds no secret leakage, path escape, store bypass, Flow coupling, or recovery-semantic fork | T005 |
