# Contributing to Dietary Index Web Calculator

Thank you for your interest in contributing! This guide outlines the workflow, standards, and best practices for collaborating on this project.

---

## 📥 Getting Started

1. **Fork** the repository to your personal GitHub account.
2. **Clone** your fork:
   ```bash
   git clone https://github.com/<your-username>/dietary-index-web.git
   cd dietary-index-web
   ```
3. **Install** dependencies and set up your environment:
   ```bash
   ./setup.sh --dev
   ```

---

## 🌿 Workflow

- Create a **feature branch** off `main` named descriptively, e.g. `feature/add-ahei` or `fix/dii-bug`.
- Commit changes locally, adhering to style guidelines.
- Rebase or merge the latest `main` to keep your branch up-to-date.
- Push your branch and open a **Pull Request** (PR) against the `main` branch of the upstream repo.

---

## 🔧 Code Standards

- **Formatting & Linting**: Code is auto-formatted with **Black** and imports sorted with **isort**.
- **Linting**: Ensure **flake8** passes with no errors or warnings.
- **Pre-commit**: Hooks (black, isort, flake8, end-of-file-fixer) run on every `git commit`.
- **Type Hints**: Use PEP 484 type annotations where appropriate.
- **Docstrings**: Follow [Google style](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings).

---

## 🧪 Testing

- **Unit Tests**: Place tests in the `tests/` directory with filenames `test_*.py`.
- **Running Tests**:
  ```bash
  pytest tests/ --cov
  ```
- **Coverage**: Aim for at least **90%** coverage on core modules (`compute/`).

---

## 🚀 Pull Request Requirements

1. **Descriptive Title** and **summary** of changes in the PR description.
2. **Linked Issues**: Reference any related issue numbers (e.g., `Closes #123`).
3. **Screenshots** or sample outputs where relevant (e.g., UI changes, sample CSV outputs).
4. All **checks** (CI, linters, tests) must pass before review.

---

## 🤝 Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you agree to abide by its terms.

---

## 📖 Additional Resources

- **README.md**: Project overview, setup, and usage.
- **docs/validation\_detailed.md**: Scientific validation details for scoring methods.
- **.pre-commit-config.yaml**: Pre-commit hook configuration.
- **pyproject.toml**: Black, isort, flake8, pytest configuration.

---

Thank you for helping improve the Dietary Index Web Calculator! We welcome your feedback and contributions.
