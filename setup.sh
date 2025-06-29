#!/bin/bash
# setup.sh — for the Dietary-Index Web Calculator

set -e

MODE="prod"
if [[ "$1" == "--dev" ]]; then
  MODE="dev"
fi

echo "🔁 Creating virtual environment..."
python3 -m venv .venv
# shellcheck source=/dev/null
source .venv/bin/activate

echo "📦 Installing Python dependencies..."
pip install --upgrade pip
pip install -r requirements.txt

echo "📎 Installing dev tools..."
pip install black flake8 isort pytest pre-commit shellcheck-py

echo "⚙️ Installing pre-commit hooks..."
pre-commit install

echo "🧪 Running tests..."
pytest tests || echo "⚠️ Some tests failed — check test output above."

echo "🚀 Starting FastAPI backend..."
pip install uvicorn 'fastapi[all]' watchdog
if [[ "$MODE" == "dev" ]]; then
  EXTRA="--reload"
else
  EXTRA=""
fi
uvicorn compute.api:app --host 0.0.0.0 --port 8000 $EXTRA
