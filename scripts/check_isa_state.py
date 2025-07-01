#!/usr/bin/env python3
"""Validate that ISA_STATE.json is current without modifying it."""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from scripts.isa_state_pipeline import compute_state, load_json  # noqa: E402

STATE_FILE = ROOT / "ISA_STATE.json"
TODO_FILE = ROOT / "schema_todo.json"


def main() -> None:
    """Exit with non-zero status if ISA state is outdated."""
    current_state = load_json(STATE_FILE)
    current_todo = load_json(TODO_FILE)
    new_state, new_todo = compute_state()
    new_state["timestamp"] = current_state.get("timestamp")
    if current_state != new_state or current_todo != new_todo:
        print(
            "ISA_STATE.json is out of date. Run `python scripts/refresh_isa_state.py`.",
            file=sys.stderr,
        )
        sys.exit(1)
    print("ISA_STATE.json is up to date.")


if __name__ == "__main__":
    main()
