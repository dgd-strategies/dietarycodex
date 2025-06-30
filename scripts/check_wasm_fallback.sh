#!/bin/bash
set -euo pipefail

# Run the Node validator using the compiled WASM module
if ! node scripts/validate_template.mjs >/dev/null 2>&1; then
  echo "Rust WASM scoring failed or Python fallback triggered" >&2
  exit 1
fi

echo "[Validation] Rust scoring path confirmed"
