import {expect, test} from '@jest/globals';
import {getUniqueCoords, isBoxController, Coord, SlippiGame, getCoordListFromGame, hasDisallowedCStickCoords, toArrayBuffer} from '../index';
import * as fs from 'fs';
import * as path from 'path';

test('Should pass check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0, 0) 
    coords.push(coord)
    coord = new Coord(1, 1) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(false);
})

test('Should trigger check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0.8, 0) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(true);
})

test('Should trigger check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0.6625, 0) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(true);
})

test('Test full game with disallowed C-stick value', () => {
    var data: Buffer = fs.readFileSync(path.join(__dirname, '../../test_data/banned_c_stick_analog_player_1.slp'), null);
    var game = new SlippiGame(toArrayBuffer(data))
    expect(game).toBeDefined()

    var gameCoords: Coord[] = getCoordListFromGame(game, 0, false)
    expect(hasDisallowedCStickCoords(gameCoords)).toBe(true)
})