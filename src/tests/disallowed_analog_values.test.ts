import {expect, test} from '@jest/globals';
import {Coord, hasDisallowedCStickCoords} from '../index';

test('Should pass check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0, 0) 
    coords.push(coord)
    coord = new Coord(1, 1) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(false);
});

test('Should trigger check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0.8, 0) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(true);
});

test('Should trigger check for disallowed C-Stick values', () => {
    var coords: Coord[] = []
    var coord = new Coord(0.6625, 0) 
    coords.push(coord)
    
    const result = hasDisallowedCStickCoords(coords);
    expect(result).toEqual(true);
});