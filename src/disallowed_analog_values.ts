import {Coord, FloatEquals} from './index';

export function hasDisallowedCStickCoords(coordinates: Coord[]): boolean {
    for (var coordinate of coordinates) {   
        if (FloatEquals(Math.abs(coordinate.x), 0.8)) {
            return true
        }
        if (FloatEquals(Math.abs(coordinate.x), 0.6625)) {
            return true
        }
    }

    return false
}