import {expect, test} from '@jest/globals';
import {Coord, averageTravelCoordHitRate, SlippiGame, getCoordListFromGame, toArrayBuffer, isGoomwave} from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Test isGoomwave (negative)', () => {
    for (let i=1; i <= 7; i++) {
        let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + i + '.slp'), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)
        expect(isGoomwave(game, 2, gameCoords).result).toEqual(false)
    }
})

test('Test isGoomwave A (positive)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/analog/goomwave_uptilt_p1.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(isGoomwave(game, 0, coords).result).toEqual(true)
})


test('Test isGoomwave B (positive)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/analog/goomwave/Game_20250216T194607.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 1, true)

    expect(isGoomwave(game, 1, coords).result).toEqual(true)
})

test('Test isGoomwave C (positive)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/analog/goomwave/Game_20250216T194746.slp'), null)
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 1, true)

    expect(isGoomwave(game, 1, coords).result).toEqual(true)
})