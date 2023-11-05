import {SlippiGame, FramesType} from './slippi'
import * as semver from 'semver'

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

// Most of entire controller state in one type
export type GamePad = {
  mainStick: Coord
  cStick: Coord
  trigger: number
  a: boolean
  b: boolean
  x: boolean
  y: boolean
  z: boolean
  l: boolean
  r: boolean
}

export type Coord = {
  x: number
  y: number
}

export function jsonToCoord(json: string): Coord {
  var parsed = JSON.parse(json)
  return {x: parsed.x, y: parsed.y}
}

export function isEqual(one: Coord, other: Coord): boolean {
  return (FloatEquals(one.x, other.x) && FloatEquals(one.y, other.y))
}

export function FloatEquals(a: number, b: number): boolean {
  if (Math.abs(a-b) < 0.0001) {
    return true
  }
  return false
}

export function toArrayBuffer(buffer: Buffer): ArrayBuffer {
  const arrayBuffer = new ArrayBuffer(buffer.length);
  const view = new Uint8Array(arrayBuffer);
  for (let i = 0; i < buffer.length; ++i) {
    view[i] = buffer[i];
  }
  return arrayBuffer;
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

// Uniques a set of coords
export function getUniqueCoords(coordinates: Coord[]): Coord[] {
  const targetsSet = new Set<string>()
  
  for (let coord of coordinates) {
      targetsSet.add(JSON.stringify(coord))
  }

  // All of this just so we can have a set of an object by value
  //  Get your shit together, JavaScript
  var targets: Coord[] = []
  for (let target of targetsSet) {
    targets.push(jsonToCoord(target))
  }

  return targets  
}

// Gets a list of "target" coordinates, defined as having dwelled there
//  for at least two consecutive frames. 
//  IE: Removing travel time coords
export function getTargetCoords(coordinates: Coord[]): Coord[] {
  const targetsSet = new Set<string>()
  
  var lastCoord: Coord = null
  for (let coord of coordinates) {
    if (lastCoord != null) {
      if (isEqual(lastCoord, coord)) {
        targetsSet.add(JSON.stringify(coord))
      }
    }
    lastCoord = coord
  }

  // All of this just so we can have a set of an object by value
  //  Get your shit together, JavaScript
  var targets: Coord[] = []
  for (let target of targetsSet) {   
    targets.push(jsonToCoord(target))
  }

  return targets
}

export function getCoordListFromGame(game: SlippiGame, playerIndex: number, isMainStick: boolean): Coord[] {
  var frames: FramesType = game.getFrames()
  var coords: Coord[] = []
  var frame: number = -123
  while (true) {
    try {
      var coord: Coord = {x: 0, y: 0}
      var x: number = 0
      if (isMainStick) {
        x = frames[frame].players[playerIndex]?.pre.rawJoystickX
      } else {
        x = frames[frame].players[playerIndex]?.pre.cStickX
      }
      if (x !== undefined && x !== null) {
        coord.x = x
      }
      var y: number = 0
      if (isMainStick) {
        y = frames[frame].players[playerIndex]?.pre.rawJoystickY
      } else {
        y = frames[frame].players[playerIndex]?.pre.cStickY
      }
      if (y !== undefined && y !== null) {
        coord.y = y
      }
      coords.push(coord)
    }
    catch(err: any) {
      break
    } 
    frame += 1
  }
  return coords
}

export function isBoxController(coordinates: Coord[]): boolean {
  var targets = getTargetCoords(coordinates)
  var deadCenter: Coord = {x: 0, y: 0}
  // If we get a non-zero target coord in the deadzone, then it's def a GCN controller
  for (let target of targets) {
    if (!isEqual(target, deadCenter) && getJoystickRegion(target.x, target.y) === JoystickRegion.DZ) {
      return false
    }
  }

  // If we get more than 13 total C-stick coords, then it's analog
  // 13 is the maximum allowed number of digital coordinates
  if (getUniqueCoords(coordinates).length > 13) {
    return false
  }

  // TODO Other checks
  return true
}

// Is this a supported SLP replay version?
export function isSlpMinVersion(game: SlippiGame): boolean {
  return semver.lt(game.getSettings().slpVersion, '3.15.0')
}