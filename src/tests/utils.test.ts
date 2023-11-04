import {expect, test} from '@jest/globals';
import {FloatEquals, getTargetCoords, getUniqueCoords, isBoxController, Coord, SlippiGame, FramesType, getCoordListFromGame, toArrayBuffer} from '../index';

import * as fs from 'fs';
import * as path from 'path';

test('Float equals allow a tiny bit of wiggle room', () => {
    expect(FloatEquals(0.8, 0.8)).toEqual(true);
    expect(FloatEquals(0.8, 0.7999)).toEqual(true);
    expect(FloatEquals(-0.7, -0.7000000000001)).toEqual(true);
    expect(FloatEquals(0.8, -0.8)).toEqual(false);
})

test('Get target coords', () => {
    var coords: Coord[] = []
    expect(getTargetCoords(coords).length).toEqual(0)

    coords.push(new Coord(0, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(.5, .5))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))

    var targets = getTargetCoords(coords)
    expect(targets.length).toEqual(1)

    expect(targets[0]).toBeInstanceOf(Coord)
})

test('Get unique coords', () => {
    var coords: Coord[] = []
    expect(getUniqueCoords(coords).length).toEqual(0)

    coords.push(new Coord(0, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(.5, .5))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))

    var targets = getUniqueCoords(coords)
    expect(targets.length).toEqual(4)

    expect(targets[0]).toBeInstanceOf(Coord)
})

test('Is box inputs?', () => {
    var coords: Coord[] = []
    coords.push(new Coord(0, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(.5, .5))
    coords.push(new Coord(1, 0))

    expect(isBoxController(coords)).toEqual(true)

    coords.push(new Coord(0.01, 0.01))
    coords.push(new Coord(0.01, 0.01))

    expect(isBoxController(coords)).toEqual(false)

    // Read from an SLP file now. Confirmed box player
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/box_port_1_Steech_vs_techno_G1.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBe(5)

    // Confirmed GCC player
    data = fs.readFileSync(path.join(__dirname, '../../test_data/gcn_port_3_4_shine_2022_gcn_1.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 3, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeGreaterThan(13)
})

test('Parse replay file correctly', () => {
    const data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/banned_c_stick_analog_player_1.slp'), null);
    const game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()

    var frames: FramesType = game.getFrames()
    for (let frame = -123; frame < 1111; frame++) {
        expect(frames[frame].players[0]?.pre.frame).toBe(frame)
    }

    expect(frames[500].players[0]?.post.internalCharacterId).toBe(0x0A)
})