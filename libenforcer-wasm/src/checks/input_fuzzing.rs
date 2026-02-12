use crate::types::{CheckResult, Coord, Violation};
use crate::utils::{float_equals, is_box_controller, is_equal_coord};
use std::collections::HashMap;

/// One raw coordinate unit in normalized space (1/80)
const UNIT: f64 = 1.0 / 80.0;

/// Minimum separate holds before flagging a 1D (deadzone) coordinate
const MIN_HOLDS_DEADZONE: usize = 12;

/// Minimum separate holds before flagging a 2D (non-cardinal) coordinate
const MIN_HOLDS_NON_CARDINAL: usize = 8;

/// Classification of a coordinate for fuzzing purposes
#[derive(Debug, Clone, Copy, PartialEq)]
enum CoordClass {
    /// (±1.0, 0.0) or (0.0, ±1.0) — exempt from fuzzing
    Cardinal,
    /// (0.0, 0.0) — exempt from fuzzing
    Origin,
    /// One axis is 0.0, other is non-zero and not ±1.0 — 1D fuzzing required
    Deadzone,
    /// Both axes non-zero — 2D fuzzing required
    NonCardinal,
}

/// Classify a coordinate for fuzzing requirements
fn classify_coord(coord: &Coord) -> CoordClass {
    let x_zero = float_equals(coord.x, 0.0);
    let y_zero = float_equals(coord.y, 0.0);
    let x_cardinal = float_equals(coord.x.abs(), 1.0);
    let y_cardinal = float_equals(coord.y.abs(), 1.0);

    if x_zero && y_zero {
        CoordClass::Origin
    } else if (x_cardinal && y_zero) || (y_cardinal && x_zero) {
        CoordClass::Cardinal
    } else if x_zero || y_zero {
        // One axis is zero, the other is non-zero and not ±1.0
        CoordClass::Deadzone
    } else {
        CoordClass::NonCardinal
    }
}

/// Get the expected fuzz neighbor coordinates for a given coordinate and classification
fn get_neighbors(coord: &Coord, class: &CoordClass) -> Vec<Coord> {
    match class {
        CoordClass::Deadzone => {
            if float_equals(coord.y, 0.0) {
                // Y is in deadzone, fuzz along X axis
                let sign = if coord.x > 0.0 { 1.0 } else { -1.0 };
                vec![
                    Coord::new(coord.x + sign * UNIT, coord.y), // further from center
                    Coord::new(coord.x - sign * UNIT, coord.y), // closer to center
                ]
            } else {
                // X is in deadzone, fuzz along Y axis
                let sign = if coord.y > 0.0 { 1.0 } else { -1.0 };
                vec![
                    Coord::new(coord.x, coord.y + sign * UNIT), // further from center
                    Coord::new(coord.x, coord.y - sign * UNIT), // closer to center
                ]
            }
        }
        CoordClass::NonCardinal => {
            // 3×3 grid minus center = 8 neighbors
            let mut neighbors = Vec::with_capacity(8);
            for dx in [-UNIT, 0.0, UNIT] {
                for dy in [-UNIT, 0.0, UNIT] {
                    if float_equals(dx, 0.0) && float_equals(dy, 0.0) {
                        continue;
                    }
                    neighbors.push(Coord::new(coord.x + dx, coord.y + dy));
                }
            }
            neighbors
        }
        // Cardinal and Origin are exempt — no neighbors expected
        _ => vec![],
    }
}

/// A hold is a sequence of 2+ identical consecutive frames (one targeting event)
pub struct Hold {
    pub coord: Coord,
    pub start_frame: usize,
}

/// Identify all holds in the coordinate sequence
/// A hold = 2+ consecutive frames with the same coordinate
pub fn identify_holds(coords: &[Coord]) -> Vec<Hold> {
    let mut holds = Vec::new();
    if coords.is_empty() {
        return holds;
    }

    let mut i = 0;
    while i < coords.len() {
        let start = i;
        while i + 1 < coords.len() && is_equal_coord(&coords[i], &coords[i + 1]) {
            i += 1;
        }
        if i > start {
            // 2+ consecutive identical frames = a hold
            holds.push(Hold {
                coord: coords[start],
                start_frame: start,
            });
        }
        i += 1;
    }

    holds
}

/// Key for hashing coordinates using integer units (multiples of 1/80)
/// This avoids floating point precision issues when comparing neighbors
fn coord_key(coord: &Coord) -> (i32, i32) {
    ((coord.x / UNIT).round() as i32, (coord.y / UNIT).round() as i32)
}

