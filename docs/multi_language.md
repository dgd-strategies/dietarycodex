# 🌐 Multi-language Library Overview

This project intends to ship identical scoring libraries in many languages so contributors can work in their preferred environment. The original code base was written in R and then ported to Python. Future ports will follow the same API.

## Supported Languages
- **R** (reference)
- **Python** (source of truth)
- **Julia**
- **JavaScript / TypeScript**
- **Go**
- **Rust**
- **Java / Kotlin**
- **C# / F#**
- **C++**
- **Scala**
- **Ruby / Lua**
- **Haskell / OCaml**

## Core Functions
Every language implementation must expose these functions with the same behavior and arguments:

- `calculate_dii`
- `calculate_mind`
- `calculate_hei_2015`
- `calculate_dash`
- utilities for parameter loading and dataframe validation

When any algorithm or function signature changes, **update all language versions at the same time**. Use the Python modules as the authoritative reference and port changes promptly.

The Rust implementation is located under `rust/` and mirrors the Python API. Run
`cargo test` to execute its validation suite.

The web app relies on Pyodide running the Python code, so keep the browser UI backward compatible even as new language ports are added.
