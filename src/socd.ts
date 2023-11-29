import {SlippiGame} from './slippi'
import {Coord, isBoxController, isEqual} from './index';

// Returns true if the given coord is the center coord or one of the surrounding 8 coords
export function isCenterIsh(coord: Coord): boolean {
    if (Math.abs(coord.x) < 0.013 && Math.abs(coord.y) < 0.013) {
        return true
    }
    return false
}

export function isLeftIsh(coord: Coord): boolean { 
    if (coord.x < -0.98) {
        return true
    }
    return false
}

export function isRightIsh(coord: Coord): boolean { 
    if (coord.x > 0.98) {
        return true
    }
    return false
}

export function hasIllegalSOCD(game: SlippiGame, playerIndex: number, coords: Coord[]) {
    // If we're on analog, then it always passes
    if (!isBoxController(coords)) {
        return false
    }
    
    // This doesn't literally need to be a dash. Just any input that goes from R <--> L
    let dashCount: number = 0 
    let passedCenterCount: number = 0
    let lastEdge = null
    let inCenterThisCycle: boolean = false
    let wasJustInCenter: boolean = false
    let travelFrameCount: number = 0

    for (let coord of coords) {      
        // If we took too long to travel, then give up and start over 
        if (travelFrameCount > 3) {
            lastEdge = null
            inCenterThisCycle = false
            travelFrameCount = 0
        }
        travelFrameCount++
        // If we go beyond the vertical deadzone, we're no longer dashing
        if (Math.abs(coord.y) > 0.2875) {
            lastEdge = null
            inCenterThisCycle = false
            travelFrameCount = 0
            continue
        }

        if (isLeftIsh(coord)) {
            if (lastEdge === "R") {
                dashCount += 1
                travelFrameCount = 0
                if (inCenterThisCycle) {
                    passedCenterCount += 1
                }
            }
            inCenterThisCycle = false
            lastEdge = "L"
        }
        if (isRightIsh(coord)) {
            if (lastEdge === "L") {
                dashCount += 1
                travelFrameCount = 0
                if (inCenterThisCycle) {
                    passedCenterCount += 1
                }
            }       
            inCenterThisCycle = false     
            lastEdge = "R"
        }
        if (isCenterIsh(coord)) {
            // If we get consecutive center coords, then abort
            if (wasJustInCenter) {
                lastEdge = null
                inCenterThisCycle = false
                travelFrameCount = 0
                continue                
            }
            inCenterThisCycle = true
            wasJustInCenter = true
        } else {
            wasJustInCenter = false
        }
    }

    // console.log("passedCenterCount", passedCenterCount)
    // console.log("dashCount", dashCount)
    console.log("percent: ",  (passedCenterCount / dashCount) * 100, dashCount)

    return false
}
