import {SlippiGame} from './slippi'
import {Coord, FloatEquals, CheckResult, Violation} from './index';

export function hasDisallowedCStickCoords(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {   
    let violations: Violation[] = []
    for (const [index, coordinate] of coords.entries()) {
        if (FloatEquals(Math.abs(coordinate.x), 0.8)) {
            violations.push(new Violation(index, "Disallowed C-Stick Coordinate", [coordinate]))
        }
        if (FloatEquals(Math.abs(coordinate.x), 0.6625)) {
            violations.push(new Violation(index, "Disallowed C-Stick Coordinate", [coordinate]))
        }
    }

    return new CheckResult(violations.length !== 0, violations)
}