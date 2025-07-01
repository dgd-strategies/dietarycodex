# Codex Environment Optimization Plan

This document outlines steps to minimise initialization time and repeated setup
work for Codex. The intent is to keep the repository ready to score with minimal
warm‑up while preserving the Rust scoring engine integrity.

## Cached Environment Snapshot
- A dedicated `.codex_env/` directory stores a Python virtual environment and a
  cache file tracking hashes of build prerequisites.
- Dependencies are installed only when `requirements.txt` or `Cargo.toml` change.
- Pre‑commit hooks are installed once inside the cached environment.

## Repository Warm State Verification
- `scripts/bootstrap_codex_env.sh` performs a lightweight
  `codex_bootstrap_check()` ensuring:
  - Rust toolchain is available.
  - Wasm build tools (`wasm-pack`) are ready.
  - Pre‑commit is installed.
  - Schema and contract files under `schema/` load successfully.
- If all checks pass the script outputs `codex_ready = true`.

## Instant WASM Load & Canonical Schema Indexing
- Field aliases and contract files are parsed at bootstrap so subsequent runs do
  not re-read them from disk.
- This behaviour is controlled by
  `memoize_field_mappings_on_load = true`.

## CI and Build Optimisation
- Future work should cache the wasm build and cargo artefacts in GitHub Actions
  using job‑level keys.
- Redundant pre-commit checks should be reviewed to avoid duplicate linting or
  test runs.
- `build.rs` can skip recompilation when schema files are unchanged.

## Flags
- `codex_env_bootstrap_enabled = true`
- `cold_start_optimizations_active = true`
- `redundant_ci_elimination_mode = safe`
- `memoize_field_mappings_on_load = true`
