# Intelligent System Agent Manifest

This document summarizes the intended behavior of the Intelligent System Agent (ISA).
The ISA monitors repository activity and aggregates external nutrition data
into the project's schema files. Its goal is to keep field mappings and
scoring contracts up to date without manual effort.

The implementation is lightweight. A background script collects new field
aliases and records proposed changes under `.codex/` so maintainers can
review them. Pre-commit hooks run these checks automatically. Following a
user override, Codex may also spawn ephemeral, session-scoped ISA threads
whenever the primary user initiates a session.

`scripts/isa_state_pipeline.py` powers these updates. It runs in a
background thread during pre-commit to update `ISA_STATE.json` and
`schema_todo.json` with any missing canonical fields.

## Last Update

This file documents the current ISA scope. The primary user has authorized
session-scoped background threads for tasks such as field alias harvesting,
food component modeling, index contract mapping and external knowledge
syncing.
