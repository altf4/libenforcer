import {expect, test} from '@jest/globals';
import {Coord, averageTravelCoordHitRate, SlippiGame, getCoordListFromGame, toArrayBuffer, hasIllegalTravelTime} from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Test average travel time', () => {
    // AAA BB C DD
    // Three targets, one travel
    // Two conversions between travel points. Thus 50%
    var coords: Coord[] = []
    coords.push({x: 0, y: 0})
    coords.push({x: 0, y: 0})
    coords.push({x: 0, y: 0})
    coords.push({x: 1, y: 1})
    coords.push({x: 1, y: 1})
    coords.push({x: 0.5, y: 0.5})
    coords.push({x: -1, y: -1})
    coords.push({x: -1, y: -1})

    expect(averageTravelCoordHitRate(coords)).toBeCloseTo(0.5);
})

test('Test average travel time from legal digital file', () => {
    for (let i=1; i <= 7; i++) {
        let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + i + '.slp'), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)

        expect(hasIllegalTravelTime(game, 2, gameCoords).result).toEqual(false)
        expect(averageTravelCoordHitRate(gameCoords)).toBeGreaterThan(0.30)
    }
})

test('Test average travel time from legal analog file', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/')
    const files: string[] = await fs.promises.readdir(slpDir);
    for(const filename of files ) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)

        expect(hasIllegalTravelTime(game, 2, gameCoords).result).toEqual(false)
        expect(averageTravelCoordHitRate(gameCoords)).toBeGreaterThan(0.85)
    }
})

test('Test average travel time from non-legal digital file', async () => {
    const slpDir = path.join(__dirname, '../../test_data/nonlegal/digital/pre-ruleset/')
    const files: string[] = await fs.promises.readdir(slpDir);
    for(const filename of files ) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 3, true)

        expect(hasIllegalTravelTime(game, 2, gameCoords).result).toEqual(true)
        expect(averageTravelCoordHitRate(gameCoords)).toBeLessThan(0.20)
    }
})