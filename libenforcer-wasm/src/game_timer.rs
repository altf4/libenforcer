use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerType {
    None = 0,
    Decreasing = 2,
    Increasing = 3,
}

/// Convert a frame number to a formatted game timer string (MM:SS.CC).
/// Mirrors the TypeScript frameToGameTimer() from slippi/utils/gameTimer.ts.
pub fn frame_to_game_timer(
    frame: i32,
    timer_type: TimerType,
    starting_timer_seconds: Option<u32>,
) -> String {
    match timer_type {
        TimerType::Decreasing => {
            let starting = match starting_timer_seconds {
                Some(s) => s,
                None => return "Unknown".to_string(),
            };
            let remainder = (60 - (frame % 60)) % 60;
            let centiseconds = ((remainder as f64 * 99.0) / 59.0).ceil() as u32;
            let total_seconds = (starting as f64 - frame as f64 / 60.0) as u32;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{:02}:{:02}.{:02}", minutes, seconds, centiseconds)
        }
        TimerType::Increasing => {
            let remainder = frame % 60;
            let centiseconds = ((remainder as f64 * 99.0) / 59.0).floor() as u32;
            let total_seconds = (frame as f64 / 60.0) as u32;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{:02}:{:02}.{:02}", minutes, seconds, centiseconds)
        }
        TimerType::None => "Infinite".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_if_no_starting_timer() {
        let result = frame_to_game_timer(1234, TimerType::Decreasing, None);
        assert_eq!(result, "Unknown");
    }

    #[test]
    fn test_increasing_timer() {
        let result = frame_to_game_timer(2014, TimerType::Increasing, Some(0));
        assert_eq!(result, "00:33.57");
    }

    #[test]
    fn test_decreasing_timer() {
        let result = frame_to_game_timer(4095, TimerType::Decreasing, Some(180));
        assert_eq!(result, "01:51.76");
    }

    #[test]
    fn test_exact_limit() {
        let result = frame_to_game_timer(10800, TimerType::Decreasing, Some(180));
        assert_eq!(result, "00:00.00");
    }
}