/// Detect missing fuzzing from a list of holds
pub fn detect_missing_fuzzing(holds: &[Hold]) -> Vec<Violation> {
    let mut violations = Vec::new();

    // Build a map of integer coordinate key → (count, first_frame)
    let mut hold_counts: HashMap<(i32, i32), (usize, usize)> = HashMap::new();
    for hold in holds {
        let key = coord_key(&hold.coord);
        let entry = hold_counts.entry(key).or_insert((0, hold.start_frame));
        entry.0 += 1;
        // Keep the earliest frame
        if hold.start_frame < entry.1 {
            entry.1 = hold.start_frame;
        }
    }

    // Check each unique coordinate for missing fuzzing
    for (&key, &(count, first_frame)) in &hold_counts {
        let coord = Coord::new(key.0 as f64 * UNIT, key.1 as f64 * UNIT);
        let class = classify_coord(&coord);

        // Skip exempt coordinates
        match class {
            CoordClass::Cardinal | CoordClass::Origin => continue,
            _ => {}
        }

        // Check threshold
        let threshold = match class {
            CoordClass::Deadzone => MIN_HOLDS_DEADZONE,
            CoordClass::NonCardinal => MIN_HOLDS_NON_CARDINAL,
            _ => unreachable!(),
        };

        if count < threshold {
            continue;
        }

        // Check if any neighbors appear as holds
        let neighbors = get_neighbors(&coord, &class);
        let has_neighbor = neighbors.iter().any(|neighbor| {
            let nkey = coord_key(neighbor);
            hold_counts.contains_key(&nkey)
        });

        if !has_neighbor {
            let fuzz_type = match class {
                CoordClass::Deadzone => "1D deadzone",
                CoordClass::NonCardinal => "2D non-cardinal",
                _ => unreachable!(),
            };
            violations.push(Violation::with_evidence(
                first_frame as f64,
                format!(
                    "No fuzzing variance detected for {} coordinate ({:.4}, {:.4}): {} holds with no neighboring coordinates",
                    fuzz_type, coord.x, coord.y, count
                ),
                vec![coord],
            ));
        }
    }

    violations
}

