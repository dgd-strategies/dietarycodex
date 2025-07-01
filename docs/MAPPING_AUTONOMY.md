# Mapping Autonomy

This document outlines the monitoring and self–improvement layer for raw food
fields. Any column that is not resolved by the `FoodItemResolver` is captured by
the `UnmappedFieldMonitor` during scoring. Each entry keeps a count of how often
it appears and up to three sample values.

The aggregated log is written to `data/unmapped_fields.json`. A helper script
[`scripts/review_unmapped.py`](../scripts/review_unmapped.py) parses this file
and proposes new definitions for `schema/food_components.json`. Suggestions are
stored under the `pending_review` block until manually accepted.

Every food mapping now records a `source`, `last_updated`, and `reviewed` flag to
track provenance.
