# Slippi Enforcer - WebAssembly Edition

A high-performance browser-based tool for detecting cheating in Super Smash Bros Melee replays (.SLP files).

## Features

- **100% Local Processing** - All analysis happens in your browser, no data uploaded
- **WebAssembly Powered** - 5-10x faster than JavaScript implementation
- **6 Detection Checks**:
  - Box Travel Time - Detects digital box controllers
  - Disallowed C-Stick Values - Flags illegal coordinates
  - Uptilt Rounding - Detects artificial input rounding
  - Fast Crouch Uptilt - Impossible state transitions
  - Illegal SDI - Impossible input patterns
  - GoomWave Clamping - Hardware modification detection

## Technology Stack

- **Rust** - Core implementation language
- **Peppi** - Rust library for parsing Slippi files
- **WebAssembly** - For browser execution
- **wasm-bindgen** - Rust/JavaScript interop

## Development

### Prerequisites

- Rust (stable)
- wasm-pack
- Node.js (for local testing)

### Building

```bash
cd libenforcer-wasm
./build-wasm.sh
```

This will compile the Rust code to WebAssembly and output to `web/pkg/`.

### Local Testing

```bash
cd web
python3 -m http.server 8000
# Open http://localhost:8000 in your browser
```

## Deployment

This project auto-deploys to GitHub Pages via GitHub Actions on push to `main` or `wasm` branches.

## License

GPL-3.0-or-later
