#!/usr/bin/env python3
"""Session-scoped ISA pipeline for background schema updates."""
from __future__ import annotations

import atexit
import json
import threading
import time
from pathlib import Path
from typing import Any, Dict, List

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schema"
STATE_FILE = ROOT / "ISA_STATE.json"
TODO_FILE = ROOT / "schema_todo.json"


def load_json(path: Path) -> Dict[str, Any]:
    if path.exists():
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    return {}


def save_json(path: Path, data: Dict[str, Any]) -> None:
    with path.open("w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2, sort_keys=True)
        fh.write("\n")


def gather_schema_info() -> Dict[str, Any]:
    info: Dict[str, Any] = {}
    for name in [
        "field_aliases.json",
        "contracts.json",
        "food_components.json",
        "scoring_priority.json",
    ]:
        info[name] = load_json(SCHEMA_DIR / name)
    return info


def detect_missing_fields(schema: Dict[str, Any]) -> List[str]:
    aliases = set(schema.get("field_aliases.json", {}).values())
    required: set[str] = set()
    for contract in schema.get("contracts.json", {}).values():
        required.update(contract.get("required", []))
    missing = sorted(required - aliases)
    return list(missing)


def compute_state() -> tuple[Dict[str, Any], Dict[str, Any]]:
    """Return the state and todo structures without writing them."""
    schema = gather_schema_info()
    state = load_json(STATE_FILE)
    state["timestamp"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    state.setdefault("observed_changes", [])
    state.setdefault("recent_mappings", [])
    state["scoring_priority"] = schema.get("scoring_priority.json", {})
    todo = {"missing_fields": detect_missing_fields(schema)}
    return state, todo


def update_state() -> None:
    """Write the computed ISA state to disk if it changed."""
    state, todo = compute_state()
    current_state = load_json(STATE_FILE)
    current_todo = load_json(TODO_FILE)
    state_no_time = {k: v for k, v in state.items() if k != "timestamp"}
    cur_no_time = {k: v for k, v in current_state.items() if k != "timestamp"}
    if state_no_time != cur_no_time or todo != current_todo:
        save_json(STATE_FILE, state)
        save_json(TODO_FILE, todo)


def run_background() -> None:
    thread = threading.Thread(target=update_state, daemon=True)
    thread.start()
    atexit.register(thread.join, timeout=3)


if __name__ == "__main__":
    run_background()
