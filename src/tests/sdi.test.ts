import {expect, test} from '@jest/globals';
import {Coord, hasIllegalSDI, SlippiGame, getCoordListFromGame, toArrayBuffer, Violation} from '../index';
import {SDIRegion, isDiagonalAdjacent, failsSDIRuleOne, failsSDIRuleTwo, failsSDIRuleThree} from '../sdi';
import * as fs from 'fs';
import * as path from 'path';

test('Test SDI from legal digital file', () => {
        let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + 2 + '.slp'), null);
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

test('Test SDI (legal A)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_r18_v2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    // TODO: Re-enable these tests once we get an SLP that works
    // expect(hasIllegalSDI(game, 0, coords)).toEqual(false)
    expect(failsSDIRuleOne(coords)).toEqual([])
    // expect(failsSDIRuleTwo(coords)).toEqual(false)
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_t20_cardinal_diagonal.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalSDI(game, 0, coords).result).toEqual(false)
    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})

test('Test SDI (legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_mash.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalSDI(game, 0, coords).result).toEqual(false)
    expect(failsSDIRuleOne(coords)).toEqual([])
    expect(failsSDIRuleTwo(coords)).toEqual([])
    expect(failsSDIRuleThree(coords)).toEqual([])
})


// TODO: This test case correctly fails. This appears to be a mistake in the controller firmware that allows too many SDI inputs
// test('Test SDI (legal C)', async () => {
//     let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_t20_neutral_cardinal.slp'), null);
//     let game = new SlippiGame(toArrayBuffer(data))
//     expect(game).not.toBeNull()
//     let coords: Coord[] = getCoordListFromGame(game, 0, true)

//     expect(hasIllegalSDI(game, 0, coords)).toEqual(false)
// })

test('Test SDI (non legal A)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_tas_neutral_cardinal.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #1
    expect(hasIllegalSDI(game, 3, coords).result).toEqual(true)

    let violations: Violation[] = failsSDIRuleOne(coords)
    expect(violations.length).toEqual(57)
    expect(violations[10].reason).toEqual("Failed SDI rule #1")
    expect(violations[10].metric).toEqual(228)
    expect(violations[10].evidence).toEqual([{"x": 0, "y": 0}, {"x": 1, "y": 0}, {"x": 1, "y": 0}, {"x": 0, "y": 0}, {"x": 1, "y": 0}, {"x": 0, "y": 0}])
})

test('Test SDI (non legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_tas_cardinal_diagonal.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    let violations: Violation[] = failsSDIRuleTwo(coords)
    expect(violations.length).toEqual(48)
    expect(violations[10].reason).toEqual("Failed SDI rule #2")
    expect(violations[10].metric).toEqual(226)
    expect(violations[10].evidence).toEqual([{"x": 1, "y": 0}, {"x": 0.7, "y": 0.7}, {"x": 1, "y": 0}, {"x": 0.7, "y": 0.7}, {"x": 1, "y": 0}, {"x": 0.7, "y": 0.7}])
})

test('Test SDI (non legal C)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_unnerfed.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #3 but not rule #1
    expect(hasIllegalSDI(game, 3, coords).result).toEqual(true)
    expect(failsSDIRuleOne(coords)).toEqual([])
    
    let violations: Violation[] = failsSDIRuleThree(coords)
    expect(violations.length).toEqual(27)
    expect(violations[10].reason).toEqual("Failed SDI rule #3")
    expect(violations[10].metric).toEqual(1659)
    expect(violations[10].evidence).toEqual([{"x": 0.7, "y": 0.7}, {"x": 0.7, "y": 0.7}, {"x": 0, "y": 1}, {"x": -0.7, "y": 0.7}, {"x": 0.7, "y": 0.7}, {"x": 0.7, "y": 0.7}])

})