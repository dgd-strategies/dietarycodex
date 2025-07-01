#!/usr/bin/env python3
"""Background intelligence worker for DietaryCodex.

This module fetches remote food component definitions and merges them into
`schema/food_components.json`. The process runs in a background thread so it
does not block startup.
"""
from __future__ import annotations

import atexit
import json
import threading
from pathlib import Path
from typing import Any, Dict, List

import requests

ROOT = Path(__file__).resolve().parents[1]
FOOD_PATH = ROOT / "schema" / "food_components.json"
LOG_FILE = ROOT / "codex_session_enrichment.log"
REMOTE_URL = (
    "https://raw.githubusercontent.com/openai-demo/food-data/main/extra_components.json"
)


def log_line(text: str) -> None:
    with LOG_FILE.open("a", encoding="utf-8") as fh:
        fh.write(text + "\n")


def load_json(path: Path) -> Dict[str, Any]:
    if path.exists():
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    return {}


def save_json(path: Path, data: Dict[str, Any]) -> None:
    with path.open("w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2, sort_keys=True)
        fh.write("\n")


def fetch_remote_components() -> Dict[str, Any]:
    try:
        resp = requests.get(REMOTE_URL, timeout=10)
        resp.raise_for_status()
        return resp.json()
    except Exception as exc:  # noqa: BLE001
        log_line(f"fetch error: {exc}")
        return {}


def merge_components(local: Dict[str, Any], remote: Dict[str, Any]) -> List[str]:
    added: List[str] = []
    for key, value in remote.items():
        if key not in local:
            local[key] = value
            added.append(key)
    return added


def intelligence_task() -> None:
    local = load_json(FOOD_PATH)
    remote = fetch_remote_components()
    if not remote:
        log_line("no remote data")
        return
    added = merge_components(local, remote)
    if added:
        save_json(FOOD_PATH, local)
        log_line("added components: " + ", ".join(added))
    else:
        log_line("no new components")


def run_background() -> None:
    thread = threading.Thread(target=intelligence_task, daemon=True)
    thread.start()
    atexit.register(thread.join, timeout=2)


if __name__ == "__main__":
    run_background()
