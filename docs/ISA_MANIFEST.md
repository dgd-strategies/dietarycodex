# Intelligent System Agent Manifest

This document summarizes the intended behavior of the Intelligent System Agent (ISA).
The ISA monitors repository activity and aggregates external nutrition data
into the project's schema files. Its goal is to keep field mappings and
scoring contracts up to date without manual effort.

The implementation is lightweight. A background script collects new field
aliases and records proposed changes under `.codex/` so maintainers can
review them. Following a user override, Codex may also spawn ephemeral,
session-scoped ISA threads whenever the primary user initiates a session.

`scripts/isa_state_pipeline.py` powers these updates. The state file is
refreshed **before** pre-commit runs and committed if changes occur.
Pre-commit only validates that `ISA_STATE.json` and `schema_todo.json`
match the computed values; it will fail if they are outdated but will not
write to disk.

## Last Update

This file documents the current ISA scope. The primary user has authorized
session-scoped background threads for tasks such as field alias harvesting,
food component modeling, index contract mapping and external knowledge
syncing.
