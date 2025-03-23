import { SlippiGame } from './slippi'
import { Coord, getUniqueCoords, isBoxController, CheckResult, Violation, FloatEquals } from './index';

export function hasIllegalUptiltRounding(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // If we're on ditigal, then it always passes
    if (isBoxController(coords)) {
        return new CheckResult(false)
    }

    return getUptiltCheck(game, playerIndex, coords)
}

export function getUptiltCheck(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // y in range 0.2 - 0.275
    for (let coord of coords) {
        // Only consider coords in the x deadzone
        if (Math.abs(coord.x) < 0.2876 && coord.y > 0.199 && coord.y < 0.2749) {
            return new CheckResult(false)
        }
    }

    // Now let's count up how many coords in the uptilt zone are on the exact boundary
    let boundaryCount: number = 0
    for (let coord of coords) {
        if (Math.abs(coord.x) < 0.2876 && FloatEquals(coord.y, 0.2875)) {
            boundaryCount++
        }
    }
    if (boundaryCount < 5) {
        return new CheckResult(false)
    }

    // Insert all coords here as evidence, for visualization
    return new CheckResult(true, [new Violation(0, "Uptilt rounding observed. No coordinates seen below uptilt area.", getUniqueCoords(coords))])
}
