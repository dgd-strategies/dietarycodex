# Dietary Index Web Calculator

**A browser-based tool for computing multiple diet-quality scores (DII, MIND, HEI‑2015, DASH) from nutrition CSV data**. The GitHub Pages site uses Pyodide so it works without any backend server.

---

## Using the Web App

Just open [https://dgd-strategies.github.io/dietarycodex](https://dgd-strategies.github.io/dietarycodex) or `index.html` locally. The page works fully client side and registers a service worker providing a demo endpoint `/api/time`.

The optional `setup.sh` script only installs dependencies for tests and the Python library.

---

## Repository Structure

```
├── Dockerfile               # Production Docker image (static frontend)
├── Dockerfile.dev           # Optional dev image for the test API
├── docker-compose.yml       # Local orchestration for API tests
├── setup.sh                 # Installs deps and optional API for tests
├── requirements.txt         # Python dependencies
├── pyproject.toml           # black, isort, flake8, pytest config
├── .pre-commit-config.yaml  # Git pre-commit hooks
├── README.md                # This file
├── docs/
│   └── validation_detailed.md  # Full scientific validation guide
├── assets/
│   └── template.csv         # CSV template with all required headers
├── index.html               # Drag/drop UI + stats + Plotly charts
├── compute/
│   ├── __init__.py          # Package marker (empty)
│   ├── base.py              # Shared utilities (validation, stats)
│   ├── dii.py               # DII calculation module
│   ├── dii_parameters.json  # 45 DII parameter definitions
│   ├── hei.py               # HEI-2015 calculation module
│   ├── mind.py              # MIND diet calculation module
│   └── dash.py              # DASH diet calculation module
├── tests/
│   └── test_scoring.py      # Unit tests for scoring functions
└── .github/
    └── workflows/ci.yml     # GitHub Actions: lint, test, Docker build
```

---

## Usage

1. **Prepare a CSV** using [`assets/template.csv`](assets/template.csv) as a guide.
2. **Upload & Score** via the web UI or `POST /score` endpoint.
3. **Download** the enriched CSV and review summary statistics.
4. **Inspect** histogram/box plots for score distributions.

---

## Development Workflow

- **Formatting & linting**:

  ```bash
  black .
  isort .
  flake8
  ```

- **Testing**:

  ```bash
  pytest tests/ --cov
  ```

- **Pre-commit** hooks will run on `git commit` (black, isort, flake8).

---

## Validation

Detailed scoring methods, formulas, and reference citations are in [docs/validation\_detailed.md](docs/validation_detailed.md).

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on branching, testing, and PRs.

---

**License:** MIT
