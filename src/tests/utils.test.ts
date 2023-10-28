import {expect, test} from '@jest/globals';
import {FloatEquals} from '../index';

test('Float equals allow a tiny bit of wiggle room', () => {
    expect(FloatEquals(0.8, 0.8)).toEqual(true);
    expect(FloatEquals(0.8, 0.7999)).toEqual(true);
    expect(FloatEquals(-0.7, -0.7000000000001)).toEqual(true);
    expect(FloatEquals(0.8, -0.8)).toEqual(false);
});
