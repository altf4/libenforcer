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

    coords.push({x: 0, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 0.5, y: 0.5})
    coords.push({x: 1, y: 0.5})
    coords.push({x: 1, y: 0.5})
    coords.push({x: 1, y: 0.5})

    var targets = getTargetCoords(coords)
    expect(targets.length).toEqual(2)
})

test('Get unique coords', () => {
    var coords: Coord[] = []
    expect(getUniqueCoords(coords).length).toEqual(0)

    coords.push({x: 0, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 0.5, y: 0.5})
    coords.push({x: 1, y: 0.5})
    coords.push({x: 1, y: 0.5})
    coords.push({x: 1, y: 0.5})

    var targets = getUniqueCoords(coords)
    expect(targets.length).toEqual(5)
})

test('Is box inputs?', () => {
    var coords: Coord[] = []
    coords.push({x: 0, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: -1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 1, y: 0})
    coords.push({x: 0.5, y: 0.5})
    coords.push({x: 1, y: 0})

    expect(isBoxController(coords)).toEqual(true)

    coords.push({x: 0.01, y: 0.01})
    coords.push({x: 0.01, y: 0.01})

    expect(isBoxController(coords)).toEqual(false)

    // Game 1. Read from an SLP file now. Confirmed box player
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G1.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeLessThanOrEqual(13)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 2
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G2.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeLessThanOrEqual(13)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 3
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G3.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeLessThanOrEqual(13)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 4
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G4.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeLessThanOrEqual(13)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 5
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G5.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeLessThanOrEqual(13)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Confirmed GCC player
    data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/analog/Game_8C56C529AEAA_20231022T181554.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 3, false)
    expect(gameCoords.length).toBeGreaterThan(0)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeGreaterThan(13)
    expect(isBoxController(gameCoords)).toEqual(false)
})

test('Is box inputs? Potion dataset', () => {
    for (let i=1; i <= 7; i++) {
        var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + i + '.slp'), null);
        var game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        var gameCoords: Coord[] = getCoordListFromGame(game, 2, false)
        var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
        expect(uniqueCoords.length).toBeLessThanOrEqual(13)
        expect(isBoxController(gameCoords)).toEqual(true)
    }
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