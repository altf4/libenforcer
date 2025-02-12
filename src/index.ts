import {SlippiGame, FramesType} from './slippi'
import * as semver from 'semver'
import {hasDisallowedCStickCoords} from './disallowed_analog_values'
import {hasIllegalTravelTime} from './travel_time'
import {hasIllegalUptiltRounding} from './uptilt_rounding'
import {hasIllegalCrouchUptilt} from './crouch_uptilt'
import {hasIllegalSDI} from './sdi'
import {isGoomwave} from './goomwave'
import {controlStickViz} from './control_stick_viz'

export {hasDisallowedCStickCoords, getCStickViolations} from './disallowed_analog_values'
export {averageTravelCoordHitRate, hasIllegalTravelTime} from './travel_time'
export {hasIllegalUptiltRounding} from './uptilt_rounding'
export {hasIllegalSDI} from './sdi'
export {isGoomwave} from './goomwave'
export {hasIllegalCrouchUptilt} from './crouch_uptilt'
export {controlStickViz} from './control_stick_viz'

export * from './slippi'

// Holds the overall results of a check against a single player on a single game
export class CheckResult {
  result: boolean
  violations: Violation[]

  constructor(result: boolean, violations: Violation[] = []) {
    this.result = result
    this.violations = violations
  }
}

// Represents a single violation of a rule
export class Violation {
  metric: number // Usually a frame number, but can represent something else
  reason: string
  evidence: any[] // optional

  constructor(metric: number, reason: string, evidence: any[] = []) {
    this.metric = metric
    this.reason = reason
    this.evidence = evidence
  }
}

export type Check = {
  name: string
  checkFunction: (game: SlippiGame, playerIndex: number, coords: Coord[]) => CheckResult
}

// Provide an array of strings that describe the available Checks
export function ListChecks(): Check[] {
  var checks: Check[] = []

  checks.push({name: "Box Travel Time", 
              checkFunction: hasIllegalTravelTime
              })
  checks.push({name: "Disallowed Analog C-Stick Values", 
              checkFunction: hasDisallowedCStickCoords
              })
  checks.push({name: "Uptilt Rounding", 
              checkFunction: hasIllegalUptiltRounding
              })
  checks.push({name: "Fast Crouch Uptilt", 
              checkFunction: hasIllegalCrouchUptilt
              })
  checks.push({name: "Illegal SDI", 
              checkFunction: hasIllegalSDI
              })
  checks.push({name: "GoomWave Clamping", 
              checkFunction: isGoomwave
              })              
  checks.push({name: "Control Stick Visualization", 
              checkFunction: controlStickViz
              })


              
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

export function processAnalogStick(coord: Coord, deadzone: boolean): Coord {
  let magnitudeSquared = (coord.x*coord.x) + (coord.y*coord.y)
  if (magnitudeSquared < 1e-3) {
    return {x: 0, y: 0}
  }

  let magnitude = Math.sqrt(magnitudeSquared)
  const threshold = 80

  let fX: number = coord.x
  let fY: number = coord.y
  if (magnitude > threshold) {
    let shrinkFactor = threshold / magnitude
    if (fX > 0) {
      fX = Math.floor(fX * shrinkFactor)
      fY = Math.floor(fY * shrinkFactor)  
    } else {
      fX = Math.ceil(fX * shrinkFactor)
      fY = Math.ceil(fY * shrinkFactor)  
    }
  }

  // Apply deadzone if applicable
  if (deadzone) {
    if (Math.abs(fX) < 23) {
      fX = 0
    }
    if (Math.abs(fY) < 23) {
      fY = 0
    }
  }

  // Round to the nearest integer (pixel)
  fX = Math.round(fX)
  fY = Math.round(fY)

  return {x: Math.floor(fX) / 80, y: Math.floor(fY) / 80}
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

      if(isMainStick) {
        coord = processAnalogStick(coord, false)
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
  // If we get two non-zero target coords in the deadzone, then it's def analog
  //  NOTE: The opposite is not true. It's normal to have an analog controller 
  //    sometimes only register targets at 0,0
  let dzTargetCount: number = 0
  for (let target of targets) {
    if (!isEqual(target, deadCenter) && (getJoystickRegion(target.x, target.y) === JoystickRegion.DZ)) {
      dzTargetCount++
    }
  }

  if (dzTargetCount >= 2) {
    return false
  }

  // Is the overall gamewide unique coords/sec rate > 5?
  let uniqueCoords = getUniqueCoords(coordinates)
  let coordsPerSecond: number = ((uniqueCoords.length*60)/coordinates.length)
  if (coordsPerSecond > 5) {
    return false
  }

  // TODO Other checks
  return true
}

// Is this a supported SLP replay version?
export function isSlpMinVersion(game: SlippiGame): boolean {
  return semver.lt(game.getSettings().slpVersion, '3.15.0')
}

export function isHandwarmer(game: SlippiGame): boolean {
  for (let playerIndex in game.getFrames()[0].players) {
    let coords = getCoordListFromGame(game, parseInt(playerIndex), true)
    // If the game is less than a minute long, then it's a handwarmer
    if (coords.length < 3600) {
      return true
    }

    // If a player went 10 straight seconds in the deadzone, then it's a handwarmer
    //  *while alive
    let frameIndex = -123
    let deadzoneFrames: number = 0
    for (let coord of coords) {
      if (getJoystickRegion(coord.x, coord.y) === JoystickRegion.DZ) {
        deadzoneFrames++
      } else {
        deadzoneFrames = 0
      }
      if (!(playerIndex in game.getFrames()[frameIndex].players)) {
        deadzoneFrames = 0
      }
      if (deadzoneFrames > 600) {
        return true
      }
      frameIndex++
    }
  }
  return false
}