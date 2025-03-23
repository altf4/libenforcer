import { expect, test } from '@jest/globals';
import { Coord, hasIllegalUptiltRounding, SlippiGame, getCoordListFromGame, toArrayBuffer, CheckResult } from '../index';
import { getUptiltCheck } from '../uptilt_rounding'
import * as fs from 'fs';
import * as path from 'path';

test('Test uptilt rounding (legal analog A)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/traveltime/')
    const files: string[] = await fs.promises.readdir(slpDir);
    for (const filename of files) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoordsPortThree: Coord[] = getCoordListFromGame(game, 2, true)
        let gameCoordsPortFour: Coord[] = getCoordListFromGame(game, 3, true)

        expect(hasIllegalUptiltRounding(game, 2, gameCoordsPortThree).result).toEqual(false)
        expect(hasIllegalUptiltRounding(game, 3, gameCoordsPortFour).result).toEqual(false)
    }
})

test('Test uptilt rounding (nonlegal analog)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/nonlegal/analog/goomwave_uptilt_p1.slp')
    let data = fs.readFileSync(slpDir, null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

    let checkResult: CheckResult = getUptiltCheck(game, 0, gameCoordsPortOne)
    expect(checkResult.result).toEqual(true)
    expect(checkResult.violations.length).toBeGreaterThanOrEqual(1)
})

test('Test uptilt rounding (legal analog B)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/Game_20250107T140347.slp')
    let data = fs.readFileSync(slpDir, null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

    let checkResult: CheckResult = hasIllegalUptiltRounding(game, 0, gameCoordsPortOne)
    expect(checkResult.result).toEqual(false)
    expect(checkResult.violations.length).toEqual(0)
})

test('Test uptilt rounding (legal analog C)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/Game_20250107T142211.slp')
    let data = fs.readFileSync(slpDir, null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

    let checkResult: CheckResult = hasIllegalUptiltRounding(game, 0, gameCoordsPortOne)
    expect(checkResult.result).toEqual(false)
    expect(checkResult.violations.length).toEqual(0)
})

test('Test uptilt rounding (legal analog D)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/Game_20250123T212056.slp')
    let data = fs.readFileSync(slpDir, null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

    let checkResult: CheckResult = hasIllegalUptiltRounding(game, 0, gameCoordsPortOne)
    expect(checkResult.result).toEqual(false)
    expect(checkResult.violations.length).toEqual(0)
})