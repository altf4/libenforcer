import {SlippiGame} from './slippi'
import {Coord, FloatEquals, isBoxController} from './index';

export function hasDisallowedCStickCoords(game: SlippiGame, playerIndex: number, coords: Coord[]): boolean {   
    for (var coordinate of coords) {   
        if (FloatEquals(Math.abs(coordinate.x), 0.8)) {
            return true
        }
        if (FloatEquals(Math.abs(coordinate.x), 0.6625)) {
            return true
        }
    }

    return false
}