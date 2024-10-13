import {expect, test} from '@jest/globals';
import {Coord, hasIllegalSDI, SlippiGame, getCoordListFromGame, toArrayBuffer} from '../index';
import {SDIRegion, isDiagonalAdjacent, failsSDIRuleOne, failsSDIRuleTwo, failsSDIRuleThree} from '../sdi';
import * as fs from 'fs';
import * as path from 'path';

test('Test SDI from legal digital file', () => {
    // for (let i=1; i <= 7; i++) { 
        let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + 2 + '.slp'), null);
        let game = new SlippiGame(toArrayBuffer(data))

        // let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + 2 + '.slp'), null);
        // let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)

        expect(failsSDIRuleOne(gameCoords)).toBe(false)

        expect(failsSDIRuleThree(gameCoords)).toBe(false)
    // }
})


test('Test isDiagonalAdjacent()', async () => {
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.NW)).toBe(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.SE)).toBe(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.DZ)).toBe(false)
    expect(isDiagonalAdjacent(SDIRegion.SE, SDIRegion.SW)).toBe(true)
    expect(isDiagonalAdjacent(SDIRegion.NW, SDIRegion.NE)).toBe(true)
    expect(isDiagonalAdjacent(SDIRegion.SW, SDIRegion.SE)).toBe(true)
})

test('Test SDI (legal)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_r18_v2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalSDI(game, 0, coords)).toBe(false)
})

test('Test SDI (non legal A)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_UNNERFED_neutral_cardinal.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #1
    // expect(hasIllegalSDI(game, 3, coords)).toBe(true)
    expect(failsSDIRuleOne(coords)).toBe(true)
})

test('Test SDI (non legal B)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_UNNERFED_cardinal_diagonal.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #1
    // expect(hasIllegalSDI(game, 3, coords)).toBe(true)
    expect(failsSDIRuleTwo(coords)).toBe(true)
})

test('Test SDI (non legal C)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/sdi/sdi_unnerfed.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    // Fails rule #3 but not rule #1
    expect(hasIllegalSDI(game, 3, coords)).toBe(true)
    expect(failsSDIRuleOne(coords)).toBe(false)
    expect(failsSDIRuleThree(coords)).toBe(true)
})