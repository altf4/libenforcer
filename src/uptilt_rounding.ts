import {SlippiGame} from './slippi'
import {Coord, isBoxController, isEqual} from './index';

export function hasIllegaUptiltRounding(game: SlippiGame, playerIndex: number, coords: Coord[]) {
    // If we're on ditigal, then it always passes
    if (isBoxController(coords)) {
        return false
    }

    // y in range 0.2 - 0.275
    for (let coord of coords) {
        // Only consider coords in the x deadzone
        if (Math.abs(coord.x) < 0.2876 && coord.y > 0.199 && coord.y < 0.2749) {
            return false
        }
    }
    return true
}