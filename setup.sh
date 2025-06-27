#!/bin/bash
# setup.sh — for the Dietary-Index Web Calculator

set -e

MODE="basic"
if [[ "$1" == "--dev" ]]; then
  MODE="dev"
fi

echo "🔁 Creating virtual environment..."
python3 -m venv .venv
source .venv/bin/activate

echo "📦 Installing Python dependencies..."
pip install --upgrade pip
pip install -r requirements.txt

echo "📎 Installing dev tools..."
pip install black flake8 isort pytest pre-commit

echo "⚙️ Installing pre-commit hooks..."
pre-commit install

echo "🧪 Running tests..."
pytest tests || echo "⚠️ Some tests failed — check test output above."

if [[ "$MODE" == "dev" ]]; then
  echo "🚀 Starting FastAPI dev backend with live reload..."
  pip install uvicorn fastapi[all] watchdog
  uvicorn compute.api:app --host 0.0.0.0 --port 8000 --reload
else
  echo "🌐 Starting local frontend server..."
  cd frontend
  python3 -m http.server 8000 &
  cd ..
  echo "✅ Setup complete. Visit http://localhost:8000"
fi
