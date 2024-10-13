import {expect, test} from '@jest/globals';
import {Coord, SlippiGame, getCoordListFromGame, toArrayBuffer} from '../index';
import {hasIllegalCrouchUptilt} from '../crouch_uptilt';
import * as fs from 'fs';
import * as path from 'path';

test('Test SDI (nonlegal)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/crouch_uptilt/crouch_uptilt_unnerfed.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    expect(hasIllegalCrouchUptilt(game, 3, coords)).toBe(true)
})

test('Test SDI (legal)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/sdi/sdi_r18_v2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalCrouchUptilt(game, 0, coords)).toBe(false)
})