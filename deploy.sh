#!/bin/bash
set -e
wasm-pack build --target web --release
mkdir -p docs/pkg docs/assets docs/data
cp -r pkg/* docs/pkg/
cp index.html docs/
cp -r assets/* docs/assets/
cp data/template.csv docs/data/
cp sw.js docs/
