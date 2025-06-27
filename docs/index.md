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
- [License](#license)

---

## Overview

The **Dietary Index Web Calculator** is a browser-based tool for computing multiple diet-quality indices (DII, MIND, HEI‑2015, DASH) from user-uploaded nutrition data. It combines a vanilla JS frontend with a Python FastAPI backend.

---

## Quick Start

1. **Install**
   ```bash
   ./setup.sh          # Frontend-only
   ./setup.sh --dev    # Full dev mode (FastAPI + reload)
   ```
2. **Run**
   - Frontend: http://localhost:8000 (static server)
   - Backend:  http://localhost:8000/ping, /score, /download

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
│   └── dii_parameters.json # DII global parameters
├── docs/                   # Documentation files
│   ├── index.md            # This file
│   ├── validation.md       # Scoring methods & rules
│   └── validation_detailed.md # Detailed formulas & references
├── frontend/               # Single-page web app
│   └── index.html
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
- **Description**: Upload a CSV and optionally specify `?indices=DII,MIND,HEI_2015,DASH`.
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
- **Dev Server**: `make dev`
- **CI**: GitHub Actions runs on push/PR (`.github/workflows/ci.yml`)

---

## Contributing

See the [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on reporting issues and submitting PRs.

---

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
