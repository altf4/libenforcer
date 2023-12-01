import {expect, test} from '@jest/globals';
import {Coord, hasIllegaUptiltRounding, SlippiGame, getCoordListFromGame, toArrayBuffer} from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Test uptilt rounding (legal analog)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/analog/')
    const files: string[] = await fs.promises.readdir(slpDir);
    for(const filename of files ) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoordsPortThree: Coord[] = getCoordListFromGame(game, 2, true)
        let gameCoordsPortFour: Coord[] = getCoordListFromGame(game, 3, true)

        expect(hasIllegaUptiltRounding(game, 2, gameCoordsPortThree)).toBe(false)
        expect(hasIllegaUptiltRounding(game, 3, gameCoordsPortFour)).toBe(false)
    }
})

test('Test uptilt rounding (legal digital)', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/digital/techno_p1/')
    const files: string[] = await fs.promises.readdir(slpDir);
    for(const filename of files ) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoordsPortOne: Coord[] = getCoordListFromGame(game, 0, true)

        expect(hasIllegaUptiltRounding(game, 0, gameCoordsPortOne)).toBe(false)
    }
})