import {SlippiGame} from './slippi'
import {Coord, FloatEquals, CheckResult, Violation, isBoxController, getCoordListFromGame} from './index';

export function hasDisallowedCStickCoords(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // If we're on analog, then it always passes
    // NOTE: We inspect C-Stick coords for this check, so we have to get the main stick coords for this conditional
        if (!isBoxController(getCoordListFromGame(game, playerIndex, true))) {
        return new CheckResult(false)
    }

    let violations = getCStickViolations(coords)
    return new CheckResult(violations.length !== 0, violations)
}

export function getCStickViolations(coords: Coord[]): Violation[] {
    let violations: Violation[] = []
    for (const [index, coordinate] of coords.entries()) {
        if (FloatEquals(Math.abs(coordinate.x), 0.8)) {
            violations.push(new Violation(index, "Disallowed C-Stick Coordinate", [coordinate]))
        }
        if (FloatEquals(Math.abs(coordinate.x), 0.6625)) {
            violations.push(new Violation(index, "Disallowed C-Stick Coordinate", [coordinate]))
        }
    }
    return violations
}