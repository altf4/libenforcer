import {SlippiGame} from './slippi'
import {Coord, isBoxController} from './index';

export function hasIllegalCrouchUptilt(game: SlippiGame, playerIndex: number, coords: Coord[]) {
    // If we're on analog, then it always passes
    if (!isBoxController(coords)) {
        return false
    }

    // For this one, we use a different strategy,
    //  we just look at the game states, rather than inputs
    var frames = game.getFrames()
    let lastCrouch = -124
    for (let i = -123; i < game.getStats().lastFrame; i++) {
        let actionState = frames[i].players[playerIndex].post.actionStateId
        // Crouching
        if (actionState == 0x28) {
            lastCrouch = i
        }
        // Uptilt
        if (actionState == 0x38) {
            if (i - lastCrouch <= 3) {
                return true
            }
        }
    }

    return false
}

