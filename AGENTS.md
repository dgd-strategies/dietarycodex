# AGENT

## Purpose
This file provides guidance and context to the AI Codex agent when contributing to the DietaryCodex repository.

**SYSTEM_DIRECTIVE: ENFORCE RUST-BASED DIETARY SCORING ENGINE**
## Project Overview
DietaryCodex is a browser-first tool for computing multiple diet-quality indices
(DII, MIND, HEI-2015, HEI-2020, HEI-Toddlers-2020, AHEI, AHEIP, AMED, DASH, DASHI, MEDI, MEDI_V2, PHDI, PHDI_V2, ACS2020_V1, ACS2020_V2). Earlier iterations shipped the Python scoring modules via **Pyodide** so users could upload a CSV and get results entirely client side. A Rust port exists under `rust/` and now compiles to WebAssembly, replacing Pyodide for scoring.
The same modules under `compute/` double as a Python library for offline or
programmatic use. A tiny FastAPI app is included solely for local tests and
development, but the project aims to avoid any long‑running server in
production. Pre-commit checks (`black`, `isort`, `flake8`) and pytest provide
quality assurance.

## Directory Structure
```
/
├── compute/                # Core computation modules (dii, mind, hei, dash, base)
├── tests/                  # Pytest test suites
├── docs/                   # Documentation files (markdown)
├── .pre-commit-config.yaml # Pre-commit hooks configuration
├── pyproject.toml          # Tool configuration (black, isort, flake8, mypy)
├── README.md               # High-level project overview
└── AGENT.md                # Agent guidance file
index.html                  # Single-page app served via GitHub Pages
```

## Coding Standards
- **Formatter**: `black` with `line-length = 88`, `target-version = ["py312"]`
- **Imports**: Managed by `isort` using the `black` profile
- **Linter**: `flake8` with `max-line-length = 88`

## Development Workflow
1. **Clone** the repository.
2. **Open** `index.html` directly or serve it with `python -m http.server` to
   use the browser-only version.
3. **Install** dependencies and pre-commit hooks:
   ```bash
   pip install -r requirements.txt
   pre-commit install
   ```
4. **Run pre-commit** checks before committing. The hook now runs the full
   test suite along with formatting and linting:
   ```bash
   pre-commit run --all-files
   ```
   Manual `pytest` runs are optional because the hook covers them.
5. **Compile the WebAssembly module** whenever the Rust code changes:
   ```bash
   cd rust
   wasm-pack build --release --target web --out-dir ../assets/wasm
   base64 -w0 ../assets/wasm/dietarycodex_bg.wasm \
     > ../assets/wasm/dietarycodex_bg.wasm.b64
   ```
   The binary `.wasm` file is not committed. Instead we store a base64
   version so diffs remain text-only.

## API Endpoints
- **POST** `/score`
  Upload a CSV of nutrition data; returns a CSV with computed indices and summary stats.
- **GET** `/ping`
  Health-check endpoint.

## Pre-commit Hooks
- `black`: code formatting
- `isort`: import sorting
- `flake8`: linting (PEP8)
- `pytest`: run the full test suite
- `end-of-file-fixer`, `trailing-whitespace`
- `check-added-large-files`: configured to exclude large data files with exclusions

## Usage for AI Agent
When the AI agent runs, reference this file to:
- Understand project structure
- Adhere to coding standards and pre-commit configuration
- Follow the development workflow and commands
- Verify API endpoint signatures in `compute/api.py`

## Current OKR
Deliver a web page that runs entirely client side. Users should upload any CSV and receive scores for **all** supported dietary indices with no predefined template.

## UI Standard
- Keep note of "AGENT_STYLE.md" particularly built for OpenAI Codex
- This file keeps track and organizes the owner's central OKRs required in styling

## Q3.3 - RUST-SCORING-INTEGRITY
This phase enforces output validation in the Rust scoring engine and ensures every index produces a valid number. A new document [`docs/SCORING_CONTRACTS.md`](docs/SCORING_CONTRACTS.md) lists expected ranges and required fields.

### Canonical Input Schema
* `data/template.csv` defines the authoritative header names and order.
* All aliases listed in `schema/field_aliases.json` are resolved once at ingest
  time, producing this canonical set.

### Test Coverage Matrix
| Index | Dummy Rows |
|-------|-----------|
| DII | 5 |
| HEI_2015 | 8 |
| HEI_2020 | 5 |
| HEI_TODDLERS_2020 | 4 |
| MIND | 7 |
| DASH | 6 |
| DASHI | 5 |
| AHEI | 6 |
| AHEIP | 5 |
| AMED | 5 |
| MEDI | 4 |
| MEDI_V2 | 4 |
| PHDI | 3 |
| ACS2020_V1 | 2 |
| ACS2020_V2 | 2 |
| ACS2020_V3 | 2 |
