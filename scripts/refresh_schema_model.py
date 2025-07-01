"""Gather domain data and update schema caches.

This script fetches public resources (when network access is available)
and stores them under `.session_cache/` with a timestamped file name.
It intentionally keeps implementation light so pre-commit hooks remain fast.
"""

from __future__ import annotations

import datetime
import json
import logging
from pathlib import Path

import requests

CACHE_DIR = Path(".session_cache")
CACHE_DIR.mkdir(exist_ok=True)
LOG_DIR = Path("logs/knowledge_sync")
LOG_DIR.mkdir(parents=True, exist_ok=True)

SOURCES = {
    "nutriverse/dietaryindex": (
        "https://raw.githubusercontent.com/nutriverse/" "dietaryindex/main/README.md"
    ),
    # Additional sources can be added here
}


def gather() -> Path:
    """Collect data from known sources and write to cache."""
    timestamp = datetime.datetime.utcnow().strftime("%Y%m%dT%H%M%SZ")
    payload: dict[str, object] = {"timestamp": timestamp, "sources": {}}

    for name, url in SOURCES.items():
        try:
            resp = requests.get(url, timeout=10)
            resp.raise_for_status()
            payload["sources"][name] = resp.text
        except Exception as exc:  # noqa: BLE001
            payload["sources"][name] = f"error: {exc}"

    cache_file = CACHE_DIR / f"canonical_ingest_{timestamp}.json"
    cache_file.write_text(json.dumps(payload, indent=2))
    logging.info("Wrote %s", cache_file)

    summary = LOG_DIR / f"{timestamp}_summary.json"
    summary.write_text(json.dumps({"ingest": cache_file.name}, indent=2))
    logging.info("Logged summary %s", summary)
    return cache_file


if __name__ == "__main__":
    gather()
