import { expect, test } from '@jest/globals';
import { Coord, controlStickViz, SlippiGame, getCoordListFromGame, toArrayBuffer, CheckResult } from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Test control stick viz (Sanity check)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/nonlegal/analog/goomwave_uptilt_p1.slp')
    let data = fs.readFileSync(slpDir, null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

    let checkResult: CheckResult = controlStickViz(game, 0, gameCoordsPortOne)
    expect(checkResult.result).toEqual(false)
    expect(checkResult.violations[0].evidence.length).toEqual(4845)
})
