use crate::types::Coord;
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

    let rim_count = count_rim_coords(coordinates);
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
