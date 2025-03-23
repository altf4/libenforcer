import { expect, test } from '@jest/globals';
import { Coord, SlippiGame, getCoordListFromGame, hasDisallowedCStickCoords, toArrayBuffer, getCStickViolations, isBoxController } from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Should pass check for disallowed C-Stick values', () => {
    let coords: Coord[] = []
    coords.push({ x: 0, y: 0 })
    coords.push({ x: 1, y: 1 })

    const result = getCStickViolations(coords)
    expect(result.length).toEqual(0);
})

test('Should trigger check for disallowed C-Stick values', () => {
    let coords: Coord[] = []
    coords.push({ x: 0.8, y: 0 })

    const result = getCStickViolations(coords)
    expect(result.length).toEqual(1);
})

test('Should trigger check for disallowed C-Stick values', () => {
    let coords: Coord[] = []
    coords.push({ x: 0.6625, y: 0 })

    const result = getCStickViolations(coords)
    expect(result.length).toEqual(1);
})

test('Test full game with disallowed C-stick value', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/banned_c_stick_analog_player_1.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    expect(hasDisallowedCStickCoords(game, 0, gameCoords).result).toEqual(true)
})

test('Test full game with analog controller, should pass', () => {
    let data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/legal/analog/traveltime/Game_8C56C529AEAA_20231022T181554.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()
    let gameCoords: Coord[] = getCoordListFromGame(game, 3, false)
    expect(hasDisallowedCStickCoords(game, 3, gameCoords).result).toEqual(false)
})