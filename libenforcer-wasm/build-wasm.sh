#!/bin/bash
set -e

echo "Building WASM module with wasm-pack..."
cd "$(dirname "$0")"

# Build for web target (loads with ES modules)
wasm-pack build --target web --out-dir ../web/pkg --release

echo ""
echo "WASM build complete!"
echo "Output directory: ../web/pkg"
ls -lh ../web/pkg/*.wasm

echo ""
echo "WASM module size:"
du -h ../web/pkg/*.wasm
