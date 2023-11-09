import {Coord, isEqual} from './index';

export function averageTravelCoordHitRate(coordinates: Coord[]): number {
    let travelCoordCount: number = 0
    let targetCount: number = 0
    let lastCoord: Coord = {x: 800, y: 800}
    let isTargetAlready: boolean = true
    let isTravelAlready: boolean = false

    for (let coord of coordinates) {
        if (isEqual(coord, lastCoord)) {
            if (!isTargetAlready) {
                targetCount++
            }
            isTargetAlready = true
            isTravelAlready = false

        } else {
            // New coordinate, and we're not in the middle of a new target
            // Means that this is an intermediate value
            if (!isTargetAlready && !isTravelAlready) {
                travelCoordCount++
                isTravelAlready = true
            }
            isTargetAlready = false
        }
        lastCoord = coord
    }

    if (targetCount <= 1) {
        return 0
    }

    return travelCoordCount / (targetCount-1)
}