import { expect, test, beforeAll } from '@jest/globals'
import * as fs from 'node:fs'
import * as path from 'node:path'
import { fileURLToPath } from 'node:url'
import init, {
  SlpGame,
  isBoxControllerFromCoords,
  FloatEquals,
  getJoystickRegion,
  JoystickRegion,
  getUniqueCoords,
  getTargetCoords,
  averageTravelCoordHitRate,
  getCStickViolations,
  hasGoomwaveClamping,
  processAnalogStick,
  isEqual,
  Coord,
  PlayerAnalysis,
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

// ---- analyzePlayer ----

test('analyzePlayer returns structured results for box controller', () => {
  const game = new SlpGame(loadSlp('legal/digital/potion_p3/potion_1.slp'))
  const result: PlayerAnalysis = game.analyzePlayer(2)

  expect(result.controller_type).toBe('Box')
  expect(typeof result.is_legal).toBe('boolean')

  // Box checks should be populated
  expect(result.travel_time).not.toBeNull()
  expect(result.disallowed_cstick).not.toBeNull()
  expect(result.crouch_uptilt).not.toBeNull()
  expect(result.sdi).not.toBeNull()
  expect(result.input_fuzzing).not.toBeNull()

  // Analog checks should be undefined for box controller
  expect(result.goomwave).toBeUndefined()
  expect(result.uptilt_rounding).toBeUndefined()

  expect(result.travel_time!.result).toBe(false)
  expect(result.sdi!.result).toBe(false)

  game.free()
})

test('analyzePlayer returns structured results for analog controller', () => {
  const game = new SlpGame(loadSlp('legal/analog/traveltime/Game_8C56C529AEAA_20231022T181554.slp'))
  const result: PlayerAnalysis = game.analyzePlayer(3)

  expect(result.controller_type).toBe('Analog')
  expect(typeof result.is_legal).toBe('boolean')

  // Box checks should be undefined for analog controller
  expect(result.travel_time).toBeUndefined()
  expect(result.disallowed_cstick).toBeUndefined()
  expect(result.crouch_uptilt).toBeUndefined()
  expect(result.sdi).toBeUndefined()
  expect(result.input_fuzzing).toBeUndefined()

  // Analog checks should be populated
  expect(result.goomwave).not.toBeNull()
  expect(result.uptilt_rounding).not.toBeNull()

  game.free()
})

// ---- Travel Time ----

test('legal digital files pass travel time', () => {
  for (let i = 1; i <= 7; i++) {
    const game = new SlpGame(loadSlp(`legal/digital/potion_p3/potion_${i}.slp`))
    const result = game.analyzePlayer(2)
    expect(result.travel_time!.result).toBe(false)
    game.free()
  }
})

test('legal digital files have travel hit rate > 0.30', () => {
  for (let i = 1; i <= 7; i++) {
    const game = new SlpGame(loadSlp(`legal/digital/potion_p3/potion_${i}.slp`))
    const coords = game.getMainStickCoords(2)
    expect(averageTravelCoordHitRate(coords)).toBeGreaterThan(0.30)
    game.free()
  }
})

test('nonlegal digital files fail travel time', () => {
  const files = loadSlpDir('nonlegal/digital/pre-ruleset/')
  for (const { name, data } of files) {
    const game = new SlpGame(data)
    const result = game.analyzePlayer(3)
    expect(result.travel_time!.result).toBe(true)
    game.free()
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
  const game = new SlpGame(loadSlp('banned_c_stick_analog_player_1.slp'))
  const result = game.analyzePlayer(0)
  expect(result.disallowed_cstick!.result).toBe(true)
  expect(result.disallowed_cstick!.details.length).toBeGreaterThan(0)
  game.free()
})

// ---- Goomwave ----

test('legal digital files are not goomwave', () => {
  for (let i = 1; i <= 7; i++) {
    const game = new SlpGame(loadSlp(`legal/digital/potion_p3/potion_${i}.slp`))
    const result = game.analyzePlayer(2)
    // Box controller - goomwave should be undefined
    expect(result.goomwave).toBeUndefined()
    game.free()
  }
})

test('goomwave files detected (full check)', () => {
  const files = loadSlpDir('nonlegal/analog/goomwave/')
  for (const { name, data } of files) {
    const game = new SlpGame(data)
    const result = game.analyzePlayer(1)
    expect(result.goomwave!.result).toBe(true)
    game.free()
  }
})

test('goomwave clamping detected from coords', () => {
  const game = new SlpGame(loadSlp('nonlegal/analog/goomwave_uptilt_p1.slp'))
  const coords = game.getMainStickCoords(0)
  expect(hasGoomwaveClamping(coords)).toBe(true)
  game.free()
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
    const game = new SlpGame(loadSlp(file))
    const result = game.analyzePlayer(player)
    expect(result.sdi!.result).toBe(false)
    game.free()
  }
})

test('nonlegal SDI files fail', () => {
  const cases = [
    { file: 'nonlegal/digital/sdi/sdi_tas_neutral_cardinal.slp', player: 3 },
    { file: 'nonlegal/digital/sdi/sdi_tas_cardinal_diagonal.slp', player: 3 },
    { file: 'nonlegal/digital/sdi/sdi_unnerfed.slp', player: 3 },
  ]
  for (const { file, player } of cases) {
    const game = new SlpGame(loadSlp(file))
    const result = game.analyzePlayer(player)
    expect(result.sdi!.result).toBe(true)
    game.free()
  }
})

// ---- Crouch Uptilt ----

test('legal crouch uptilt file passes', () => {
  const game = new SlpGame(loadSlp('legal/digital/crouch_uptilt_r18_v2.slp'))
  const result = game.analyzePlayer(0)
  expect(result.crouch_uptilt!.result).toBe(false)
  game.free()
})

test('nonlegal crouch uptilt file fails', () => {
  const game = new SlpGame(loadSlp('nonlegal/digital/crouch_uptilt/crouch_uptilt_unnerfed.slp'))
  const result = game.analyzePlayer(3)
  expect(result.crouch_uptilt!.result).toBe(true)
  game.free()
})

// ---- Handwarmer ----

test('handwarmer files detected', () => {
  const files = loadSlpDir('handwarmers/')
  for (const { name, data } of files) {
    const game = new SlpGame(data)
    expect(game.isHandwarmer()).toBe(true)
    game.free()
  }
})

test('normal game is not handwarmer', () => {
  const game = new SlpGame(loadSlp('legal/digital/potion_p3/potion_1.slp'))
  expect(game.isHandwarmer()).toBe(false)
  game.free()
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
  const game = new SlpGame(loadSlp('legal/digital/potion_p3/potion_1.slp'))
  expect(game.isBoxController(2)).toBe(true)
  game.free()
})

test('isBoxControllerFromCoords works with coord array', () => {
  const game = new SlpGame(loadSlp('legal/digital/potion_p3/potion_1.slp'))
  const coords = game.getMainStickCoords(2)
  expect(isBoxControllerFromCoords(coords)).toBe(true)
  game.free()
})

test('getMainStickCoords returns coords', () => {
  const game = new SlpGame(loadSlp('legal/digital/potion_p3/potion_1.slp'))
  const coords = game.getMainStickCoords(2)
  expect(coords.length).toBeGreaterThan(0)
  expect(coords[0]).toHaveProperty('x')
  expect(coords[0]).toHaveProperty('y')
  game.free()
})

test('invalid input throws error', () => {
  expect(() => {
    new SlpGame(new Uint8Array([0, 1, 2, 3]))
  }).toThrow()
})
