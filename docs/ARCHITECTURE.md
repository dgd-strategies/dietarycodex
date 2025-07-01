# System Architecture

The project delivers diet-quality scoring directly in the browser. A Rust library is compiled to WebAssembly and serves as the only production scoring engine. Python modules exist solely for tests and regression validation.

**SYSTEM_DIRECTIVE: ENFORCE RUST-BASED DIETARY SCORING ENGINE**

Input CSV files are normalized via `compute.mapping.apply_mapping` before validation. Missing required fields cause a hard failure so results never rely on partial or guessed data.

**Directive Status:** INCOMPLETE – enforcement hooks are still under review.

## Intelligent System Agent
A thin automation layer keeps the schema and contracts in sync with
external datasets. Maintainers run `scripts/isa_state_pipeline.py`
before invoking pre-commit. The hook only checks that `ISA_STATE.json`
and `schema_todo.json` are current. Proposed updates are written under
`.codex/` for review.
