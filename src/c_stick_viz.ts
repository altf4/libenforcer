import {SlippiGame} from './slippi'
import {Coord, CheckResult, Violation} from './index';

export function CStickViz(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    let coordViolation: Violation = new Violation(0, "C Stick Viz", coords)
    return new CheckResult(false, [coordViolation])
}
