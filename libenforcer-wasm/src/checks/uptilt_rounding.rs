use crate::types::{CheckResult, Coord};
use crate::utils::is_box_controller;

/// Check for illegal uptilt rounding on analog controllers
/// Detects when analog inputs are artificially rounded/quantized
/// Only applies to analog controllers
pub fn check(coords: &[Coord]) -> CheckResult {
    // Only applies to analog controllers (box controllers pass)
    if is_box_controller(coords) {
        return CheckResult::pass();
    }

    get_uptilt_check(coords)
}

/// Inner uptilt check without the box controller guard
/// Matches TypeScript's getUptiltCheck()
pub fn get_uptilt_check(coords: &[Coord]) -> CheckResult {
    // Uptilt zone boundaries (in normalized -1..1 range)
    const Y_MIN: f64 = 0.199;
    const Y_MAX: f64 = 0.2749;
    const X_MAX: f64 = 0.2876;
    const EXACT_BOUNDARY: f64 = 0.2875;

    let mut coords_at_exact_boundary = 0;

    for coord in coords {
        // Check if in uptilt zone (intermediate values) — natural variation
        if coord.x.abs() < X_MAX && coord.y > Y_MIN && coord.y < Y_MAX {
            return CheckResult::pass();
        }

        // Count coordinates exactly at the boundary
        if coord.x.abs() < X_MAX && (coord.y - EXACT_BOUNDARY).abs() < 0.0001 {
            coords_at_exact_boundary += 1;
        }
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
        // Has a coord in the intermediate uptilt zone - natural variation, should pass
        let result = get_uptilt_check(&[
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.1, y: 0.22 },   // In intermediate zone - natural
            Coord { x: 0.0, y: 0.2875 },
        ]);
        assert_eq!(result.result, false);
    }

    #[test]
    fn test_artificial_rounding() {
        // Many at exact boundary, none in intermediate zone
        let result = get_uptilt_check(&[
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
            Coord { x: 0.0, y: 0.2875 },
        ]);
        assert_eq!(result.result, true);
    }
}
