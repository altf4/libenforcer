use crate::types::{Coord, JoystickRegion};
use std::collections::HashSet;

/// Float equality comparison with epsilon tolerance
pub fn float_equals(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0001
}

/// Check if two coordinates are equal (within float tolerance)
pub fn is_equal_coord(one: &Coord, other: &Coord) -> bool {
    float_equals(one.x, other.x) && float_equals(one.y, other.y)
}

/// Determines if the player is using a box controller (digital)
/// Based on how many unique rim coordinates they hit
/// Box controllers hit fewer rim coordinates than analog sticks
pub fn is_box_controller(coordinates: &[Coord]) -> bool {
    const RIM_COORD_MAX: usize = 432;
    const THREE_MINUTES: usize = 10800; // frames
    const MIN_ANALOG_SMALL_OFF_AXIS_RATIO: f64 = 0.02;
    const MIN_ANALOG_SMALL_OFF_AXIS_UNIQUE: usize = 32;
    const MIN_ANALOG_CARDINAL_UNIQUE: usize = 100;
    const MIN_ANALOG_RIM_UNIQUE: usize = 40;

    let (small_off_axis_count, small_off_axis_unique_count) =
        count_small_off_axis_coords(coordinates);
    let small_off_axis_ratio = if coordinates.is_empty() {
        0.0
    } else {
        small_off_axis_count as f64 / coordinates.len() as f64
    };

    if small_off_axis_ratio >= MIN_ANALOG_SMALL_OFF_AXIS_RATIO
        && small_off_axis_unique_count >= MIN_ANALOG_SMALL_OFF_AXIS_UNIQUE
    {
        return false;
    }

    let rim_count = count_rim_coords(coordinates);

    if count_strict_rim_coords(coordinates) >= MIN_ANALOG_RIM_UNIQUE {
        let (_, cardinal_analog_unique_count) = count_cardinal_analog_coords(coordinates);

        if cardinal_analog_unique_count >= MIN_ANALOG_CARDINAL_UNIQUE {
            return false;
        }
    }

    let mut rim_proportion = rim_count as f64 / RIM_COORD_MAX as f64;

    // Boost proportion for shorter games to avoid false positives
    // Shorter games naturally have fewer rim coordinates
    if coordinates.len() < THREE_MINUTES {
        let boost = 1.0 + ((THREE_MINUTES - coordinates.len()) as f64 / THREE_MINUTES as f64);
        rim_proportion *= boost;
    }

    // If less than 50% of rim coordinates hit, it's likely a box controller
    rim_proportion < 0.50
}

/// Count coordinates with natural analog off-axis evidence.
/// These are coordinates where both axes are active, but one axis is a small
/// nonzero value. Box controllers should almost never produce many of these.
fn count_small_off_axis_coords(coords: &[Coord]) -> (usize, usize) {
    const SMALL_OFF_AXIS_THRESHOLD: f64 = 0.08;

    let mut count = 0;
    let mut unique_coords = HashSet::new();

    for coord in coords {
        let min_abs_axis = coord.x.abs().min(coord.y.abs());
        if min_abs_axis > 0.0 && min_abs_axis < SMALL_OFF_AXIS_THRESHOLD {
            count += 1;
            unique_coords.insert((coord.x.to_bits(), coord.y.to_bits()));
        }
    }

    (count, unique_coords.len())
}

/// Count unique coordinates that are actually on the rim, without fuzz tolerance.
/// The box fallback uses tolerant rim detection; this stricter version avoids
/// treating normal one-step fuzz around full cardinals as analog rim coverage.
fn count_strict_rim_coords(coords: &[Coord]) -> usize {
    let mut rim_coords = HashSet::new();

    for coord in coords {
        let distance = (coord.x.powi(2) + coord.y.powi(2)).sqrt();

        if distance >= 1.0 {
            rim_coords.insert((coord.x.to_bits(), coord.y.to_bits()));
        }
    }

    rim_coords.len()
}

