// slp-enforcer v3.0 -- WASM-backed implementation (universal: Node.js + Browser)
// All business logic runs in Rust/WebAssembly via the peppi SLP parser.
//
// Usage:
//   import init, { SlpGame } from 'slp-enforcer'
//   await init()
//   const game = new SlpGame(slpBytes)
//   const result = game.analyzePlayer(playerIndex)

import wasmInit, {
  initSync,
  is_box_controller_from_coords,
  get_cstick_violations,
  average_travel_coord_hit_rate,
  has_goomwave_clamping,
  get_joystick_region,
  process_analog_stick,
  float_equals,
  is_equal,
  get_unique_coords,
  get_target_coords,
  SlpGame,
} from '../pkg/web/libenforcer_wasm.js'

// ---- Type Definitions ----

/** A 2D coordinate representing joystick position (values normalized -1.0 to 1.0) */
export type Coord = {
  x: number
  y: number
}

/** Represents a single violation of a rule */
export type Violation = {
  metric: number   // Usually a frame number
  reason: string
  evidence: Coord[]
}

/** Result of running a single check */
export type CheckResult = {
  result: boolean
  details: Violation[]
}

/** Controller type classification */
export type ControllerType = "Box" | "Analog"

/** Detailed statistical analysis of input fuzzing compliance */
export type FuzzAnalysis = {
  pass: boolean
  llr_score: number
  p_value_x: number | null
  p_value_y: number | null
  total_fuzz_events: number
  observed_x: [number, number, number]  // [n_minus, n_zero, n_plus]
  observed_y: [number, number, number]  // [n_minus, n_zero, n_plus]
  violations: Violation[]
}

/** Full analysis results for a single player */
export type PlayerAnalysis = {
  controller_type: ControllerType
  is_legal: boolean

  // Box controller checks (undefined if analog)
  travel_time?: CheckResult
  disallowed_cstick?: CheckResult
  crouch_uptilt?: CheckResult
  sdi?: CheckResult
  input_fuzzing?: FuzzAnalysis

  // Analog controller checks (undefined if box)
  goomwave?: CheckResult
  uptilt_rounding?: CheckResult
}

export type GameSettings = {
  stageId: number
  players: {
    playerIndex: number
    characterId: number
    playerType: number
    characterColor: number
  }[]
}

export enum JoystickRegion {
  DZ = 0,
  NE = 1,
  SE = 2,
  SW = 3,
  NW = 4,
  N = 5,
  E = 6,
  S = 7,
  W = 8,
}

// ---- Initialization ----

let initialized = false

function ensureInitialized(): void {
  if (!initialized) {
    throw new Error('slp-enforcer: WASM not initialized. Call init() first.')
  }
}

/**
 * Initialize the WASM module. Must be called once before using any other function.
 *
 * Call with no arguments to load from the default URL (relative to the module),
 * or pass a custom URL/path/Response to the .wasm file if your bundler requires it.
 * You may also pass raw WASM bytes (ArrayBuffer/Uint8Array) for synchronous initialization.
 */
export default async function init(wasmSource?: any): Promise<void> {
  if (initialized) return
  if (wasmSource instanceof ArrayBuffer || ArrayBuffer.isView(wasmSource)) {
    initSync({ module: wasmSource })
  } else {
    await wasmInit(wasmSource)
  }
  initialized = true
}
export { init }

// ---- Primary API: SlpGame ----

export { SlpGame }

// ---- Utility Functions ----

export function isBoxControllerFromCoords(coords: Coord[]): boolean {
  ensureInitialized()
  return is_box_controller_from_coords(coords)
}

export function getCStickViolations(coords: Coord[]): CheckResult {
  ensureInitialized()
  return get_cstick_violations(coords) as CheckResult
}

export function averageTravelCoordHitRate(coords: Coord[]): number {
  ensureInitialized()
  return average_travel_coord_hit_rate(coords)
}

export function hasGoomwaveClamping(coords: Coord[]): boolean {
  ensureInitialized()
  return has_goomwave_clamping(coords)
}

export function getJoystickRegion(x: number, y: number): JoystickRegion {
  ensureInitialized()
  return get_joystick_region(x, y) as JoystickRegion
}

export function processAnalogStick(x: number, y: number, deadzone: boolean): Coord {
  ensureInitialized()
  return process_analog_stick(x, y, deadzone) as Coord
}

export function FloatEquals(a: number, b: number): boolean {
  ensureInitialized()
  return float_equals(a, b)
}

export function isEqual(one: Coord, other: Coord): boolean {
  ensureInitialized()
  return is_equal(one, other)
}

export function getUniqueCoords(coords: Coord[]): Coord[] {
  ensureInitialized()
  return get_unique_coords(coords) as Coord[]
}

export function getTargetCoords(coords: Coord[]): Coord[] {
  ensureInitialized()
  return get_target_coords(coords) as Coord[]
}
