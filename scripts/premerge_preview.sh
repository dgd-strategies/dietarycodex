#!/bin/bash
set -euo pipefail

# Build minimal preview directory
TMPDIR=$(mktemp -d)
mkdir -p "$TMPDIR/pkg" "$TMPDIR/assets" "$TMPDIR/data"
cp index.html "$TMPDIR/"
cp sw.js "$TMPDIR/"
cp -r assets/* "$TMPDIR/assets/"
cp data/template.csv "$TMPDIR/data/"
base64 -d assets/wasm/dietarycodex_bg.wasm.b64 > "$TMPDIR/pkg/dietarycodex_bg.wasm"
cp assets/wasm/dietarycodex.js "$TMPDIR/pkg/"

# Verify scoring works
node scripts/validate_template.mjs >/dev/null

# Start local server and check page loads
python3 -m http.server -d "$TMPDIR" 8080 &
PID=$!
sleep 3
curl -sSf http://localhost:8080/index.html | grep -q "Loading WASM"
kill $PID

echo "[Validation] Preview built successfully"
