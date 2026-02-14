use crate::types::{CheckResult, Coord};
use crate::utils::{is_box_controller, is_equal_coord};

/// Check for illegal travel time patterns on box controllers
/// Box controllers should have ~36% travel coordinates
/// Less than 25% indicates suspicious behavior
pub fn check(coords: &[Coord]) -> CheckResult {
    // Only applies to box controllers
    if !is_box_controller(coords) {
        return CheckResult::pass();
    }

    let travel_percent = average_travel_coord_hit_rate(coords);

    if travel_percent < 0.25 {
        return CheckResult::fail_single(
            travel_percent,
            format!("Fewer than 25% of coordinates had travel ({:.1}%)", travel_percent * 100.0),
        );
    }

    CheckResult::pass_single(
        travel_percent,
        format!("Travel coordinate hit rate: {:.1}%", travel_percent * 100.0),
    )
}

/// Calculate the average travel coordinate hit rate
/// Travel coordinates are intermediate values between target positions
/// Target positions are coords that stay the same for 2+ frames
pub fn average_travel_coord_hit_rate(coordinates: &[Coord]) -> f64 {
    let mut travel_coord_count = 0;
    let mut target_count = 0;
    let mut last_coord = Coord { x: 800.0, y: 800.0 }; // Impossible coord
    let mut is_target_already = true;
    let mut is_travel_already = false;

    for coord in coordinates {
        if is_equal_coord(coord, &last_coord) {
            // Same as last coord - this is a target position
            if !is_target_already {
                target_count += 1;
            }
            is_target_already = true;
            is_travel_already = false;
        } else {
            // Different from last coord
            // If we're not in a target and haven't counted this as travel yet
            if !is_target_already && !is_travel_already {
                travel_coord_count += 1;
                is_travel_already = true;
            }
            is_target_already = false;
        }
        last_coord = *coord;
    }

    // Need at least 2 targets to have travel between them
    if target_count <= 1 {
        return 0.0;
    }

    travel_coord_count as f64 / (target_count - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel_time_calculation() {
        // Pattern: target, target, travel, target, target
        // Should have 1 travel coord between 2 targets = 100% hit rate
        let coords = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 }, // Target 1
            Coord { x: 0.5, y: 0.5 }, // Travel
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 }, // Target 2
        ];

        let rate = average_travel_coord_hit_rate(&coords);
        assert!((rate - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_no_travel() {
        // All same coord - no travel
        let coords = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
        ];

        let rate = average_travel_coord_hit_rate(&coords);
        assert_eq!(rate, 0.0);
    }
}
