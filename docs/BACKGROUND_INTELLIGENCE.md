# Background Intelligence Worker

`scripts/codex_intelligence_worker.py` fetches remote food component definitions
and merges them into `schema/food_components.json`. The script runs a background
thread so repository startup is not delayed. The log of actions is appended to
`codex_session_enrichment.log`.

Usage:
```bash
python scripts/codex_intelligence_worker.py
```
