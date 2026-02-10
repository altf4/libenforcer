#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Building WASM module (Node.js target)..."
wasm-pack build --target nodejs --out-dir ../pkg/node --release

echo ""
echo "Building WASM module (Web target)..."
wasm-pack build --target web --out-dir ../pkg/web --release

echo ""
echo "Build complete!"
echo ""
echo "Node.js module:"
du -h ../pkg/node/*.wasm
echo ""
echo "Web module:"
du -h ../pkg/web/*.wasm
