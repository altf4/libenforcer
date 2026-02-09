use crate::types::{CheckResult, Coord};
use crate::utils::is_box_controller;

/// Check for GoomWave hardware modification
/// GoomWave clamps small stick movements to cardinal directions
/// Detection: absence of coordinates with small off-axis values
/// Only applies to analog controllers
pub fn check(coords: &[Coord]) -> CheckResult {
    // Only applies to analog controllers (box controllers pass)
    if is_box_controller(coords) {
        return CheckResult::pass();
    }

    const THRESHOLD: f64 = 0.08;

    // Look for any coordinate with small off-axis value
    // Natural analog sticks have plenty of these
    // GoomWave mod clamps them to perfect cardinals, so they're missing
    for coord in coords {
        // Skip coordinates on cardinal axes (x=0 or y=0)
        if coord.x.abs() < 0.0001 || coord.y.abs() < 0.0001 {
            continue;
        }

        // If we find any coord with small X or Y value, it's natural
        if coord.x.abs() < THRESHOLD || coord.y.abs() < THRESHOLD {
            return CheckResult::pass(); // Found natural variation
        }
    }

    // No small off-axis values found - suspicious
    CheckResult::fail_single(
        0.0,
        "No coordinates found with small off-axis values (characteristic of GoomWave clamping)".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_analog() {
        // Has small off-axis values - should pass
        let coords = vec![
            Coord { x: 0.05, y: 0.5 }, // Small X
            Coord { x: 0.5, y: 0.5 },
            Coord { x: 1.0, y: 0.03 }, // Small Y
        ];

        let result = check(&coords);
        assert_eq!(result.result, false);
    }

    #[test]
    fn test_goomwave_pattern() {
        // All coordinates are either on cardinal axes or have large values
        let coords = vec![
            Coord { x: 0.0, y: 1.0 },   // Cardinal
            Coord { x: 1.0, y: 0.0 },   // Cardinal
            Coord { x: 0.9, y: 0.9 },   // Both values large
        ];

        let result = check(&coords);
        assert_eq!(result.result, true);
    }
}
