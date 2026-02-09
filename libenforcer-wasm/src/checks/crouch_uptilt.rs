use crate::types::{CheckResult, Coord, Violation};
use crate::utils::is_box_controller;

/// Check for impossibly fast crouch-to-uptilt transitions
/// Human reaction time makes transitions <3 frames impossible
/// Only applies to box controllers
pub fn check(coords: &[Coord], action_states: &[u16]) -> CheckResult {
    // Only applies to box controllers
    if !is_box_controller(coords) {
        return CheckResult::pass();
    }

    const CROUCH_STATE: u16 = 0x28;
    const UPTILT_STATE: u16 = 0x38;

    let mut violations = Vec::new();
    let mut last_crouch_frame: i32 = -124;

    for (i, &action_state) in action_states.iter().enumerate() {
        let frame_number = i as i32 - 123; // Frames start at -123

        // Track when player is crouching
        if action_state == CROUCH_STATE {
            last_crouch_frame = frame_number;
        }

        // Check for uptilt
        if action_state == UPTILT_STATE {
            let frames_since_crouch = frame_number - last_crouch_frame;

            // If uptilt occurs within 3 frames of crouch, it's suspicious
            if frames_since_crouch <= 3 {
                // Get evidence coordinates (4 frames worth)
                let evidence_start = (last_crouch_frame + 123) as usize;
                let evidence_end = (evidence_start + 4).min(coords.len());
                let evidence = coords[evidence_start..evidence_end].to_vec();

                violations.push(Violation::with_evidence(
                    last_crouch_frame as f64,
                    format!(
                        "Crouch-uptilt occurred within {} frames (frame {} to {})",
                        frames_since_crouch, last_crouch_frame, frame_number
                    ),
                    evidence,
                ));
            }
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
    fn test_legal_crouch_uptilt() {
        // Box controller coords (minimal for test)
        let coords = vec![Coord { x: 0.0, y: 0.0 }; 200];

        // Crouch at frame 0, uptilt at frame 10 - legal (>3 frames)
        let mut action_states = vec![0; 200];
        action_states[123] = 0x28; // Frame 0 (index 123 since frames start at -123)
        action_states[133] = 0x38; // Frame 10

        let result = check(&coords, &action_states);
        assert_eq!(result.result, false);
    }

    #[test]
    fn test_illegal_crouch_uptilt() {
        // Box controller coords
        let coords = vec![Coord { x: 1.0, y: 1.0 }; 200]; // Rim coords = box

        // Crouch at frame 0, uptilt at frame 2 - illegal (<=3 frames)
        let mut action_states = vec![0; 200];
        action_states[123] = 0x28; // Frame 0
        action_states[125] = 0x38; // Frame 2

        let result = check(&coords, &action_states);
        assert_eq!(result.result, true);
        assert!(result.violations.len() > 0);
    }
}
