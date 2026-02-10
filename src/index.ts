// slp-enforcer v2.0 -- WASM-backed implementation
// All business logic runs in Rust/WebAssembly via the peppi SLP parser.

const wasm = require('../pkg/node/libenforcer_wasm')

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

// ---- Replay Analysis ----

/** Run all checks on a player in one call */
export function analyzeReplay(slpBytes: Uint8Array, playerIndex: number): AllCheckResults {
  return wasm.analyze_replay(slpBytes, playerIndex) as AllCheckResults
}

// ---- Individual Check Functions ----

export function hasIllegalTravelTime(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_travel_time(slpBytes, playerIndex) as CheckResult
}

export function hasDisallowedCStickCoords(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_disallowed_cstick(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalUptiltRounding(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_uptilt_rounding(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalCrouchUptilt(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_crouch_uptilt(slpBytes, playerIndex) as CheckResult
}

export function hasIllegalSDI(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_sdi(slpBytes, playerIndex) as CheckResult
}

export function isGoomwave(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_goomwave(slpBytes, playerIndex) as CheckResult
}

export function controlStickViz(slpBytes: Uint8Array, playerIndex: number): CheckResult {
  return wasm.check_control_stick_viz(slpBytes, playerIndex) as CheckResult
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
  ]
}

// ---- Utility Functions ----

export function isHandwarmer(slpBytes: Uint8Array): boolean {
  return wasm.check_handwarmer(slpBytes) as boolean
}

export function isSlpMinVersion(slpBytes: Uint8Array): boolean {
  return wasm.is_slp_min_version(slpBytes)
}

export function isBoxController(slpBytes: Uint8Array, playerIndex: number): boolean {
  return wasm.is_box_controller(slpBytes, playerIndex)
}

export function isBoxControllerFromCoords(coords: Coord[]): boolean {
  return wasm.is_box_controller_from_coords(coords)
}

export function getCoordListFromGame(slpBytes: Uint8Array, playerIndex: number, isMainStick: boolean): Coord[] {
  return wasm.get_coord_list_from_game(slpBytes, playerIndex, isMainStick) as Coord[]
}

export function getCStickViolations(coords: Coord[]): Violation[] {
  return wasm.get_cstick_violations(coords) as Violation[]
}

export function averageTravelCoordHitRate(coords: Coord[]): number {
  return wasm.average_travel_coord_hit_rate(coords)
}

export function hasGoomwaveClamping(coords: Coord[]): boolean {
  return wasm.has_goomwave_clamping(coords)
}

export function getJoystickRegion(x: number, y: number): JoystickRegion {
  return wasm.get_joystick_region(x, y) as JoystickRegion
}

export function processAnalogStick(x: number, y: number, deadzone: boolean): Coord {
  return wasm.process_analog_stick(x, y, deadzone) as Coord
}

export function FloatEquals(a: number, b: number): boolean {
  return wasm.float_equals(a, b)
}

export function isEqual(one: Coord, other: Coord): boolean {
  return wasm.is_equal(one, other)
}

export function getUniqueCoords(coords: Coord[]): Coord[] {
  return wasm.get_unique_coords(coords) as Coord[]
}

export function getTargetCoords(coords: Coord[]): Coord[] {
  return wasm.get_target_coords(coords) as Coord[]
}

/** Convert a Node.js Buffer to Uint8Array (convenience for migration from v1) */
export function toUint8Array(buffer: Buffer): Uint8Array {
  return new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength)
}
