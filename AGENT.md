# AGENT

## Purpose
This file provides guidance and context to the AI Codex agent when contributing to the DietaryCodex repository.

## Project Overview
DietaryCodex is a browser-first tool for computing multiple diet-quality indices
(DII, MIND, HEI‑2015, DASH). The GitHub Pages site runs the Python scoring
modules through **Pyodide**, so users can upload a CSV and get results entirely
client side. The same modules under `compute/` double as a Python library for
offline or programmatic use. A tiny FastAPI app is included solely for local
tests and development, but the project aims to avoid any long‑running server in
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
3. **Install** dependencies (for running tests or the optional library):
   ```bash
   pip install -r requirements.txt
   ```
4. **Run pre-commit** checks before committing:
   ```bash
   pre-commit run --all-files
   ```
5. **Run tests**:
   ```bash
   pytest -v
   ```

## API Endpoints
- **POST** `/score`
  Upload a CSV of nutrition data; returns a CSV with computed indices and summary stats.
- **GET** `/ping`
  Health-check endpoint.

## Pre-commit Hooks
- `black`: code formatting
- `isort`: import sorting
- `flake8`: linting (PEP8)
- `end-of-file-fixer`, `trailing-whitespace`
- `check-added-large-files`: configured to exclude large data files with exclusions

## Usage for AI Agent
When the AI agent runs, reference this file to:
- Understand project structure
- Adhere to coding standards and pre-commit configuration
- Follow the development workflow and commands
- Verify API endpoint signatures in `compute/api.py`

## Current OKRs
- Keep scoring fully client side using Pyodide.
- Ship a default CSV template that loads automatically.
- Document calculations and validation steps clearly.
- Enforce code quality via pre-commit and pytest in CI.
  
## UI Standard
- Keep note of "AGENT_STYLE.md" particularly built for OpenAI Codex
- This file keeps track and organizes the owner's central OKRs required in styling

