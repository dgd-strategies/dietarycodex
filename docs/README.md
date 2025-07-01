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
- [Mapping Autonomy](MAPPING_AUTONOMY.md)
- [Development Workflow](#development-workflow)
- [Contributing](#contributing)
- [Roadmap & OKRs](okrs.md)
- [License](#license)

---

## Overview

The **Dietary Index Web Calculator** lets users compute multiple diet-quality
indices (DII, MIND, HEI‑2015, HEI‑2020, HEI‑Toddlers‑2020, AHEI, AHEIP, AMED, DASH, DASHI, MEDI, MEDI_V2, PHDI, PHDI_V2, ACS2020_V1, ACS2020_V2) right in the browser. Earlier versions executed the Python scoring modules using **Pyodide**. Now the frontend loads a WebAssembly module compiled from Rust for the core calculations. A minimal FastAPI backend exists only for automated tests.

### Rust Scoring Pipeline

All scoring runs through this Rust WebAssembly module. CSV columns are mapped to
standard names using the utilities in `compute.mapping` before being passed to
the engine. The Python modules remain available for automated tests and isolated
validation but are not part of the production path.

Production scoring executes **exclusively** through this WebAssembly engine. All
uploaded files are first normalized so their column names match the canonical
schema. The browser UI applies any saved mappings and `compute.mapping.apply_mapping`
performs the final rename step before validation. If a required field is still
missing after mapping, the engine aborts with a clear error message; there is no
silent fallback.

Example: a user might provide a column called `ALCOHOL` instead of the expected
`alcohol_g`. The normalization step recognizes this variant and maps it to
`alcohol_g`, allowing the Rust scoring engine to compute without issue.

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

An optional reference template is available: [template.csv](../data/template.csv).
It lists the canonical base field names derived from the scoring contracts.
Default units for each field live in
[`schema/default_units.json`](../schema/default_units.json). Uploaded
columns may include the unit suffix (e.g., `alcohol_g`) or provide the
unit separately. The import layer infers these units, converts to the
canonical measurement, and finally appends the expected suffixes to
match `[schema/required_columns.json](../schema/required_columns.json)`.
Use the template only for reference—the web app accepts any CSV and will
prompt for column mapping if headers differ.

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
- **Compile WASM** whenever Rust changes:
  ```bash
  cd rust
  wasm-pack build --release --target web --out-dir ../assets/wasm
  base64 -w0 ../assets/wasm/dietarycodex_bg.wasm \
    > ../assets/wasm/dietarycodex_bg.wasm.b64
  ```
  Commit only the `.b64` file.
- **Optional API**: `make dev`
- **CI**: GitHub Actions runs on push/PR (`.github/workflows/ci.yml`)
- **Review Unmapped Fields**: run `scripts/review_unmapped.py` to generate
  pending food mappings based on `data/unmapped_fields.json`.

## CI Scoring Validation Pipeline

The CI workflow asserts that the Rust binary is executed for all scoring tests.
It loads the WebAssembly module in a headless browser environment and fails if
any Python scoring path is used. Test artifacts confirm that the WASM module
initializes correctly and that example outputs contain populated, non-identical
index scores.

## Validation of Output Integrity

CI checks that every run produces non-null scores for required indices such as
`ACS2020`, `DII`, and `AHEI`. Blank or flat results trigger a failure condition.
Unit tests also verify that computed index values vary across rows rather than
remaining constant.
Snapshot or range-based comparisons may be added to ensure results stay within
expected bounds over time.

## Troubleshooting

### RuntimeError: unreachable

This usually means the WebAssembly module panicked. Follow these steps:

- Use a modern browser like Chrome or Firefox.
- Serve the page via `python -m http.server` instead of opening it directly.
- Open the browser console to read the panic message (enabled by the panic hook).

If the page stays on **Loading WASM...**, the module failed to load.

---


## Contributing

See the [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on reporting issues and submitting PRs.

---

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Roadmap & OKRs
See [okrs.md](okrs.md) for the current objectives and key results guiding development.
