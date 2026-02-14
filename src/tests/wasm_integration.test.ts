import { expect, test, beforeAll } from '@jest/globals'
import * as fs from 'node:fs'
import * as path from 'node:path'
import { fileURLToPath } from 'node:url'
import init, {
  analyzeReplay,
  hasIllegalTravelTime,
  hasDisallowedCStickCoords,
  hasIllegalUptiltRounding,
  hasIllegalCrouchUptilt,
  hasIllegalSDI,
  isGoomwave,
  controlStickViz,
  isHandwarmer,
  isSlpMinVersion,
  isBoxController,
  isBoxControllerFromCoords,
  ListChecks,
  FloatEquals,
  getJoystickRegion,
  JoystickRegion,
  getUniqueCoords,
  getTargetCoords,
  getCoordListFromGame,
  averageTravelCoordHitRate,
  getCStickViolations,
  hasGoomwaveClamping,
  processAnalogStick,
  isEqual,
  Coord,
} from '../index.js'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const TEST_DATA = path.join(__dirname, '../../test_data')

beforeAll(async () => {
  const wasmBytes = fs.readFileSync(path.join(__dirname, '../../pkg/web/libenforcer_wasm_bg.wasm'))
  await init(wasmBytes)
})

function loadSlp(relativePath: string): Uint8Array {
  const data = fs.readFileSync(path.join(TEST_DATA, relativePath))
  return new Uint8Array(data.buffer, data.byteOffset, data.byteLength)
}

function loadSlpDir(relativePath: string): { name: string; data: Uint8Array }[] {
  const dir = path.join(TEST_DATA, relativePath)
  return fs.readdirSync(dir)
    .filter(f => f.endsWith('.slp'))
    .sort()
    .map(f => {
      const data = fs.readFileSync(path.join(dir, f))
      return { name: f, data: new Uint8Array(data.buffer, data.byteOffset, data.byteLength) }
    })
}

// ---- analyzeReplay ----

test('analyzeReplay returns all check results', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  const results = analyzeReplay(slp, 2)

  expect(results).toHaveProperty('travel_time')
  expect(results).toHaveProperty('disallowed_cstick')
  expect(results).toHaveProperty('uptilt_rounding')
  expect(results).toHaveProperty('crouch_uptilt')
  expect(results).toHaveProperty('sdi')
  expect(results).toHaveProperty('goomwave')
  expect(results).toHaveProperty('control_stick_viz')

  expect(results.travel_time.result).toBe(false)
  expect(results.sdi.result).toBe(false)
})

// ---- Travel Time ----

test('legal digital files pass travel time', () => {
  for (let i = 1; i <= 7; i++) {
    const slp = loadSlp(`legal/digital/potion_p3/potion_${i}.slp`)
    const result = hasIllegalTravelTime(slp, 2)
    expect(result.result).toBe(false)
  }
})

test('legal digital files have travel hit rate > 0.30', () => {
  for (let i = 1; i <= 7; i++) {
    const slp = loadSlp(`legal/digital/potion_p3/potion_${i}.slp`)
    const coords = getCoordListFromGame(slp, 2, true)
    expect(averageTravelCoordHitRate(coords)).toBeGreaterThan(0.30)
  }
})

test('nonlegal digital files fail travel time', () => {
  const files = loadSlpDir('nonlegal/digital/pre-ruleset/')
  for (const { name, data } of files) {
    const result = hasIllegalTravelTime(data, 3)
    expect(result.result).toBe(true)
  }
})

test('averageTravelCoordHitRate with known coords', () => {
  const coords: Coord[] = [
    { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 },
    { x: 1, y: 1 }, { x: 1, y: 1 },
    { x: 0.5, y: 0.5 },
    { x: -1, y: -1 }, { x: -1, y: -1 },
  ]
  expect(averageTravelCoordHitRate(coords)).toBeCloseTo(0.5)
})

// ---- Disallowed C-Stick ----

test('banned c-stick file detected', () => {
  const slp = loadSlp('banned_c_stick_analog_player_1.slp')
  const result = hasDisallowedCStickCoords(slp, 0)
  expect(result.result).toBe(true)
  expect(result.details.length).toBeGreaterThan(0)
})

// ---- Goomwave ----

test('legal digital files are not goomwave', () => {
  for (let i = 1; i <= 7; i++) {
    const slp = loadSlp(`legal/digital/potion_p3/potion_${i}.slp`)
    const result = isGoomwave(slp, 2)
    expect(result.result).toBe(false)
  }
})

test('goomwave files detected (full check)', () => {
  const files = loadSlpDir('nonlegal/analog/goomwave/')
  for (const { name, data } of files) {
    const result = isGoomwave(data, 1)
    expect(result.result).toBe(true)
  }
})

test('goomwave clamping detected from coords', () => {
  const slp = loadSlp('nonlegal/analog/goomwave_uptilt_p1.slp')
  const coords = getCoordListFromGame(slp, 0, true)
  expect(hasGoomwaveClamping(coords)).toBe(true)
})

// ---- SDI ----

