import {SlippiGame} from './slippi'
import {Coord, CheckResult, Violation, isBoxController} from './index';

export enum SDIRegion {
    DZ = 0,
    NE = 1,
    SE = 2,
    SW = 3,
    NW = 4,
    N = 5,
    E = 6,
    S = 7,
    W = 8,
}

const DIAGONALS = [SDIRegion.NE, SDIRegion.SE, SDIRegion.NW, SDIRegion.SW]
const CARDINALS = [SDIRegion.N, SDIRegion.E, SDIRegion.S, SDIRegion.W]

export function getSDIRegion(x: number, y: number): SDIRegion {
    let region = SDIRegion.DZ;

    if (x >= 0.2875 && y >= 0.2875) {
        region = SDIRegion.NE;
    } else if (x >= 0.2875 && y <= -0.2875) {
        region = SDIRegion.SE;
    } else if (x <= -0.2875 && y <= -0.2875) {
        region = SDIRegion.SW;
    } else if (x <= -0.2875 && y >= 0.2875) {
        region = SDIRegion.NW;
    } else if (y >= 0.2875) {
        region = SDIRegion.N;
    } else if (x >= 0.7) {
        region = SDIRegion.E;
    } else if (y <= -0.7) {
        region = SDIRegion.S;
    } else if (x <= -0.7) {
        region = SDIRegion.W;
    }

    return region;
}

// Directly adjacent
export function isRegionAdjacent(regionA: SDIRegion, regionB: SDIRegion): boolean {
    if (regionA == SDIRegion.N) {
        if (regionB == SDIRegion.NW || regionB == SDIRegion.NE) {
            return true
        }                 
    }
    if (regionA == SDIRegion.NE) {
        if (regionB == SDIRegion.N || regionB == SDIRegion.E) {
            return true
        }                 
    }
    if (regionA == SDIRegion.E) {
        if (regionB == SDIRegion.NE || regionB == SDIRegion.SE) {
            return true
        }                 
    }    
    if (regionA == SDIRegion.SE) {
        if (regionB == SDIRegion.E || regionB == SDIRegion.S) {
            return true
        }                 
    }
    if (regionA == SDIRegion.S) {
        if (regionB == SDIRegion.SE || regionB == SDIRegion.SW) {
            return true
        }                 
    }
    if (regionA == SDIRegion.SW) {
        if (regionB == SDIRegion.S || regionB == SDIRegion.W) {
            return true
        }                 
    }
    if (regionA == SDIRegion.W) {
        if (regionB == SDIRegion.SW || regionB == SDIRegion.NW) {
            return true
        }                 
    }
    if (regionA == SDIRegion.NW) {
        if (regionB == SDIRegion.W || regionB == SDIRegion.N) {
            return true
        }                 
    }
    return false
}


// Just for diagonals, ie: skipping the cardinals
export function isDiagonalAdjacent(regionA: SDIRegion, regionB: SDIRegion): boolean {
    const DIAGONALS = [SDIRegion.NE, SDIRegion.SE, SDIRegion.NW, SDIRegion.SW]
    if (!DIAGONALS.includes(regionA)) {
        return false
    }
    if (!DIAGONALS.includes(regionB)) {
        return false
    }

    if (regionA == SDIRegion.NE) {
        if (regionB == SDIRegion.NW || regionB == SDIRegion.SE) {
            return true
        }                 
    }
    if (regionA == SDIRegion.NW) {
        if (regionB == SDIRegion.NE || regionB == SDIRegion.SW) {
            return true
        }                 
    }
    if (regionA == SDIRegion.SW) {
        if (regionB == SDIRegion.SE || regionB == SDIRegion.NW) {
            return true
        }                 
    }
    if (regionA == SDIRegion.SE) {
        if (regionB == SDIRegion.NE || regionB == SDIRegion.SW) {
            return true
        }                 
    }

    return false
}
  
