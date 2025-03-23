import { expect, test } from '@jest/globals';
import { Coord, SlippiGame, getCoordListFromGame, toArrayBuffer } from '../index';
import { hasIllegalCrouchUptilt } from '../crouch_uptilt';
import * as fs from 'fs';
import * as path from 'path';

test('Test Crouch Uptilt (nonlegal)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/nonlegal/digital/crouch_uptilt/crouch_uptilt_unnerfed.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 3, true)

    let checkResult = hasIllegalCrouchUptilt(game, 3, coords)
    expect(checkResult.result).toEqual(true)
    expect(checkResult.violations.length).toEqual(6)

    for (let violation of checkResult.violations) {
        expect(violation.evidence[0]).toEqual({ "x": 0, "y": -1 })
    }
})

test('Test Crouch Uptilt (legal)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/crouch_uptilt_r18_v2.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalCrouchUptilt(game, 0, coords).result).toEqual(false)
})

// Doubles sometimes has blank entries for players. Handle this without crashing
test('Test Crouch Uptilt (breaking?)', async () => {
    let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/doubles_with_blank_player.slp'), null);
    let game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    let coords: Coord[] = getCoordListFromGame(game, 0, true)

    expect(hasIllegalCrouchUptilt(game, 0, coords).result).toEqual(false)
})