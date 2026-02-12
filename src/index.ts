// slp-enforcer v3.0 -- WASM-backed implementation (universal: Node.js + Browser)
// All business logic runs in Rust/WebAssembly via the peppi SLP parser.
//
// Usage:
//   import init, { analyzeReplay } from 'slp-enforcer'
//   await init()
//   const result = analyzeReplay(slpBytes, playerIndex)

import wasmInit, {
  initSync,
  analyze_replay,
  check_travel_time,
  check_disallowed_cstick,
  check_uptilt_rounding,
  check_crouch_uptilt,
  check_sdi,
  check_goomwave,
  check_control_stick_viz,
  check_input_fuzzing,
  check_handwarmer,
  is_slp_min_version,
  is_box_controller,
  is_box_controller_from_coords,
  get_coord_list_from_game,
  get_cstick_violations,
  average_travel_coord_hit_rate,
  has_goomwave_clamping,
  get_joystick_region,
  process_analog_stick,
  float_equals,
  is_equal,
  get_unique_coords,
  get_target_coords,
  get_game_settings,
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
  violations: Violation[]
}

/** Result of running all checks via analyzeReplay */
export type AllCheckResults = {
  travel_time: CheckResult
  disallowed_cstick: CheckResult
  uptilt_rounding: CheckResult
  crouch_uptilt: CheckResult
  sdi: CheckResult
  goomwave: CheckResult
  control_stick_viz: CheckResult
  input_fuzzing: CheckResult
}

/** Descriptor for a named check */
export type Check = {
  name: string
  checkFunction: (slpBytes: Uint8Array, playerIndex: number) => CheckResult
}

/** Metadata about an available check */
export type CheckInfo = {
  name: string
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

// ---- Replay Analysis ----

/** Run all checks on a player in one call */
export function analyzeReplay(slpBytes: Uint8Array, playerIndex: number): AllCheckResults {
  ensureInitialized()
  return analyze_replay(slpBytes, playerIndex) as AllCheckResults
}

// ---- Individual Check Functions ----

export function hasIllegalTravelTime(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_travel_time(slpBytes, playerIndex) as CheckResult
}

export function hasDisallowedCStickCoords(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_disallowed_cstick(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalUptiltRounding(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_uptilt_rounding(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalCrouchUptilt(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_crouch_uptilt(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalSDI(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_sdi(slpBytes, playerIndex) as CheckResult
}

export function isGoomwave(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_goomwave(slpBytes, playerIndex) as CheckResult
}

export function controlStickViz(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_control_stick_viz(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalInputFuzzing(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  ensureInitialized()
  return check_input_fuzzing(slpBytes, playerIndex) as CheckResult
}

// ---- List Checks ----

export function ListChecks(): Check[] {
  return [
    { name: "Box Travel Time", checkFunction: hasIllegalTravelTime },
    { name: "Disallowed Analog C-Stick Values", checkFunction: hasDisallowedCStickCoords },
    { name: "Uptilt Rounding", checkFunction: hasIllegalUptiltRounding },
    { name: "Fast Crouch Uptilt", checkFunction: hasIllegalCrouchUptilt },
    { name: "Illegal SDI", checkFunction: hasIllegalSDI },
    { name: "GoomWave Clamping", checkFunction: isGoomwave },
    { name: "Control Stick Visualization", checkFunction: controlStickViz },
    { name: "Input Fuzzing", checkFunction: hasIllegalInputFuzzing },
  ]
}

// ---- Utility Functions ----

export function isHandwarmer(slpBytes: Uint8Array): boolean {
  ensureInitialized()
  return check_handwarmer(slpBytes) as boolean
}

export function isSlpMinVersion(slpBytes: Uint8Array): boolean {
  ensureInitialized()
  return is_slp_min_version(slpBytes)
}

export function isBoxController(slpBytes: Uint8Array, playerIndex: number): boolean {
  ensureInitialized()
  return is_box_controller(slpBytes, playerIndex)
}

export function isBoxControllerFromCoords(coords: Coord[]): boolean {
  ensureInitialized()
  return is_box_controller_from_coords(coords)
}

export function getCoordListFromGame(slpBytes: Uint8Array, playerIndex: number, isMainStick: boolean): Coord[] {
  ensureInitialized()
  return get_coord_list_from_game(slpBytes, playerIndex, isMainStick) as Coord[]
}

export function getCStickViolations(coords: Coord[]): Violation[] {
  ensureInitialized()
  return get_cstick_violations(coords) as Violation[]
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

export function getGameSettings(slpBytes: Uint8Array): GameSettings {
  ensureInitialized()
  return get_game_settings(slpBytes) as GameSettings
}

// ---- Parsed Game Handle (parse once, query many times) ----

export { SlpGame }
