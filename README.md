# libenforcer

TypeScript and WebAssembly library for detecting cheating in Super Smash Bros Melee replays.

## WebAssembly Implementation

Includes a high-performance Rust/WASM implementation.

**Quick Start:**
```bash
cd libenforcer-wasm
./build-wasm.sh
cd ../web
python3 -m http.server 8000
```

## TypeScript Library

### Get Dependencies

`yarn`

### Test cases

`yarn test`