/// Count axis-aligned coordinates with analog magnitudes below full cardinal.
/// This catches analog-button traces that have little off-axis noise, while the
/// rim-coordinate requirement keeps ordinary fuzzed box inputs classified as box.
fn count_cardinal_analog_coords(coords: &[Coord]) -> (usize, usize) {
    let mut count = 0;
    let mut unique_coords = HashSet::new();

    for coord in coords {
        let min_abs_axis = coord.x.abs().min(coord.y.abs());
        let max_abs_axis = coord.x.abs().max(coord.y.abs());

        if min_abs_axis == 0.0 && max_abs_axis > 0.0 && max_abs_axis < 1.0 {
            count += 1;
            unique_coords.insert((coord.x.to_bits(), coord.y.to_bits()));
        }
    }

    (count, unique_coords.len())
}

/// Count unique coordinates on the rim of the joystick
/// A coordinate is on the rim if its distance from center is >= 1.0
fn count_rim_coords(coords: &[Coord]) -> usize {
    let mut rim_coords = HashSet::new();

    for coord in coords {
        // Calculate distance from center with slight tolerance
        let distance = ((coord.x.abs() + 0.0125).powi(2) + (coord.y.abs() + 0.0125).powi(2)).sqrt();

        if distance >= 1.0 {
            // Use float bits for precise hashing
            rim_coords.insert((coord.x.to_bits(), coord.y.to_bits()));
        }
    }

    rim_coords.len()
}

/// Get unique coordinates from a list
pub fn get_unique_coords(coordinates: &[Coord]) -> Vec<Coord> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for coord in coordinates {
        let key = (coord.x.to_bits(), coord.y.to_bits());
        if seen.insert(key) {
            unique.push(*coord);
        }
    }

    unique
}

/// Get "target" coordinates - coords where the stick dwelled for 2+ frames
/// This removes travel/transition coordinates
pub fn get_target_coords(coordinates: &[Coord]) -> Vec<Coord> {
    if coordinates.is_empty() {
        return vec![];
    }

    let mut targets = HashSet::new();
    let mut last_coord: Option<Coord> = None;

    for coord in coordinates {
        if let Some(last) = last_coord {
            if is_equal_coord(&last, coord) {
                targets.insert((coord.x.to_bits(), coord.y.to_bits()));
            }
        }
        last_coord = Some(*coord);
    }

    targets
        .into_iter()
        .map(|(x_bits, y_bits)| Coord {
            x: f64::from_bits(x_bits),
            y: f64::from_bits(y_bits),
        })
        .collect()
}

/// Classify joystick position into one of 9 regions
/// Mirrors TypeScript getJoystickRegion() from index.ts
pub fn get_joystick_region(x: f64, y: f64) -> JoystickRegion {
    if x >= 0.2875 && y >= 0.2875 {
        JoystickRegion::NE
    } else if x >= 0.2875 && y <= -0.2875 {
        JoystickRegion::SE
    } else if x <= -0.2875 && y <= -0.2875 {
        JoystickRegion::SW
    } else if x <= -0.2875 && y >= 0.2875 {
        JoystickRegion::NW
    } else if y >= 0.2875 {
        JoystickRegion::N
    } else if x >= 0.2875 {
        JoystickRegion::E
    } else if y <= -0.2875 {
        JoystickRegion::S
    } else if x <= -0.2875 {
        JoystickRegion::W
    } else {
        JoystickRegion::DZ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_equals() {
        assert!(float_equals(1.0, 1.00009));
        assert!(float_equals(0.0, 0.00009));
        assert!(!float_equals(1.0, 1.001));
    }

    #[test]
    fn test_is_equal_coord() {
        let c1 = Coord { x: 1.0, y: 0.5 };
        let c2 = Coord { x: 1.00009, y: 0.50009 };
        let c3 = Coord { x: 1.1, y: 0.5 };

        assert!(is_equal_coord(&c1, &c2));
        assert!(!is_equal_coord(&c1, &c3));
    }

    #[test]
    fn test_rim_detection() {
        let rim_coords = vec![
            Coord { x: 1.0, y: 0.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: -1.0, y: 0.0 },
            Coord { x: 0.7071, y: 0.7071 }, // 45 degrees
        ];

        let count = count_rim_coords(&rim_coords);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_get_target_coords() {
        let coords = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 }, // Target
            Coord { x: 0.5, y: 0.5 }, // Travel
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 }, // Target
        ];

        let targets = get_target_coords(&coords);
        assert_eq!(targets.len(), 2);
    }
}
