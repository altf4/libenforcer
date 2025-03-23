import { SlippiGame } from './slippi'
import { Coord, isBoxController, CheckResult, Violation, FloatEquals } from './index';

export function isGoomwave(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // Goomwaves seem to clamp anything under 0.0875 to the cardinal
    let CLAMP_MAXIMUM: number = 0.08

    // If we're on box, then it always passes
    if (isBoxController(coords)) {
        return new CheckResult(false)
    }

    for (let coord of coords) {
        // Ignore coords on the cardinals. They don't count
        if (FloatEquals(coord.x, 0) || FloatEquals(coord.y, 0)) {
            continue
        }

        // If there's a coord inside CLAMP_MAXIMUM, then it's not doing goomwave clamping
        if (Math.abs(coord.x) < CLAMP_MAXIMUM || Math.abs(coord.y) < CLAMP_MAXIMUM) {
            return new CheckResult(false, [])
        }
    }

    return new CheckResult(true, [new Violation(0, "Evidence of cardinal clamping", coords)])
}
