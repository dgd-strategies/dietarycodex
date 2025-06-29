# 📈 Project OKRs

This document outlines the **Objectives and Key Results** guiding ongoing development.

## Objective 1 – Reliable Client‑Side Scoring
- Keep the application 100% browser‑based using Pyodide.
- Default CSV template loads automatically so new users can try scoring immediately.
- Drag‑and‑drop file uploads work on all modern browsers.
- Maintain full functionality without any network connection once loaded.

## Objective 2 – Solid Validation & Documentation
- Provide clear instructions in the README and docs.
- Document calculation methods in `validation.md` and `validation_detailed.md`.
- Keep validation datasets under `data/` to enable automated regression tests.
- Maintain a tested Rust implementation mirroring the Python algorithms.
- Add examples and screenshots as features mature.

## Objective 3 – Code Quality & Automation
- Use pre‑commit (black, isort, flake8) and pytest on every change.
- Keep test coverage above 80% on the `compute` modules.
- Run CI via GitHub Actions to catch regressions.

These OKRs are intentionally lightweight. They should remind contributors and AI agents of the project's direction while leaving room for iterative improvement.
