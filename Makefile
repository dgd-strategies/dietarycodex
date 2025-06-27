.PHONY: dev lint test clean run

dev:  # Local run
	python3 -m http.server 8000 --directory frontend

lint:  # Format + lint
	black compute/ tests/
	isort compute/ tests/
	flake8 compute/ tests/

test:
	pytest

clean:
	find . -name "__pycache__" -type d -exec rm -r {} +
	rm -rf .venv

run:
	source .venv/bin/activate && python3 -m http.server 8000 --directory frontend
