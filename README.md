# Dietary Index Web Calculator

[![GitHub Pages](https://img.shields.io/badge/view-online-blue)](https://dgd-strategies.github.io/dietarycodex/)

**A browser-based tool for computing multiple diet-quality scores (DII, MIND, HEI-2015, HEI-2020, HEI-Toddlers-2020, AHEI, AHEIP, AMED, DASH, DASHI, MEDI, MEDI_V2, PHDI, PHDI_V2, ACS2020_V1, ACS2020_V2) from nutrition CSV data**. The original frontend relied on Pyodide to run Python scoring modules entirely client side. The current version ships a WebAssembly module compiled from Rust, eliminating the Pyodide dependency and speeding up calculations.

### Rust Scoring Pipeline

All public calculations flow through the Rust engine compiled to WebAssembly. The
Python modules under `compute/` remain for regression tests and offline
validation only. Uploaded CSV files are normalized to the expected column names
and then routed into the Rust binary for scoring. This ensures a single,
deterministic code path no matter where the data originates.

The engine now recognizes common field aliases. For example,
`alcohol_intake` is accepted as `alcohol_g` when parsing input
JSON or CSV data. This helps integrate real-world exports without
manual column renaming.

Production deployments run strictly through this Rust WebAssembly engine. The
browser applies stored column mappings and the Python layer verifies the rename
step before handing data to the binary. If any required field remains unmapped,
scoring stops with an explicit error instead of guessing or substituting values.

This repository doubles as a high-quality corpus for exploring generative AI techniques in nutrition science. By openly documenting every algorithm and validation step, we hope future models can learn from these methods and foster collaborative research across disciplines.

For a complete documentation index, see [docs/README.md](docs/README.md).

---

## Using the Web App

Just open [https://dgd-strategies.github.io/dietarycodex](https://dgd-strategies.github.io/dietarycodex) or `index.html` locally. The page works fully client side and registers a service worker providing a demo endpoint `/api/time`.

The optional `setup.sh` script only installs dependencies for tests and the Python library.
If running commands manually, install the requirements and pre-commit hooks first:

```bash
pip install -r requirements.txt
pre-commit install
```

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
├── data/
│   ├── template.csv         # Optional sample CSV
│   └── dii_parameters.json  # 45 DII parameter definitions
├── index.html               # Drag/drop UI + stats + Plotly charts
├── compute/
│   ├── __init__.py          # Package marker (empty)
│   ├── base.py              # Shared utilities (validation, stats)
│   ├── dii.py               # DII calculation module
│   ├── hei.py               # HEI-2015/HEI-2020 scoring modules
│   ├── mind.py              # MIND diet calculation module
│   ├── dash.py              # DASH diet calculation module
│   └── ahei.py              # Alternative Healthy Eating Index module
├── rust/                   # Rust implementation for future WASM frontend
├── tests/
│   └── test_scoring.py      # Unit tests for scoring functions
└── .github/
    └── workflows/ci.yml     # GitHub Actions: lint, test, Docker build
```

---

## Usage

1. **Upload & Score** your own CSV via the web UI or `POST /score` endpoint.
2. **Download** the enriched CSV and review summary statistics.
3. **Inspect** histogram/box plots for score distributions.

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

- **Pre-commit** hooks run automatically on `git commit` and include the
  complete pytest suite. You can trigger them manually as well:
  ```bash
  pre-commit run --all-files
  ```
- **Compile the WASM module** after editing the Rust code:
  ```bash
  cd rust
  wasm-pack build --release --target web --out-dir ../assets/wasm
  base64 -w0 ../assets/wasm/dietarycodex_bg.wasm \
    > ../assets/wasm/dietarycodex_bg.wasm.b64
  ```
  Only the `.b64` file is committed to avoid binary diffs.

---

## Validation

Detailed scoring methods, formulas, and reference citations are in [docs/validation_detailed.md](docs/validation_detailed.md). Reference CSV files used for regression tests live under `data/` and are executed automatically whenever the pre-commit hook runs.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on branching, testing, and PRs.

## Roadmap & OKRs
The sole objective is a **fully client-side webpage** where users drop in their own CSV and the app computes all available diet-quality indices.
---

**License:** MIT