test('legal SDI files pass', () => {
  const cases = [
    { file: 'legal/digital/sdi/sdi_t20_cardinal_diagonal.slp', player: 0 },
    { file: 'legal/digital/sdi/sdi_mash.slp', player: 0 },
    { file: 'legal/digital/sdi/Game_20250201T233229.slp', player: 0 },
    { file: 'legal/digital/sdi/Game_20250201T232732.slp', player: 1 },
  ]
  for (const { file, player } of cases) {
    const slp = loadSlp(file)
    const result = hasIllegalSDI(slp, player)
    expect(result.result).toBe(false)
  }
})

test('nonlegal SDI files fail', () => {
  const cases = [
    { file: 'nonlegal/digital/sdi/sdi_tas_neutral_cardinal.slp', player: 3 },
    { file: 'nonlegal/digital/sdi/sdi_tas_cardinal_diagonal.slp', player: 3 },
    { file: 'nonlegal/digital/sdi/sdi_unnerfed.slp', player: 3 },
  ]
  for (const { file, player } of cases) {
    const slp = loadSlp(file)
    const result = hasIllegalSDI(slp, player)
    expect(result.result).toBe(true)
  }
})

// ---- Crouch Uptilt ----

test('legal crouch uptilt file passes', () => {
  const slp = loadSlp('legal/digital/crouch_uptilt_r18_v2.slp')
  const result = hasIllegalCrouchUptilt(slp, 0)
  expect(result.result).toBe(false)
})

test('nonlegal crouch uptilt file fails', () => {
  const slp = loadSlp('nonlegal/digital/crouch_uptilt/crouch_uptilt_unnerfed.slp')
  const result = hasIllegalCrouchUptilt(slp, 3)
  expect(result.result).toBe(true)
})

// ---- Handwarmer ----

test('handwarmer files detected', () => {
  const files = loadSlpDir('handwarmers/')
  for (const { name, data } of files) {
    expect(isHandwarmer(data)).toBe(true)
  }
})

test('normal game is not handwarmer', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  expect(isHandwarmer(slp)).toBe(false)
})

// ---- ListChecks ----

test('ListChecks returns 7 checks', () => {
  const checks = ListChecks()
  expect(checks.length).toBe(7)
  for (const check of checks) {
    expect(typeof check.name).toBe('string')
    expect(typeof check.checkFunction).toBe('function')
  }
})

test('ListChecks check functions are callable', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  const checks = ListChecks()
  for (const check of checks) {
    const result = check.checkFunction(slp, 2)
    expect(result).toHaveProperty('result')
    expect(result).toHaveProperty('violations')
  }
})

// ---- Utility Functions ----

test('FloatEquals', () => {
  expect(FloatEquals(0.8, 0.8)).toBe(true)
  expect(FloatEquals(0.8, 0.80009)).toBe(true)
  expect(FloatEquals(0.8, 0.81)).toBe(false)
  expect(FloatEquals(0.8, -0.8)).toBe(false)
})

test('isEqual', () => {
  expect(isEqual({ x: 0.5, y: 0.5 }, { x: 0.5, y: 0.5 })).toBe(true)
  expect(isEqual({ x: 0.5, y: 0.5 }, { x: -0.5, y: 0.5 })).toBe(false)
})

test('getJoystickRegion', () => {
  expect(getJoystickRegion(0, 0)).toBe(JoystickRegion.DZ)
  expect(getJoystickRegion(0.5, 0.5)).toBe(JoystickRegion.NE)
  expect(getJoystickRegion(0.5, -0.5)).toBe(JoystickRegion.SE)
  expect(getJoystickRegion(-0.5, -0.5)).toBe(JoystickRegion.SW)
  expect(getJoystickRegion(-0.5, 0.5)).toBe(JoystickRegion.NW)
  expect(getJoystickRegion(0, 0.5)).toBe(JoystickRegion.N)
  expect(getJoystickRegion(0.5, 0)).toBe(JoystickRegion.E)
  expect(getJoystickRegion(0, -0.5)).toBe(JoystickRegion.S)
  expect(getJoystickRegion(-0.5, 0)).toBe(JoystickRegion.W)
})

test('getUniqueCoords deduplicates', () => {
  const coords: Coord[] = [
    { x: 0, y: 0 },
    { x: 1, y: 1 },
    { x: 0, y: 0 },
    { x: 1, y: 1 },
    { x: 0.5, y: 0.5 },
  ]
  const unique = getUniqueCoords(coords)
  expect(unique.length).toBe(3)
})

test('getTargetCoords returns coords held 2+ frames', () => {
  const coords: Coord[] = [
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 1 },
    { x: 0.5, y: 0.5 },
    { x: 0.5, y: 0.5 },
  ]
  const targets = getTargetCoords(coords)
  expect(targets.length).toBe(2) // (0,0) and (0.5,0.5)
})

test('isBoxController detects box controller', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  expect(isBoxController(slp, 2)).toBe(true)
})

test('isBoxControllerFromCoords works with coord array', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  const coords = getCoordListFromGame(slp, 2, true)
  expect(isBoxControllerFromCoords(coords)).toBe(true)
})

test('controlStickViz returns coords as evidence', () => {
  const slp = loadSlp('legal/digital/potion_p3/potion_1.slp')
  const result = controlStickViz(slp, 2)
  expect(result.result).toBe(false)
  expect(result.details.length).toBe(1)
  expect(result.details[0].evidence.length).toBeGreaterThan(0)
})

test('invalid input throws error', () => {
  expect(() => {
    analyzeReplay(new Uint8Array([0, 1, 2, 3]), 0)
  }).toThrow()
})
