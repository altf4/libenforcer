import {SlippiGame} from './slippi'
import {Coord, FloatEquals, CheckResult} from './index';

export function hasDisallowedCStickCoords(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {   
    for (var coordinate of coords) {   
        if (FloatEquals(Math.abs(coordinate.x), 0.8)) {
            return new CheckResult(true)
        }
        if (FloatEquals(Math.abs(coordinate.x), 0.6625)) {
            return new CheckResult(true)
        }
    }

    return new CheckResult(false)
}