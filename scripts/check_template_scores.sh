#!/bin/bash
set -euo pipefail

out=$(node scripts/validate_template.mjs)
echo "$out" | jq -e 'length > 0 and .[0].DII != null' >/dev/null