// Rapidly tapping the same direction and returning to neutral faster than once every 5.5 frames triggers 1 SDI and ignores subsequent attempts.
// Returns: array of violations
export function failsSDIRuleOne(coords: Coord[]): Violation[] {
    // Pull out the region of every input
    let violations: Violation[] = []
    let regions: SDIRegion[] = []
    for (let coord of coords) {
        regions.push(getSDIRegion(coord.x, coord.y))
    }

    for (let [i, region] of regions.entries()) {
        // Look ahead 5 frames to see if we hit two SDIs from neutral
        if (region === SDIRegion.DZ) {
            let lastRegion: SDIRegion = SDIRegion.DZ
            let sdi_count: number = 0
            let firstSDIRegion: SDIRegion = null
            for (let j = 1; j <= 5 && (i+j) < regions.length; j++) {
                // Get the first SDI region
                if (regions[i+j] !== SDIRegion.DZ && firstSDIRegion === null) {
                    firstSDIRegion = regions[i+j]
                }
                // If we went from DZ to the first SDI region. 
                // ie: Last region was the deadzone, current region is the starting point
                if (lastRegion === SDIRegion.DZ && regions[i+j] === firstSDIRegion) {
                    sdi_count++
                }
                lastRegion = regions[i+j]
            }
            if (sdi_count >= 2) {
                violations.push(new Violation(i, "Failed SDI rule #1", coords.slice(i, i+6)))
            }
        }
    }

    return violations
}

// Rapidly tapping the same diagonal and returning to an adjacent cardinal faster than once every 5.5 frames triggers 1 SDI and ignores subsequent attempts.
// Returns: array of violations
export function failsSDIRuleTwo(coords: Coord[]): Violation[] {
    // Pull out the region of every input
    let violations: Violation[] = []
    let regions: SDIRegion[] = []
    for (let coord of coords) {
        regions.push(getSDIRegion(coord.x, coord.y))
    }
    
    for (let i = 0; i < regions.length; i++) {
        const startingRegion = regions[i];

        // Start from a cardinal
        if (!CARDINALS.includes(startingRegion)) {
            continue
        }

        // Now look 5 frames ahead. 
        // Do we alternate between here and an adjacent diagonal twice?
        let sdiCount = 0
        let adjacentCardinalRegion = -1
        for (let j = 1; j <= 5 && (i+j) < regions.length; j++) {
            // Ignore if we haven't moved regions
            if (regions[i+j] === regions[i+j-1]) {
                continue
            }

            // Have we hit the diagonal SDI? (an adjacent diagonal)
            if (isRegionAdjacent(startingRegion, regions[i+j]) && DIAGONALS.includes(regions[i+j])) {
                if (adjacentCardinalRegion === -1 || adjacentCardinalRegion === regions[i+j]) {
                    adjacentCardinalRegion = regions[i+j]
                    sdiCount++
                }
            }
        }
        if (sdiCount >= 2) {
            violations.push(new Violation(i, "Failed SDI rule #2", coords.slice(i, i+6)))
        }
    }
    return violations
}

// Alternating between adjacent cardinals
// Returns: array of violations
export function failsSDIRuleThree(coords: Coord[]): Violation[] {
    // Pull out the region of every input
    let violations: Violation[] = []
    let regions: SDIRegion[] = []
    for (let coord of coords) {
        regions.push(getSDIRegion(coord.x, coord.y))
    }
    
     for (let i = 0; i < regions.length; i++) {
        const currentRegion = regions[i];

        if (DIAGONALS.includes(currentRegion)) {
            // Look forward 5 frames to see if it goes to an adjacent diagonal and back 
            var hitAdjacent = false
            for (let j = i + 1; j <= i + 5 && j < regions.length; j++) {
                // Hit the adjacent
                if (isDiagonalAdjacent(regions[j], currentRegion)) {
                    hitAdjacent = true
                }
                // Then returned back
                if (hitAdjacent && (regions[j] === currentRegion)) {
                    violations.push(new Violation(i, "Failed SDI rule #3", coords.slice(i, i+6)))
                }
            }
        }
    }
  
    return violations;
  }

export function hasIllegalSDI(game: SlippiGame, playerIndex: number, coords: Coord[]): CheckResult {
    // If we're on analog, then it always passes
    if (!isBoxController(coords)) {
        return new CheckResult(false)
    }

    let infractionFrames = failsSDIRuleOne(coords)
    if (infractionFrames.length > 0) {
        return new CheckResult(true, infractionFrames)
    } 
     
    infractionFrames = failsSDIRuleTwo(coords)
    if (infractionFrames.length > 0) {
        return new CheckResult(true, infractionFrames)
    } 

    infractionFrames = failsSDIRuleThree(coords)
    if (infractionFrames.length > 0) {
        return new CheckResult(true, infractionFrames)
    } 

    return new CheckResult(false)
}
