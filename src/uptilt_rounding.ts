import {SlippiGame} from './slippi'
import {Coord, isBoxController, CheckResult} from './index';

export function hasIllegalUptiltRounding(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // If we're on ditigal, then it always passes
    if (isBoxController(coords)) {
        return new CheckResult(false)
    }

    // y in range 0.2 - 0.275
    for (let coord of coords) {
        // Only consider coords in the x deadzone
        if (Math.abs(coord.x) < 0.2876 && coord.y > 0.199 && coord.y < 0.2749) {
            return new CheckResult(false)
        }
    }
    return new CheckResult(true, 0, "Uptilt rounding observed. No coordinates seen below uptilt area.")
}