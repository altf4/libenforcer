use crate::types::{CheckResult, Coord};
use crate::utils::is_box_controller;

/// Check for illegal uptilt rounding on analog controllers
/// Detects when analog inputs are artificially rounded/quantized
/// Only applies to analog controllers
pub fn check(main_coords: &[Coord], raw_coords: &[Coord]) -> CheckResult {
    // Only applies to analog controllers (box controllers pass)
    if is_box_controller(main_coords) {
        return CheckResult::pass();
    }

    // Uptilt zone boundaries
    const Y_MIN: f64 = 0.199;
    const Y_MAX: f64 = 0.2749;
    const X_MIN: f64 = -0.2876;
    const X_MAX: f64 = 0.2876;
    const EXACT_BOUNDARY: f64 = 0.2875;

    let mut coords_in_zone = 0;
    let mut coords_at_exact_boundary = 0;

    for coord in raw_coords {
        // Check if in uptilt zone (intermediate values)
        if coord.x >= X_MIN
            && coord.x <= X_MAX
            && coord.y >= Y_MIN
            && coord.y <= Y_MAX
        {
            coords_in_zone += 1;
        }

        // Count coordinates exactly at the boundary
        if (coord.y - EXACT_BOUNDARY).abs() < 0.0001 {
            coords_at_exact_boundary += 1;
        }
    }

    // If ANY coordinate exists in the intermediate zone, pass (natural variation)
    if coords_in_zone > 0 {
        return CheckResult::pass();
    }

    // If fewer than 5 coordinates at exact boundary, pass (not enough data)
    if coords_at_exact_boundary < 5 {
        return CheckResult::pass();
    }

    // Suspicious: many coordinates at exact boundary, none in intermediate zone
    CheckResult::fail_single(
        coords_at_exact_boundary as f64,
        format!(
            "Uptilt rounding detected: {} coordinates at exact boundary, 0 in intermediate zone",
            coords_at_exact_boundary
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_analog() {
        let main_coords = vec![Coord { x: 0.0, y: 0.0 }; 100]; // Analog-like
        let raw_coords = vec![
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.1, y: 0.22 },   // In intermediate zone - natural
            Coord { x: 0.0, y: 0.2875 },
        ];

        let result = check(&main_coords, &raw_coords);
        assert_eq!(result.result, false);
    }

    #[test]
    fn test_artificial_rounding() {
        let main_coords = vec![Coord { x: 0.0, y: 0.0 }; 100]; // Analog-like
        let raw_coords = vec![
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 }, // Many at exact boundary, none intermediate
        ];

        let result = check(&main_coords, &raw_coords);
        assert_eq!(result.result, true);
    }
}
