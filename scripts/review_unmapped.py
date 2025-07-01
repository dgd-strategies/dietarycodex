#!/usr/bin/env python3
#!/usr/bin/env python3
"""Review unmapped field log and suggest updates to food_components.json."""

from __future__ import annotations

import json
from difflib import get_close_matches
from pathlib import Path
from typing import Any, Dict

ROOT = Path(__file__).resolve().parents[1]
LOG_PATH = ROOT / "data" / "unmapped_fields.json"
FOOD_PATH = ROOT / "schema" / "food_components.json"

CONFIDENCE_THRESHOLD = 0.9


def load_json(path: Path) -> Any:
    if path.exists():
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def save_json(path: Path, data: Any) -> None:
    with path.open("w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, sort_keys=True)
        f.write("\n")


def suggest_mappings(log: Dict[str, Any], food_map: Dict[str, Any]) -> Dict[str, Any]:
    pending = food_map.setdefault("pending_review", {})
    existing = {k.lower() for k in food_map if k != "pending_review"}
    for field, info in log.items():
        name = field.lower()
        if name in existing or name in pending:
            continue
        choices = get_close_matches(name, existing, n=1, cutoff=CONFIDENCE_THRESHOLD)
        if choices:
            pending[name] = {
                "components": food_map[choices[0]]["components"],
                "source": "codex-suggested",
                "last_updated": "",
                "reviewed": False,
            }
    return food_map


def main() -> None:
    log = load_json(LOG_PATH)
    food_map = load_json(FOOD_PATH)
    if not log:
        print("No unmapped fields to review")
        return
    updated = suggest_mappings(log, food_map)
    save_json(FOOD_PATH, updated)
    print("Updated food_components.json with pending suggestions")


if __name__ == "__main__":
    main()
