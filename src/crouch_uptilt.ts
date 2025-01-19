import {SlippiGame} from './slippi'
import {Coord, isBoxController, CheckResult, Violation} from './index';

export function hasIllegalCrouchUptilt(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    let violations: Violation[] = []
    // If we're on analog, then it always passes
    if (!isBoxController(coords)) {
        return new CheckResult(false)
    }

    let actions: number[] = []

    // For this one, we use a different strategy,
    //  we just look at the game states, rather than inputs
    let frames = game.getFrames()
    let lastCrouch = -124
    for (let i = -123; i < game.getStats().lastFrame; i++) {
        if (!(playerIndex in frames[i].players)) {
            continue
        }
        let actionState = frames[i].players[playerIndex].post.actionStateId
        // Crouching
        if (actionState == 0x28) {
            lastCrouch = i
        }
        // Uptilt
        if (actionState == 0x38) {
            if (i - lastCrouch <= 3) {
                violations.push(new Violation(lastCrouch, "Crouch-uptilt occurred within three frames", coords.slice(lastCrouch+123, lastCrouch+123+4)))
            }
        }
    }

    return new CheckResult(violations.length !== 0, violations)
}
