import { TimerType } from "../types";
import { frameToGameTimer } from "./gameTimer";

describe("when calculating the in-game timer", () => {
  it("should return unknown if no starting timer is provided", () => {
    const gameTimer = frameToGameTimer(1234, {
      timerType: TimerType.DECREASING,
      startingTimerSeconds: null,
    });
    expect(gameTimer).toEqual("Unknown");
  });

  it("should support increasing timers", () => {
    const gameTimer = frameToGameTimer(2014, {
      timerType: TimerType.INCREASING,
      startingTimerSeconds: 0,
    });
    expect(gameTimer).toEqual("00:33.57");
  });

  it("should support decreasing timers", () => {
    const gameTimer = frameToGameTimer(4095, {
      timerType: TimerType.DECREASING,
      startingTimerSeconds: 180,
    });

    expect(gameTimer).toEqual("01:51.76");
  });

  it("should support when the exact limit is hit", () => {
    const gameTimer = frameToGameTimer(10800, {
      timerType: TimerType.DECREASING,
      startingTimerSeconds: 180,
    });

    expect(gameTimer).toEqual("00:00.00");
  });
});
