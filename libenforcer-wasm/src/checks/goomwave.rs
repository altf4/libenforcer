use crate::types::{CheckResult, Coord};

/// Check for GoomWave hardware modification
/// GoomWave clamps small stick movements to cardinal directions
/// Detection: absence of coordinates with small off-axis values
pub fn check(coords: &[Coord]) -> CheckResult {
    if has_goomwave_clamping(coords) {
        return CheckResult::fail_single(
            0.0,
            "No coordinates found with small off-axis values (characteristic of GoomWave clamping)".to_string(),
        );
    }

    CheckResult::pass()
}

/// Check for GoomWave cardinal clamping pattern
/// Returns true if coords show evidence of clamping (no small off-axis values)
pub fn has_goomwave_clamping(coords: &[Coord]) -> bool {
    const THRESHOLD: f64 = 0.08;

    for coord in coords {
        // Skip coordinates on cardinal axes (x=0 or y=0)
        if coord.x.abs() < 0.0001 || coord.y.abs() < 0.0001 {
            continue;
        }

        // If we find any coord with small X or Y value, it's natural
        if coord.x.abs() < THRESHOLD || coord.y.abs() < THRESHOLD {
            return false;
        }
    }

    true
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
        // Need enough unique rim coordinates to not be classified as a box controller
        // (is_box_controller requires >= 50% of 432 rim coords, with short-game boost)
        // All coordinates have both |x| and |y| > THRESHOLD (0.08), simulating goomwave clamping
        let mut coords: Vec<Coord> = Vec::new();

        // Generate 220 unique rim coordinates on the unit circle where both
        // |x| and |y| are well above THRESHOLD (0.08).
        // Safe angle range per quadrant: [0.1, 1.47] keeps sin & cos > 0.09
        // 55 points per quadrant * 4 quadrants = 220
        for q in 0..4 {
            let base = q as f64 * std::f64::consts::FRAC_PI_2;
            for i in 0..55 {
                let angle = base + 0.1 + (i as f64) * 0.025;
                let x = angle.cos();
                let y = angle.sin();
                coords.push(Coord { x, y });
            }
        }

        // Add some cardinal coordinates too (these get skipped by the clamping check)
        coords.push(Coord { x: 0.0, y: 1.0 });
        coords.push(Coord { x: 1.0, y: 0.0 });
        coords.push(Coord { x: 0.0, y: -1.0 });
        coords.push(Coord { x: -1.0, y: 0.0 });

        let result = check(&coords);
        assert_eq!(result.result, true);
    }
}
