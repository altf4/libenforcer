use crate::types::{CheckResult, Coord, Violation};
use crate::utils::is_box_controller;
use std::collections::HashSet;

/// SDI regions for directional input classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SDIRegion {
    DZ = 0,    // Deadzone
    NE = 1,    // Northeast diagonal
    SE = 2,    // Southeast diagonal
    SW = 3,    // Southwest diagonal
    NW = 4,    // Northwest diagonal
    N = 5,     // North cardinal
    E = 6,     // East cardinal
    S = 7,     // South cardinal
    W = 8,     // West cardinal
    TILT = 9,  // Tilt zone (between deadzone and full directions)
}

const DIAGONALS: [SDIRegion; 4] = [
    SDIRegion::NE,
    SDIRegion::SE,
    SDIRegion::SW,
    SDIRegion::NW,
];

const CARDINALS: [SDIRegion; 4] = [
    SDIRegion::N,
    SDIRegion::E,
    SDIRegion::S,
    SDIRegion::W,
];

/// Classify a coordinate into an SDI region
pub fn get_sdi_region(x: f64, y: f64) -> SDIRegion {
    // Deadzone
    if x.abs() <= 0.2875 && y.abs() <= 0.2875 {
        return SDIRegion::DZ;
    }

    let magnitude = (x.powi(2) + y.powi(2)).sqrt();

    // Diagonals (need both X and Y beyond threshold AND sufficient magnitude)
    if x >= 0.2875 && y >= 0.2875 && magnitude >= 0.7 {
        return SDIRegion::NE;
    }
    if x >= 0.2875 && y <= -0.2875 && magnitude >= 0.7 {
        return SDIRegion::SE;
    }
    if x <= -0.2875 && y <= -0.2875 && magnitude >= 0.7 {
        return SDIRegion::SW;
    }
    if x <= -0.2875 && y >= 0.2875 && magnitude >= 0.7 {
        return SDIRegion::NW;
    }

    // Cardinals (magnitude must be >= 0.7 if we reach here)
    if y >= 0.7 {
        return SDIRegion::N;
    }
    if x >= 0.7 {
        return SDIRegion::E;
    }
    if y <= -0.7 {
        return SDIRegion::S;
    }
    if x <= -0.7 {
        return SDIRegion::W;
    }

    // Everything else is tilt zone
    SDIRegion::TILT
}

/// Check if two regions are directly adjacent
pub fn is_region_adjacent(region_a: SDIRegion, region_b: SDIRegion) -> bool {
    use SDIRegion::*;
    match region_a {
        N => matches!(region_b, NW | NE),
        NE => matches!(region_b, N | E),
        E => matches!(region_b, NE | SE),
        SE => matches!(region_b, E | S),
        S => matches!(region_b, SE | SW),
        SW => matches!(region_b, S | W),
        W => matches!(region_b, SW | NW),
        NW => matches!(region_b, W | N),
        _ => false,
    }
}

/// Check if two diagonal regions are adjacent (skipping cardinals)
pub fn is_diagonal_adjacent(region_a: SDIRegion, region_b: SDIRegion) -> bool {
    use SDIRegion::*;

    if !DIAGONALS.contains(&region_a) || !DIAGONALS.contains(&region_b) {
        return false;
    }

    match region_a {
        NE => matches!(region_b, NW | SE),
        NW => matches!(region_b, NE | SW),
        SW => matches!(region_b, SE | NW),
        SE => matches!(region_b, NE | SW),
        _ => false,
    }
}

/// Count unique coordinates in a slice
fn count_unique_coordinates(coords: &[Coord]) -> usize {
    let mut unique = HashSet::new();
    for coord in coords {
        unique.insert((coord.x.to_bits(), coord.y.to_bits()));
    }
    unique.len()
}

/// SDI Rule #1: Rapidly tapping the same direction and returning to neutral
/// faster than once every 5.5 frames triggers 1 SDI and ignores subsequent attempts
pub fn fails_sdi_rule_one(coords: &[Coord]) -> Vec<Violation> {
    let mut violations = Vec::new();

    // Convert all coords to regions
    let regions: Vec<SDIRegion> = coords.iter().map(|c| get_sdi_region(c.x, c.y)).collect();

    for (i, &region) in regions.iter().enumerate() {
        // Look ahead 10 frames when starting from deadzone
        if region != SDIRegion::DZ {
            continue;
        }

        let mut last_region = SDIRegion::DZ;
        let mut first_sdi_region: Option<SDIRegion> = None;
        let mut last_sdi_frame: i32 = -1000;
        let mut consecutive_tilt_frames = 0;
        let mut has_touched_dz = true; // Must touch deadzone for SDI to count

        for j in 1..=9 {
            if i + j >= regions.len() {
                break;
            }

            let current_region = regions[i + j];

            if current_region == SDIRegion::DZ {
                has_touched_dz = true;
            }

            if current_region == SDIRegion::TILT {
                consecutive_tilt_frames += 1;
            } else {
                // Get the first SDI region
                if current_region != SDIRegion::DZ && first_sdi_region.is_none() {
                    first_sdi_region = Some(current_region);
                }
            }

            // If we went from DZ/TILT to the first SDI region
            // And haven't spent more than 3 consecutive frames in tilt zone
            if has_touched_dz
                && (last_region == SDIRegion::DZ || last_region == SDIRegion::TILT)
                && Some(current_region) == first_sdi_region
                && consecutive_tilt_frames <= 3
            {
                let current_frame = (i + j) as i32;

                if current_frame <= last_sdi_frame + 4 {
                    // This is a hack to be lenient as long as there's travel time
                    if count_unique_coordinates(&coords[i..i + j]) <= 2 {
                        // Two SDI frames were less than 5 frames away from each other!
                        let evidence = coords[i..(i + 10).min(coords.len())].to_vec();
                        violations.push(Violation::with_evidence(
                            i as f64,
                            "Failed SDI rule #1".to_string(),
                            evidence,
                        ));
                    }
                }

                last_sdi_frame = current_frame;
                has_touched_dz = false; // Reset the DZ counter
            }

            last_region = current_region;

            if current_region != SDIRegion::TILT {
                consecutive_tilt_frames = 0;
            }
        }
    }

    violations
}

