# Dietary Index Web Calculator

**A browser-based tool for computing multiple diet-quality scores (DII, MIND, HEI‑2015, DASH) from nutrition CSV data**.

---

## Quick Start

```bash
# Clone the repo
git clone https://github.com/<your‑fork>/dietary-index-web.git
d
# Change directory
cd dietary-index-web

# Start static site locally
./setup.sh

# OR full dev mode with FastAPI backend
./setup.sh --dev
```

- Static UI served at: [http://localhost:8000](http://localhost:8000)
- API documentation (Swagger) at: [http://localhost:8000/docs](http://localhost:8000/docs) when in `--dev` mode.

---

## Repository Structure

```
├── Dockerfile               # Production Docker image (static frontend)
├── Dockerfile.dev           # Dev Docker image (FastAPI + reload)
├── docker-compose.yml       # Mounts code + runs FastAPI / frontend together
├── setup.sh                 # Local setup script with `--dev` flag
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
