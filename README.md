# libenforcer

TypeScript and WebAssembly library for detecting cheating in Super Smash Bros Melee replays (.slp files).

## Install

```bash
yarn add slp-enforcer
```

## Usage

```typescript
import init, { SlpGame } from 'slp-enforcer'

await init()

const game = new SlpGame(slpBytes)
const result = game.analyzePlayer(playerIndex)

console.log(result.controller_type) // "Box" or "Analog"
console.log(result.is_legal)        // true if all applicable checks pass

// Box controller checks (undefined if analog)
result.travel_time       // CheckResult
result.disallowed_cstick // CheckResult
result.crouch_uptilt     // CheckResult
result.sdi               // CheckResult
result.input_fuzzing     // FuzzAnalysis (LLR score, p-values, odds ratio)

// Analog controller checks (undefined if box)
result.goomwave          // CheckResult
result.uptilt_rounding   // CheckResult

game.free()
```

### Other SlpGame methods

```typescript
game.isBoxController(playerIndex)    // boolean
game.getMainStickCoords(playerIndex) // Coord[]
game.getCStickCoords(playerIndex)    // Coord[]
game.isHandwarmer()                  // boolean
game.getGameSettings()               // GameSettings
game.isSlpMinVersion()               // boolean
```

### Utility functions

Standalone functions that operate on raw coordinate arrays:

`isBoxControllerFromCoords`, `getCStickViolations`, `averageTravelCoordHitRate`, `hasGoomwaveClamping`, `getJoystickRegion`, `processAnalogStick`, `FloatEquals`, `isEqual`, `getUniqueCoords`, `getTargetCoords`

## Development

### Build

```bash
yarn build        # WASM + TypeScript
yarn build:wasm   # WASM only
yarn build:ts     # TypeScript only
```

### Test

```bash
yarn test             # Rust + TypeScript (all tests)
yarn test:rust        # Rust unit + integration tests only
yarn test:ts          # TypeScript integration tests only
```
