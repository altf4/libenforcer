import {expect, test} from '@jest/globals';
import {Coord, hasIllegalSOCD, SlippiGame, toArrayBuffer, getCoordListFromGame} from '../index';
import * as fs from 'fs';
import * as path from 'path';


test('Known bad digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/pre-ruleset/p4_Game_002147A7254F_20231021T074705.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 3, true)
    expect(hasIllegalSOCD(game, 3, gameCoords)).toBe(false);
})

test('Known bad digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/pre-ruleset/p4_Game_002147A7254F_20231021T075046.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 3, true)
    expect(hasIllegalSOCD(game, 3, gameCoords)).toBe(false);
})

test('Known bad digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/pre-ruleset/p4_Game_002147A7254F_20231021T075432.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 3, true)
    expect(hasIllegalSOCD(game, 3, gameCoords)).toBe(false);
})

test('Known good digital SOCD. TIGHT DASH DANCES', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/dash_dances/tight_dash_dances_box_p2_1.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 1, true)
    expect(hasIllegalSOCD(game, 1, gameCoords)).toBe(false);
})

test('Known good digital SOCD. TIGHT DASH DANCES', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/dash_dances/tight_dash_dances_box_p1_2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    expect(hasIllegalSOCD(game, 0, gameCoords)).toBe(false);
})

test('Known good SOCD. Analog', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/analog/Game_8C56C529AEAA_20231022T181554.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 3, true)
    expect(hasIllegalSOCD(game, 3, gameCoords)).toBe(false);
})

test('Known good digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_1.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)
    expect(hasIllegalSOCD(game, 2, gameCoords)).toBe(false);
})

test('Known good digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)
    expect(hasIllegalSOCD(game, 2, gameCoords)).toBe(false);
})

test('Known good digital SOCD', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_3.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)
    expect(hasIllegalSOCD(game, 2, gameCoords)).toBe(false);
})