#!/bin/bash
set -e
wasm-pack build --target web --release
mkdir -p docs/pkg
cp -r pkg/* docs/pkg/
cp static/index.html docs/
