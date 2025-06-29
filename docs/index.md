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
- [Multi-language Library](multi_language.md)
- [Contributing](#contributing)
- [Roadmap & OKRs](okrs.md)
- [License](#license)

---

## Overview

The **Dietary Index Web Calculator** lets users compute multiple diet-quality
indices (DII, MIND, HEI‑2015, HEI‑2020, HEI‑Toddlers‑2020, AHEI, DASH, MEDI, PHDI, ACS2020_V1, ACS2020_V2) right in the browser. The GitHub Pages site
runs the Python scoring modules via **Pyodide**, so no server is required. A
minimal FastAPI backend exists only for automated tests.

---

## Using the Web App

Open the hosted site or `index.html` locally. A service worker provides a demo `/api/time` endpoint. Everything runs in the browser.

---

## Project Structure

```
├── assets/                 # Sample CSV templates, logos
├── compute/                # Python scoring modules
│   ├── base.py             # Validation & utils
│   ├── dii.py              # Dietary Inflammatory Index
│   ├── hei.py              # Healthy Eating Index 2015
│   ├── mind.py             # MIND diet score
│   ├── dash.py             # DASH diet score
│   ├── ahei.py             # Alternative Healthy Eating Index
│   ├── medi.py             # Mediterranean Diet Index
│   └── dii_parameters.json # DII global parameters
├── docs/                   # Documentation files
│   ├── index.md            # This file
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
  `?indices=DII,MIND,HEI_2015,HEI_2020,HEI_TODDLERS_2020,DASH,AHEI,MEDI`.
- **Response**: JSON with `filename` and `stats` (mean, std, min, max, median, quintiles).

### Download Endpoint

- **Endpoint**: `GET /download/{filename}`
- **Description**: Download the scored CSV.

### OpenAPI Spec

See the complete API definition: [openapi.yml](../openapi.yml)

---

## Validation & Scoring Standards

Refer to our validation rules and scientific methods: [validation.md](validation.md) and [validation_detailed.md](validation_detailed.md)

---

## CSV Template

Download the master template for user uploads: [template.csv](../assets/template.csv)

---

## Development Workflow

- **Formatting & Linting**: `make format`, `make lint`
- **Testing**: `make test`
- **Optional API**: `make dev`
- **CI**: GitHub Actions runs on push/PR (`.github/workflows/ci.yml`)

---

## Multi-language Library

The scoring algorithms are being ported to several languages beyond Python. Each
language module should expose the same functions (`calculate_dii`,
`calculate_mind`, `calculate_hei_2015`, `calculate_dash`). When one
implementation changes, every other port must be updated to stay in sync. See
[multi_language.md](multi_language.md) for the list of supported languages and
workflow tips.

---

## Contributing

See the [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on reporting issues and submitting PRs.

---

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Roadmap & OKRs
See [okrs.md](okrs.md) for the current objectives and key results guiding development.
