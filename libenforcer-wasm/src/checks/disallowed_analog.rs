use crate::types::{CheckResult, Coord, Violation};
use crate::utils::float_equals;

/// Check for disallowed C-stick coordinate values
/// Certain exact values indicate controller hardware manipulation
pub fn check(c_coords: &[Coord]) -> CheckResult {
    get_cstick_violations(c_coords)
}

/// Get all C-stick violations
pub fn get_cstick_violations(c_coords: &[Coord]) -> CheckResult {
    const DISALLOWED_X_1: f64 = 0.8;
    const DISALLOWED_X_2: f64 = 0.6625;

    let mut violations = Vec::new();

    for (i, coord) in c_coords.iter().enumerate() {
        // Check if X coordinate matches disallowed values
        if float_equals(coord.x, DISALLOWED_X_1) || float_equals(coord.x, DISALLOWED_X_2) {
            violations.push(Violation::with_evidence(
                i as f64,
                format!("Disallowed C-Stick coordinate: x={}", coord.x),
                vec![*coord],
            ));
        }
    }

    if violations.is_empty() {
        CheckResult::pass()
    } else {
        CheckResult::fail(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disallowed_values() {
        let coords = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.8, y: 0.5 },      // Disallowed
            Coord { x: 0.6625, y: 0.3 },   // Disallowed
            Coord { x: 0.7, y: 0.4 },      // OK
        ];

        let result = get_cstick_violations(&coords);
        assert_eq!(result.result, true);
        assert_eq!(result.details.len(), 2);
    }

    #[test]
    fn test_no_violations() {
        let coords = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.5, y: 0.5 },
            Coord { x: 1.0, y: 1.0 },
        ];

        let result = get_cstick_violations(&coords);
        assert_eq!(result.result, false);
    }
}
