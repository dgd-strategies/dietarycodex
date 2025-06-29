# 📚 Dietary Index Web Calculator Documentation

Welcome to the documentation portal for the **Dietary Index Web Calculator**. This page links to all key resources and guides to help you understand, use, and contribute to the project.

---

## 📖 Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Project Structure](#project-structure)
- [API Reference](#api-reference)
  - [Health Check](#health-check)
  - [Score Endpoint](#score-endpoint)
  - [Download Endpoint](#download-endpoint)
  - [OpenAPI Spec](#openapi-spec)
- [Validation & Scoring Standards](#validation--scoring-standards)
- [CSV Template](#csv-template)
- [Development Workflow](#development-workflow)
- [Contributing](#contributing)
- [Roadmap & OKRs](okrs.md)
- [License](#license)

---

## Overview

The **Dietary Index Web Calculator** lets users compute multiple diet-quality
indices (DII, MIND, HEI‑2015, HEI‑2020, HEI‑Toddlers‑2020, AHEI, AHEIP, AMED, DASH, DASHI, MEDI, MEDI_V2, PHDI, PHDI_V2, ACS2020_V1, ACS2020_V2) right in the browser. The original version runs the Python scoring modules via **Pyodide**. A Rust port is available under [`rust/`](../rust) and is meant to compile to WebAssembly so the frontend can eventually drop Pyodide. A minimal FastAPI backend exists only for automated tests.

---

## Quick Start

Open the hosted site or `index.html` locally. A service worker provides a demo `/api/time` endpoint. Everything runs in the browser.

---

## Project Structure

```
├── data/                   # Sample CSV templates & parameter JSON
├── compute/                # Python scoring modules
│   ├── base.py             # Validation & utils
│   ├── dii.py              # Dietary Inflammatory Index
│   ├── hei.py              # Healthy Eating Index 2015
│   ├── mind.py             # MIND diet score
│   ├── dash.py             # DASH diet score
│   ├── ahei.py             # Alternative Healthy Eating Index
│   ├── medi.py             # Mediterranean Diet Index
│   └── dii_parameters.json # Moved to data/
├── docs/                   # Documentation files
│   ├── README.md            # This file
│   ├── validation.md       # Scoring methods & rules
│   └── validation_detailed.md # Detailed formulas & references
├── index.html              # Single-page web app
├── tests/                  # Pytest suites
│   └── test_scoring.py
├── Dockerfile              # Static frontend container
├── Dockerfile.dev          # FastAPI dev container
├── docker-compose.yml      # Local orchestration
├── Makefile                # Handy shortcuts
├── openapi.yml             # API specification
├── pyproject.toml          # Tool configuration
├── requirements.txt        # Python dependencies
├── setup.sh                # Setup & dev launcher
└── .codexrc                # AI agent config
```

---

## API Reference

### Health Check

- **Endpoint**: `GET /ping`
- **Response**: `{ "status": "ok" }`

### Score Endpoint

- **Endpoint**: `POST /score`
- **Description**: Upload a CSV and optionally specify
  `?indices=DII,MIND,HEI_2015,HEI_2020,HEI_TODDLERS_2020,AHEI,AHEIP,AMED,DASH,DASHI,MEDI,MEDI_V2,PHDI,PHDI_V2,ACS2020_V1,ACS2020_V2`.
- **Response**: JSON with `filename` and `stats` (mean, std, min, max, median, quintiles).

### Download Endpoint

- **Endpoint**: `GET /download/{filename}`
- **Description**: Download the scored CSV.

### OpenAPI Spec

See the complete API definition: [openapi.yml](../openapi.yml)

---

## Validation & Scoring Standards

Refer to our validation rules and scientific methods: [validation.md](validation.md) and [validation_detailed.md](validation_detailed.md). Additional tables for scoring cut points and serving-size formulas are in [scoring_algorithms.md](scoring_algorithms.md) and [serving_sizes.md](serving_sizes.md).

---

## CSV Template

Download the master template for user uploads: [template.csv](../data/template.csv)

## Automatic Column Mapping

Raw USDA or FNDDS exports often use different column names than the scoring modules.
Use `compute.mapping.apply_mapping` to rename them automatically. Example:

```python
import pandas as pd
from compute.mapping import USDA_HEI_MAP, apply_mapping

df = pd.read_csv("fndds_export.csv")
df = apply_mapping(df, USDA_HEI_MAP)
```

### Manual Column Mapping

When your CSV headers don't exactly match the expected names, the web app now
prompts you to map them. After an initial upload, any unmatched required columns
are listed with a dropdown of your file's headers. Select the appropriate source
column for each requirement and click **Save Mapping**. The choices are stored in
your browser's local storage so future uploads automatically rename columns
before validation.

---

## Development Workflow

- **Install** requirements and pre-commit hooks:
  ```bash
  pip install -r requirements.txt
  pre-commit install
  ```
- **Formatting & Linting**: `make format`, `make lint`
- **Testing**: `make test`
- **Validation & Hooks**: `pre-commit run --all-files` will format, lint,
  and execute the full test suite before commit.
- **Optional API**: `make dev`
- **CI**: GitHub Actions runs on push/PR (`.github/workflows/ci.yml`)

---


## Contributing

See the [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on reporting issues and submitting PRs.

---

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Roadmap & OKRs
See [okrs.md](okrs.md) for the current objectives and key results guiding development.
