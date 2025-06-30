#!/bin/bash
set -euo pipefail

out=$(python scripts/validate_template.py)
# Expect JSON object with lists of scores
indexes=(DII DASH DASHI AHEI AHEIP AMED MEDI MEDI_V2 HEI_2015 HEI_2020 HEI_TODDLERS_2020 PHDI PHDI_V2 ACS2020_V1 ACS2020_V2)
for idx in "${indexes[@]}"; do
    val=$(echo "$out" | jq -r --arg k "$idx" '.[$k] | @sh')
    if [[ "$val" == "" || "$val" == "null" ]]; then
        echo "Missing score for $idx" >&2
        exit 1
    fi
done
