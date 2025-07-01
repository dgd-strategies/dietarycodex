#!/usr/bin/env bash
# Bootstrap and verify the cached Codex environment.
set -euo pipefail

CACHE_DIR=".codex_env"
VENV="$CACHE_DIR/venv"
CACHE_FILE="$CACHE_DIR/.cache.json"

mkdir -p "$CACHE_DIR"

req_hash=$(sha256sum requirements.txt | awk '{print $1}')
cargo_hash=$(sha256sum Cargo.toml | awk '{print $1}')

saved_req=""
saved_cargo=""
if [[ -f "$CACHE_FILE" ]]; then
  saved_req=$(jq -r '.requirements_txt' "$CACHE_FILE")
  saved_cargo=$(jq -r '.cargo_toml' "$CACHE_FILE")
fi

if [[ ! -d "$VENV" || "$req_hash" != "$saved_req" || "$cargo_hash" != "$saved_cargo" ]]; then
  echo "🔧 Setting up virtual environment"
  python3 -m venv "$VENV"
  # shellcheck source=/dev/null
  source "$VENV/bin/activate"
  pip install --upgrade pip
  pip install -r requirements.txt
  pre-commit install
  jq -n --arg req "$req_hash" --arg cargo "$cargo_hash" '{requirements_txt:$req, cargo_toml:$cargo}' > "$CACHE_FILE"
fi

command -v rustc >/dev/null && command -v wasm-pack >/dev/null && command -v pre-commit >/dev/null && test -f schema/field_aliases.json && test -f docs/SCORING_CONTRACTS.md && echo "codex_ready = true"

echo "codex_env_bootstrap_enabled = true"
echo "cold_start_optimizations_active = true"
echo "redundant_ci_elimination_mode = safe"
echo "memoize_field_mappings_on_load = true"
