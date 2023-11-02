import {expect, test} from '@jest/globals';
import {FloatEquals, getTargetCoords, isBoxController, Coord} from '../index';

test('Float equals allow a tiny bit of wiggle room', () => {
    expect(FloatEquals(0.8, 0.8)).toEqual(true);
    expect(FloatEquals(0.8, 0.7999)).toEqual(true);
    expect(FloatEquals(-0.7, -0.7000000000001)).toEqual(true);
    expect(FloatEquals(0.8, -0.8)).toEqual(false);
});

test('Get target coords', () => {
    var coords: Coord[] = []
    expect(getTargetCoords(coords).length).toEqual(0)

    coords.push(new Coord(0, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(-1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(1, 0))
    coords.push(new Coord(.5, .5))
    coords.push(new Coord(1, 0))

    expect(getTargetCoords(coords).length).toEqual(1)

});

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
});