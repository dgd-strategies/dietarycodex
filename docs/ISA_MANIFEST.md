# Intelligent System Agent Manifest

This document summarizes the intended behavior of the Intelligent System Agent (ISA).
The ISA monitors repository activity and aggregates external nutrition data
into the project's schema files. Its goal is to keep field mappings and
scoring contracts up to date without manual effort.

The implementation is lightweight. A background script collects new field
aliases and records proposed changes under `.codex/` so maintainers can
review them. Pre-commit hooks run these checks automatically.

## Last Update

This file was generated during an automated Codex session to document the
current ISA scope. No persistent background process is running.