/// Check for missing input fuzzing on box controllers
/// Box controllers must apply variance to non-cardinal control stick outputs
pub fn check(coords: &[Coord]) -> CheckResult {
    // Only applies to box controllers
    if !is_box_controller(coords) {
        return CheckResult::pass();
    }

    let holds = identify_holds(coords);
    let violations = detect_missing_fuzzing(&holds);

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
    fn test_classify_coordinate() {
        // Cardinals — exempt
        assert_eq!(classify_coord(&Coord::new(1.0, 0.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(-1.0, 0.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(0.0, 1.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(0.0, -1.0)), CoordClass::Cardinal);

        // Origin — exempt
        assert_eq!(classify_coord(&Coord::new(0.0, 0.0)), CoordClass::Origin);

        // Deadzone — one axis zero, other non-zero non-cardinal
        assert_eq!(classify_coord(&Coord::new(0.5, 0.0)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(0.0, 0.5)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(-0.3, 0.0)), CoordClass::Deadzone);

        // Non-cardinal — both axes non-zero
        assert_eq!(classify_coord(&Coord::new(0.5, 0.5)), CoordClass::NonCardinal);
        assert_eq!(classify_coord(&Coord::new(0.7, 0.7)), CoordClass::NonCardinal);
        assert_eq!(classify_coord(&Coord::new(-0.3, 0.4)), CoordClass::NonCardinal);
    }

    #[test]
    fn test_get_neighbors_deadzone() {
        // Y-deadzone: fuzz along X axis
        let coord = Coord::new(0.5, 0.0);
        let neighbors = get_neighbors(&coord, &CoordClass::Deadzone);
        assert_eq!(neighbors.len(), 2);
        // Further from center (positive direction)
        assert!(float_equals(neighbors[0].x, 0.5 + UNIT));
        assert!(float_equals(neighbors[0].y, 0.0));
        // Closer to center
        assert!(float_equals(neighbors[1].x, 0.5 - UNIT));
        assert!(float_equals(neighbors[1].y, 0.0));

        // Negative X: further = more negative, closer = less negative
        let coord = Coord::new(-0.5, 0.0);
        let neighbors = get_neighbors(&coord, &CoordClass::Deadzone);
        assert_eq!(neighbors.len(), 2);
        assert!(float_equals(neighbors[0].x, -0.5 - UNIT)); // further
        assert!(float_equals(neighbors[1].x, -0.5 + UNIT)); // closer
    }

    #[test]
    fn test_get_neighbors_non_cardinal() {
        let coord = Coord::new(0.5, 0.5);
        let neighbors = get_neighbors(&coord, &CoordClass::NonCardinal);
        assert_eq!(neighbors.len(), 8);

        // Verify all 8 neighbors are within ±UNIT
        for n in &neighbors {
            assert!((n.x - coord.x).abs() <= UNIT + 0.0001);
            assert!((n.y - coord.y).abs() <= UNIT + 0.0001);
            // Not the center
            assert!(!is_equal_coord(n, &coord));
        }
    }

    #[test]
    fn test_identify_holds() {
        let coords = vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.5, 0.0), // travel (1 frame)
            Coord::new(0.7, 0.0),
            Coord::new(0.7, 0.0), // hold at (0.7, 0.0)
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0), // hold at (0.0, 0.0)
        ];

        let holds = identify_holds(&coords);
        assert_eq!(holds.len(), 2);
        assert!(is_equal_coord(&holds[0].coord, &Coord::new(0.7, 0.0)));
        assert_eq!(holds[0].start_frame, 2);
        assert!(is_equal_coord(&holds[1].coord, &Coord::new(0.0, 0.0)));
        assert_eq!(holds[1].start_frame, 4);
    }

    /// Helper: create a coordinate sequence with N separate holds at the given coordinate,
    /// interspersed with neutral (0,0) holds
    fn make_repeated_holds(coord: Coord, n: usize) -> Vec<Coord> {
        let mut coords = Vec::new();
        for _ in 0..n {
            // Neutral hold
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.0, 0.0));
            // Travel frame
            coords.push(Coord::new(0.3, 0.3));
            // Target hold
            coords.push(coord);
            coords.push(coord);
        }
        // Extra neutral at the end
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords
    }

    #[test]
    fn test_no_violation_with_fuzzing_1d() {
        // Deadzone coordinate with proper 1D fuzzing variance
        let target = Coord::new(0.5, 0.0);
        let further = Coord::new(0.5 + UNIT, 0.0);
        let closer = Coord::new(0.5 - UNIT, 0.0);

        let mut coords = Vec::new();
        // Simulate 20 targeting events with fuzzing: ~50% target, ~25% each neighbor
        let outputs = [
            target, target, further, target, closer,
            target, further, target, closer, target,
            target, further, target, closer, further,
            target, target, closer, further, target,
        ];

        for &output in &outputs {
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.3, 0.0)); // travel
            coords.push(output);
            coords.push(output);
        }

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert_eq!(violations.len(), 0, "Should pass with 1D fuzzing variance");
    }

    #[test]
    fn test_no_violation_with_fuzzing_2d() {
        // Non-cardinal coordinate with proper 2D fuzzing variance
        let target = Coord::new(0.5, 0.5);
        let neighbor = Coord::new(0.5 + UNIT, 0.5);

        let mut coords = Vec::new();
        // Some holds on target, some on neighbor
        let outputs = [
            target, target, neighbor, target, target,
            neighbor, target, target, neighbor, target,
        ];

        for &output in &outputs {
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.3, 0.3)); // travel
            coords.push(output);
            coords.push(output);
        }

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert_eq!(violations.len(), 0, "Should pass with 2D fuzzing variance");
    }

    #[test]
    fn test_violation_no_fuzzing_1d() {
        // Same deadzone coordinate repeated in 15 separate holds, no neighbors
        let coords = make_repeated_holds(Coord::new(0.5, 0.0), 15);

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert!(
            !violations.is_empty(),
            "Should detect missing 1D fuzzing with 15 holds"
        );
    }

    #[test]
    fn test_violation_no_fuzzing_2d() {
        // Same non-cardinal coordinate repeated in 10 separate holds, no neighbors
        let coords = make_repeated_holds(Coord::new(0.5, 0.5), 10);

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert!(
            !violations.is_empty(),
            "Should detect missing 2D fuzzing with 10 holds"
        );
    }

    #[test]
    fn test_cardinals_exempt() {
        // Cardinal coordinates repeated many times — should not trigger
        let coords = make_repeated_holds(Coord::new(1.0, 0.0), 20);

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert_eq!(violations.len(), 0, "Cardinals should be exempt from fuzzing");
    }

    #[test]
    fn test_below_threshold_passes() {
        // Non-cardinal coordinate with only 3 holds — below threshold
        let coords = make_repeated_holds(Coord::new(0.5, 0.5), 3);

        let holds = identify_holds(&coords);
        let violations = detect_missing_fuzzing(&holds);
        assert_eq!(
            violations.len(),
            0,
            "Below threshold should not trigger violation"
        );
    }
}
