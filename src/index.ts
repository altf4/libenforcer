export {hasDisallowedCStickCoords} from './disallowed_analog_values'
export * from './slippi'


export class Check {
  name: string
  isProbabilistic: boolean

  constructor() {
    this.name = "unknown"
    this.isProbabilistic = false
  }
}

// Provide an array of strings that describe the available Checks
export function ListChecks(): Check[] {
  var checks: Check[]

  var disallowedAnalogValues: Check
  disallowedAnalogValues.name = "Disallowed Analog C-Stick Values"
  disallowedAnalogValues.isProbabilistic = false
  checks.push(disallowedAnalogValues)

  return checks
}

export class Coord {
    x: number
    y: number

    constructor(x: number, y: number) {
      this.x = x
      this.y = y
    }

    isEqual(other: Coord): boolean {
      return (FloatEquals(this.x, other.x) && FloatEquals(this.y, other.y))
    }
}

export function FloatEquals(a: number, b: number): boolean {
  if (Math.abs(a-b) < 0.0001) {
    return true
  }
  return false
}

export enum JoystickRegion {
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

export function getJoystickRegion(x: number, y: number): JoystickRegion {
    let region = JoystickRegion.DZ;
  
    if (x >= 0.2875 && y >= 0.2875) {
      region = JoystickRegion.NE;
    } else if (x >= 0.2875 && y <= -0.2875) {
      region = JoystickRegion.SE;
    } else if (x <= -0.2875 && y <= -0.2875) {
      region = JoystickRegion.SW;
    } else if (x <= -0.2875 && y >= 0.2875) {
      region = JoystickRegion.NW;
    } else if (y >= 0.2875) {
      region = JoystickRegion.N;
    } else if (x >= 0.2875) {
      region = JoystickRegion.E;
    } else if (y <= -0.2875) {
      region = JoystickRegion.S;
    } else if (x <= -0.2875) {
      region = JoystickRegion.W;
    }
  
    return region;
  }

  export function getTargetCoords(coordinates: Coord[]): Coord[] {
    var targets: Coord[] = []
    var lastCoord: Coord = null
    for (let coord of coordinates) {
      if (lastCoord != null) {
        if (lastCoord.isEqual(coord)) {
            targets.push(coord)
        }
      }
      lastCoord = coord
    }
    return targets
  }

  export function isBoxController(coordinates: Coord[]): boolean {
    var targets = getTargetCoords(coordinates)
    const deadCenter: Coord = new Coord(0, 0)
    // If we get a non-zero target coord in the deadzone, then it's def a GCN controller
    for (let target of targets) {
      if (!target.isEqual(deadCenter) && getJoystickRegion(target.x, target.y) === JoystickRegion.DZ) {
        return false
      }
    }
    // TODO Other checks
    return true
  }