import { expect, test } from '@jest/globals'
import { Coord, hasIllegalSDI, SlippiGame, getCoordListFromGame, toArrayBuffer, Violation } from '../index'
import { SDIRegion, getSDIRegion, isDiagonalAdjacent, failsSDIRuleOne, failsSDIRuleTwo, failsSDIRuleThree } from '../sdi'
import * as fs from 'fs'
import * as path from 'path'

test('Test Region Sanity Check', () => {
    // DZ - Dead Zone Test
    expect(getSDIRegion(0, 0)).toBe(SDIRegion.DZ)
    expect(getSDIRegion(0.2, 0.2)).toBe(SDIRegion.DZ)
    expect(getSDIRegion(-0.2, -0.2)).toBe(SDIRegion.DZ)

    // NE - Northeast
    expect(getSDIRegion(0.8, 0.8)).toBe(SDIRegion.NE)
    expect(getSDIRegion(0.9, 0.7)).toBe(SDIRegion.NE)

    // SE - Southeast
    expect(getSDIRegion(0.8, -0.8)).toBe(SDIRegion.SE)
    expect(getSDIRegion(0.9, -0.7)).toBe(SDIRegion.SE)

    // SW - Southwest
    expect(getSDIRegion(-0.8, -0.8)).toBe(SDIRegion.SW)
    expect(getSDIRegion(-0.9, -0.7)).toBe(SDIRegion.SW)

    // NW - Northwest
    expect(getSDIRegion(-0.8, 0.8)).toBe(SDIRegion.NW)
    expect(getSDIRegion(-0.9, 0.7)).toBe(SDIRegion.NW)

    // N - North
    expect(getSDIRegion(0, 0.8)).toBe(SDIRegion.N)
    expect(getSDIRegion(0.2, 0.9)).toBe(SDIRegion.N)

    // E - East
    expect(getSDIRegion(0.8, 0)).toBe(SDIRegion.E)
    expect(getSDIRegion(0.9, 0.2)).toBe(SDIRegion.E)

    // S - South
    expect(getSDIRegion(0, -0.8)).toBe(SDIRegion.S)
    expect(getSDIRegion(0.2, -0.9)).toBe(SDIRegion.S)

    // W - West
    expect(getSDIRegion(-0.8, 0)).toBe(SDIRegion.W)
    expect(getSDIRegion(-0.9, 0.2)).toBe(SDIRegion.W)

    // TILT - Middle-ish area
    expect(getSDIRegion(0.4, 0.4)).toBe(SDIRegion.TILT)
    expect(getSDIRegion(-0.4, -0.4)).toBe(SDIRegion.TILT)
    expect(getSDIRegion(0.2, -0.3)).toBe(SDIRegion.TILT)

    // Edge cases
    // Testing near the edges of the DZ boundary (0.2875)
    expect(getSDIRegion(0.2876, 0)).toBe(SDIRegion.TILT)
    expect(getSDIRegion(0, 0.2876)).toBe(SDIRegion.TILT)
    expect(getSDIRegion(-0.2876, 0)).toBe(SDIRegion.TILT)
    expect(getSDIRegion(0, -0.2876)).toBe(SDIRegion.TILT)

    // Test values on the boundary of magnitude 0.7 (diagonal)
    expect(getSDIRegion(0.7, 0.7)).toBe(SDIRegion.NE)
    expect(getSDIRegion(-0.7, 0.7)).toBe(SDIRegion.NW)
    expect(getSDIRegion(0.7, -0.7)).toBe(SDIRegion.SE)
    expect(getSDIRegion(-0.7, -0.7)).toBe(SDIRegion.SW)

    // Boundary checks for cardinal directions (0.7)
    expect(getSDIRegion(0.7, 0)).toBe(SDIRegion.E)
    expect(getSDIRegion(0, 0.7)).toBe(SDIRegion.N)
    expect(getSDIRegion(-0.7, 0)).toBe(SDIRegion.W)
    expect(getSDIRegion(0, -0.7)).toBe(SDIRegion.S)
})

test('Test SDI from legal digital file', () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + 2 + '.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))

    expect(game).not.toBeNull()
    let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)

    expect(failsSDIRuleOne(gameCoords)).toEqual([])
    expect(failsSDIRuleThree(gameCoords)).toEqual([])
})

test('Test isDiagonalAdjacent()', async () => {
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.NW)).toEqual(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.SE)).toEqual(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.DZ)).toEqual(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.SW)).toEqual(true)
    expect(isDiagonalAdjacent(SDIRegion.NW, SDIRegion.NE)).toEqual(true)
    expect(isDiagonalAdjacent(SDIRegion.SW, SDIRegion.SE)).toEqual(true)
})

