#!/usr/bin/env python3
"""Synchronously refresh ISA state before running hooks."""
from __future__ import annotations

import sys
from pathlib import Path


def main() -> None:
    """Ensure path setup and update ISA state."""
    root = Path(__file__).resolve().parents[1]
    sys.path.insert(0, str(root))
    from scripts.isa_state_pipeline import update_state

    update_state()


if __name__ == "__main__":
    main()
