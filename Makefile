.PHONY: install dev clean format lint test ci

# Install Python dependencies and set up environment
install:
	./setup.sh

# Start FastAPI backend (with reload) and static frontend server
dev:
	./setup.sh --dev

# Format code with Black and sort imports with isort
format:
	black compute tests frontend
	isort compute tests

# Run linter (flake8)
lint:
	flake8 compute tests

# Run unit tests with coverage
test:
	pytest tests --cov=compute

# Clean up Python caches and temporary files
clean:
	rm -rf __pycache__ .pytest_cache .venv

# Continuous integration target: format, lint, and test
ci: format lint test
