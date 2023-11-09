import {expect, test} from '@jest/globals';
import {FloatEquals, getTargetCoords, getUniqueCoords, isBoxController, Coord, SlippiGame, FramesType, getCoordListFromGame, toArrayBuffer, processAnalogStick, isEqual, getJoystickRegion, isHandwarmer} from '../index';

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
    // Game 1. Read from an SLP file now. Confirmed box player
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G1.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 2
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G2.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 3
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G3.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 4
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G4.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Game 5
    var data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G5.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 0, true)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(isBoxController(gameCoords)).toEqual(true)

    // Confirmed GCC player
    data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/analog/Game_8C56C529AEAA_20231022T181554.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).not.toBeNull()
    var gameCoords: Coord[] = getCoordListFromGame(game, 3, true)
    expect(gameCoords.length).toBeGreaterThan(0)
    var uniqueCoords: Coord[] = getUniqueCoords(gameCoords)
    expect(uniqueCoords.length).toBeGreaterThan(13)
    expect(isBoxController(gameCoords)).toEqual(false)
})

test('Is box inputs? Potion dataset', () => {
    for (let i=1; i <= 7; i++) {
        let data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/potion_p3/potion_' + i + '.slp'), null);
        let game = new SlippiGame(toArrayBuffer(data))
        expect(game).not.toBeNull()
        let gameCoords: Coord[] = getCoordListFromGame(game, 2, true)
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

test('Process main stick inputs', () => {

    expect(isEqual(processAnalogStick({x: 0, y: 0}, false), {x: 0, y: 0})).toBe(true)
    expect(isEqual(processAnalogStick({x: 0, y: -80}, false), {x: 0, y: -1})).toBe(true)

    const data = fs.readFileSync(path.join(__dirname, '../../test_data/legal/digital/techno_p1/Steech_vs_techno_G1.slp'), null);
    const game = new SlippiGame(toArrayBuffer(data))
    var frames: FramesType = game.getFrames()
    for (let frame = -123; frame < 1794; frame++) {
        let x = frames[frame].players[0]?.pre.rawJoystickX
        let y = frames[frame].players[0]?.pre.rawJoystickY
        let rawCoord: Coord = processAnalogStick({x: x, y:y}, true)
        let processedX = frames[frame].players[0]?.pre.joystickX
        let processedY = frames[frame].players[0]?.pre.joystickY
        let processedCoord = {x: processedX, y: processedY}

        expect(isEqual(processedCoord, rawCoord)).toBe(true)
    }
})

test('Is handwarmer?', async () => {
    const slpDir = path.join(__dirname, '../../test_data/legal/digital/techno_p1')
    const files: string[] = await fs.promises.readdir(slpDir);
    for(const filename of files ) {
        var data = fs.readFileSync(path.join(slpDir, filename), null);
        var game = new SlippiGame(toArrayBuffer(data))
        expect(isHandwarmer(game)).toBe(false)
    }

    const handwarmerDir = path.join(__dirname, '../../test_data/handwarmers/')
    const handwarmerFiles: string[] = await fs.promises.readdir(handwarmerDir);
    for(const filename of handwarmerFiles ) {
        var data = fs.readFileSync(path.join(handwarmerDir, filename), null);
        var game = new SlippiGame(toArrayBuffer(data))
        expect(isHandwarmer(game)).toBe(true)
    }
})