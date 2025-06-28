# AGENT

## Purpose
This file provides guidance and context to the AI Codex agent when contributing to the DietaryCodex repository.

## Project Overview
DietaryCodex is a Python-based FastAPI application that computes multiple diet-quality indices (DII, MIND, HEI-2015, DASH) from uploaded nutrition data. It uses `pandas` for data handling and includes pre-commit checks (`black`, `isort`, `flake8`) and pytest for testing.

## Directory Structure
```
/
├── compute/                # Core computation modules (dii, mind, hei, dash, base)
├── frontend/               # Front-end static assets (CSS/JS)
├── tests/                  # Pytest test suites
├── docs/                   # Documentation files (markdown)
├── .pre-commit-config.yaml # Pre-commit hooks configuration
├── pyproject.toml          # Tool configuration (black, isort, flake8, mypy)
├── README.md               # High-level project overview
└── AGENT.md                # Agent guidance file
index.html                  # kept in root directy as the page has a static deployment using GitHub Pages
```

## Coding Standards
- **Formatter**: `black` with `line-length = 88`, `target-version = ["py312"]`
- **Imports**: Managed by `isort` using the `black` profile
- **Linter**: `flake8` with `max-line-length = 88`

## Development Workflow
1. **Clone** the repository.
2. **Install** dependencies:
   ```bash
   pip install -r requirements.txt
   ```
3. **Run pre-commit** checks before committing:
   ```bash
   pre-commit run --all-files
   ```
4. **Start server** for local testing:
   ```bash
   uvicorn compute.api:app --reload
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
