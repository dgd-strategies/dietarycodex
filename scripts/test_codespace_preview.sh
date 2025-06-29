#!/bin/bash
set -e
echo "[Test] Simulating Codespace environment..."

wasm-pack build --target web --release
mkdir -p docs/pkg
cp -r pkg/* docs/pkg/
cp static/index.html docs/

echo "[Test] Launching local preview server at http://localhost:8080"
cd docs
python3 -m http.server 8080 &
server_pid=$!
sleep 3

curl -sSf http://localhost:8080 | grep "<h1>Rust WASM App</h1>" || {
  echo "WASM app failed to load expected content"
  kill $server_pid
  exit 1
}

echo "[Test] WASM app loaded successfully"
kill $server_pid