test('Test SDI (manual coords)', async () => {
    // Sanity check. No movement. No violation
    let coords: Coord[] = [{ x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }]
    expect(failsSDIRuleOne(coords)).toEqual([])

    // Easy case. Lots of SDI. Violation.
    coords = [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 1, y: 0 }]
    expect(failsSDIRuleOne(coords).length).toBeGreaterThanOrEqual(1)

    // Too many frames in the tilt zone, doesn't count as SDI!
    coords = [{ x: 0, y: 0 }, { x: 0.3, y: 0 }, { x: 0.32, y: 0 }, { x: 0.35, y: 0 }, { x: 0.4, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 1, y: 0 }]
    expect(failsSDIRuleOne(coords)).toEqual([])

    // Slowest possible SDIs. Doesn't count as SDI since it has travel time
    coords = [{ x: 0, y: 0 }, { x: 0.3, y: 0 }, { x: 0.35, y: 0 }, { x: 0.4, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 0.35, y: 0 }, { x: 0.4, y: 0 }, { x: 1, y: 0 }]
    expect(failsSDIRuleOne(coords).length).toEqual(0)

    // Doesn't count as SDI since it has travel time
    coords = [{ x: 0, y: 0 }, { x: 0.3, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 0.3, y: 0 }, { x: 1, y: 0 }]
    expect(failsSDIRuleOne(coords).length).toEqual(0)

    // Violation. SDI, then put in a bunch of travel time after. 
    // IE: You don't get off the hook just because you have travel time after the SDI
    coords = [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0.3, y: 0 }, { x: 0.35, y: 0 }, { x: 0.4, y: 0 }]
    expect(failsSDIRuleOne(coords).length).toBeGreaterThanOrEqual(1)
})


test('Test SDI (legal A)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_r18_v2.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    // TODO: Re-enable these tests once we get an SLP that works
    // expect(hasIllegalSDI(game, 0, coords)).toEqual([])
    expect(failsSDIRuleOne(coords)).toEqual([])
    // expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_t20_cardinal_diagonal.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalSDI(game, 0, coords).result).toEqual(false)
    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})


test('Test SDI (legal C)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_mash.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalSDI(game, 0, coords).result).toEqual(false)
    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (legal D)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/Game_20250201T233229.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (legal E)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/Game_20250201T232732.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 1, true)

    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (non legal A)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_tas_neutral_cardinal.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #1
    expect(hasIllegalSDI(game, 3, coords).result).toEqual(true)

    let violations: Violation[] = failsSDIRuleOne(coords)
    expect(violations.length).toEqual(195)
    expect(violations[10].reason).toEqual("Failed SDI rule #1")
    expect(violations[10].metric).toEqual(139)
    expect(violations[10].evidence).toEqual([{ "x": 0, "y": 0 }, { "x": 0, "y": 0 }, { "x": -1, "y": 0 }, { "x": 0, "y": 0 }, { "x": -1, "y": 0 }, { "x": 0, "y": 0 }, { "x": -1, "y": 0 }, { "x": 0, "y": 0 }, { "x": -1, "y": 0 }, { "x": 0, "y": 0 }])
})

test('Test SDI (non legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_tas_cardinal_diagonal.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    let violations: Violation[] = failsSDIRuleTwo(coords)
    expect(violations.length).toEqual(36)
    expect(violations[10].reason).toEqual("Failed SDI rule #2")
    expect(violations[10].metric).toEqual(226)
    expect(violations[10].evidence).toEqual([{ "x": 1, "y": 0 }, { "x": 0.7, "y": 0.7 }, { "x": 1, "y": 0 }, { "x": 0.7, "y": 0.7 }, { "x": 1, "y": 0 }])
})

test('Test SDI (non legal C)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_unnerfed.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #3 but not rule #1
    expect(hasIllegalSDI(game, 3, coords).result).toEqual(true)
    expect(failsSDIRuleOne(coords)).toEqual([])

    let violations: Violation[] = failsSDIRuleThree(coords)
    expect(violations.length).toEqual(8)
    expect(violations[4].reason).toEqual("Failed SDI rule #3")
    expect(violations[4].metric).toEqual(2535)
    expect(violations[4].evidence).toEqual([{ "x": 0.7, "y": 0.7 }, { "x": 0.7, "y": 0.7 }, { "x": 0, "y": 1 }, { "x": -0.7, "y": 0.7 }, { "x": 0.7, "y": 0.7 }])
})