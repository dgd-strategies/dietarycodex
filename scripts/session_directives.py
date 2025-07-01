#!/usr/bin/env python3
"""Utility to maintain the session directives log."""
from __future__ import annotations

import argparse
import datetime
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CODEX_DIR = ROOT / ".codex"
LOG_PATH = CODEX_DIR / "session_directives.log"
ARCHIVE_PATH = CODEX_DIR / "session_directives_archive.md"
HISTORY_DIR = ROOT / "logs" / "codex_session_history"


def append_entry(text: str, commit: str | None = None) -> None:
    CODEX_DIR.mkdir(exist_ok=True)
    HISTORY_DIR.mkdir(parents=True, exist_ok=True)
    ts = datetime.datetime.utcnow().isoformat()
    if commit is None:
        try:
            commit = subprocess.check_output(
                [
                    "git",
                    "rev-parse",
                    "HEAD",
                ],
                text=True,
                cwd=ROOT,
            ).strip()
        except Exception:
            commit = "unknown"
    line = f"{ts} | {commit} | {text}"
    with LOG_PATH.open("a", encoding="utf-8") as fh:
        fh.write(line + "\n")
    history_file = HISTORY_DIR / f"{ts[:10]}.log"
    with history_file.open("a", encoding="utf-8") as hist:
        hist.write(line + "\n")


def archive_if_needed() -> None:
    if not LOG_PATH.exists():
        return
    size = LOG_PATH.stat().st_size
    lines = LOG_PATH.read_text(encoding="utf-8").splitlines()
    if len(lines) <= 200 and size <= 100 * 1024:
        return
    old = lines[:-50]
    keep = lines[-50:]
    ts = datetime.datetime.utcnow().isoformat()
    first_ts = old[0].split("|")[0].strip() if old else ""
    last_ts = old[-1].split("|")[0].strip() if old else ""
    summary = f"### {ts}\nArchived {len(old)} entries from {first_ts} to {last_ts}.\n\n"
    with ARCHIVE_PATH.open("a", encoding="utf-8") as arch:
        arch.write(summary)
        arch.write("\n".join(old) + "\n")
    new_summary = (
        f"{ts} | archive | Archived {len(old)} entries to {ARCHIVE_PATH.name} "
        f"({first_ts}..{last_ts})"
    )
    with LOG_PATH.open("w", encoding="utf-8") as log:
        log.write(new_summary + "\n")
        log.write("\n".join(keep) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description="Append session directive entry")
    parser.add_argument("text", help="Directive summary text")
    parser.add_argument("--commit", help="Optional commit hash or message ID")
    args = parser.parse_args()
    archive_if_needed()
    append_entry(args.text, args.commit)


if __name__ == "__main__":
    main()
