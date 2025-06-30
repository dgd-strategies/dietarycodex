#!/bin/bash
set -euo pipefail

node scripts/validate_template.mjs | jq -e 'length > 0 and (.[0] | length > 0)' >/dev/null
