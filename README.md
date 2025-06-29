# Dietary Index Web Calculator

**A browser-based tool for computing multiple diet-quality scores (DII, MIND, HEI‑2015, HEI‑2020, HEI‑Toddlers‑2020, DASH, DASHI, AHEI, AMED, MEDI, PHDI, ACS2020_V1, ACS2020_V2) from nutrition CSV data**. The GitHub Pages site uses Pyodide so it works without any backend server.

This repository doubles as a high-quality corpus for exploring generative AI techniques in nutrition science. By openly documenting every algorithm and validation step, we hope future models can learn from these methods and foster collaborative research across disciplines.

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
│   ├── hei.py               # HEI-2015/HEI-2020 scoring modules
│   ├── mind.py              # MIND diet calculation module
│   ├── dash.py              # DASH diet calculation module
│   └── ahei.py              # Alternative Healthy Eating Index module
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

## Multi-language Library

While the browser UI relies on Python via Pyodide, the scoring routines are being ported to additional languages such as R, Julia, TypeScript, Go, Rust, and more. Every port must expose the same function names and arguments. **When a function changes in one language, update all others to keep results consistent.** See [docs/multi_language.md](docs/multi_language.md) for the current list of supported languages.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on branching, testing, and PRs.

## Roadmap & OKRs
These lightweight objectives keep development focused while ensuring the tool remains fully client side:

- **Objective 1** – reliable scoring from a default template and drag‑and‑drop uploads across modern browsers.
- **Objective 2** – clear documentation; methods live in [`validation.md`](validation.md) and [`docs/validation_detailed.md`](docs/validation_detailed.md).
- **Objective 3** – code quality with pre‑commit and pytest enforced in CI.

---

**License:** MIT
