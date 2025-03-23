import { SlippiGame } from './slippi'
import { Coord, CheckResult, Violation } from './index';

export function controlStickViz(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    let coordViolation: Violation = new Violation(0, "Control Stick Viz", coords)
    return new CheckResult(false, [coordViolation])
}