/// SDI Rule #2: Rapidly tapping the same diagonal and returning to an adjacent cardinal
/// faster than once every 5.5 frames
pub fn fails_sdi_rule_two(coords: &[Coord]) -> Vec<Violation> {
    let mut violations = Vec::new();

    let regions: Vec<SDIRegion> = coords.iter().map(|c| get_sdi_region(c.x, c.y)).collect();

    for i in 0..regions.len() {
        let starting_region = regions[i];

        // Start from a cardinal
        if !CARDINALS.contains(&starting_region) {
            continue;
        }

        // Look 4 frames ahead
        // Do we alternate between here and an adjacent diagonal twice?
        let mut sdi_count = 0;
        let mut adjacent_diagonal_region: Option<SDIRegion> = None;

        for j in 1..=4 {
            if i + j >= regions.len() {
                break;
            }

            // Ignore if we haven't moved regions
            if regions[i + j] == regions[i + j - 1] {
                continue;
            }

            // Have we hit the diagonal SDI? (an adjacent diagonal)
            if is_region_adjacent(starting_region, regions[i + j])
                && DIAGONALS.contains(&regions[i + j])
            {
                if adjacent_diagonal_region.is_none()
                    || adjacent_diagonal_region == Some(regions[i + j])
                {
                    adjacent_diagonal_region = Some(regions[i + j]);
                    sdi_count += 1;
                }
            }
        }

        if sdi_count >= 2 {
            let evidence = coords[i..(i + 5).min(coords.len())].to_vec();
            violations.push(Violation::with_evidence(
                i as f64,
                "Failed SDI rule #2".to_string(),
                evidence,
            ));
        }
    }

    violations
}

/// SDI Rule #3: Alternating between adjacent diagonals
pub fn fails_sdi_rule_three(coords: &[Coord]) -> Vec<Violation> {
    let mut violations = Vec::new();

    let regions: Vec<SDIRegion> = coords.iter().map(|c| get_sdi_region(c.x, c.y)).collect();

    for i in 0..regions.len() {
        let current_region = regions[i];

        if !DIAGONALS.contains(&current_region) {
            continue;
        }

        // Look forward 4 frames to see if it goes to an adjacent diagonal and back
        let mut hit_adjacent = false;

        for j in (i + 1)..=(i + 4).min(regions.len() - 1) {
            // Hit the adjacent diagonal
            if is_diagonal_adjacent(regions[j], current_region) {
                hit_adjacent = true;
            }

            // Then returned back
            if hit_adjacent && regions[j] == current_region {
                let evidence = coords[i..(i + 5).min(coords.len())].to_vec();
                violations.push(Violation::with_evidence(
                    i as f64,
                    "Failed SDI rule #3".to_string(),
                    evidence,
                ));
                break;
            }
        }
    }

    violations
}

/// Check for illegal SDI patterns
/// Only applies to box controllers
pub fn check(coords: &[Coord]) -> CheckResult {
    // Only applies to box controllers
    if !is_box_controller(coords) {
        return CheckResult::pass();
    }

    let mut all_violations = Vec::new();

    all_violations.extend(fails_sdi_rule_one(coords));
    all_violations.extend(fails_sdi_rule_two(coords));
    all_violations.extend(fails_sdi_rule_three(coords));

    if all_violations.is_empty() {
        CheckResult::pass()
    } else {
        CheckResult::fail(all_violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdi_region_classification() {
        assert_eq!(get_sdi_region(0.0, 0.0), SDIRegion::DZ);
        assert_eq!(get_sdi_region(0.9, 0.9), SDIRegion::NE);
        assert_eq!(get_sdi_region(0.9, 0.0), SDIRegion::E);
        assert_eq!(get_sdi_region(0.0, 0.9), SDIRegion::N);
        assert_eq!(get_sdi_region(-0.9, -0.9), SDIRegion::SW);
    }

    #[test]
    fn test_region_adjacency() {
        assert!(is_region_adjacent(SDIRegion::N, SDIRegion::NE));
        assert!(is_region_adjacent(SDIRegion::N, SDIRegion::NW));
        assert!(!is_region_adjacent(SDIRegion::N, SDIRegion::S));
    }

    #[test]
    fn test_diagonal_adjacency() {
        assert!(is_diagonal_adjacent(SDIRegion::NE, SDIRegion::NW));
        assert!(is_diagonal_adjacent(SDIRegion::NE, SDIRegion::SE));
        assert!(!is_diagonal_adjacent(SDIRegion::NE, SDIRegion::SW));
    }
}
