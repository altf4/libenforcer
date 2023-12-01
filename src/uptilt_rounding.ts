import {SlippiGame} from './slippi'
import {Coord, isBoxController, isEqual} from './index';

export function hasIllegaUptiltRounding(game: SlippiGame, playerIndex: number, coords: Coord[]) {
    // If we're on ditigal, then it always passes
    if (isBoxController(coords)) {
        return false
    }

    // 0.2 - 0.275
    for (let coord of coords) {
        if (coord.y > 0.199 && coord.y < 0.2749) {
            return false
        }
    }
    
    return true
}