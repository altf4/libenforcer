//! Integration tests for crouch uptilt check
//! Mirrors TypeScript tests in src/tests/crouch_uptilt.test.ts
//! Test count: 3

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::crouch_uptilt, parser, types::Coord};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_crouch_uptilt_nonlegal() {
    let data = read_slp_file("nonlegal/digital/crouch_uptilt/crouch_uptilt_unnerfed.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 3).unwrap();

    let result = crouch_uptilt::check(&player_data.main_coords, &player_data.action_states);
    assert_eq!(
        result.result, true,
        "crouch_uptilt_unnerfed.slp should fail crouch uptilt check"
    );
    assert_eq!(
        result.details.len(),
        6,
        "Should have exactly 6 violations"
    );

    // All violations should start with evidence {x: 0, y: -1}
    for violation in &result.details {
        assert!(
            !violation.evidence.is_empty(),
            "Violation should have evidence"
        );
        let first_coord = &violation.evidence[0];
        assert_eq!(
            *first_coord,
            Coord { x: 0.0, y: -1.0 },
            "First evidence coordinate should be {{x: 0, y: -1}}"
        );
    }
}

#[test]
fn test_crouch_uptilt_legal() {
    let data = read_slp_file("legal/digital/crouch_uptilt_r18_v2.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = crouch_uptilt::check(&player_data.main_coords, &player_data.action_states);
    assert_eq!(
        result.result, false,
        "crouch_uptilt_r18_v2.slp should pass crouch uptilt check"
    );
}

#[test]
fn test_crouch_uptilt_doubles_with_blank_player() {
    // Doubles sometimes has blank entries for players. Handle this without crashing
    let data = read_slp_file("legal/doubles_with_blank_player.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = crouch_uptilt::check(&player_data.main_coords, &player_data.action_states);
    assert_eq!(
        result.result, false,
        "doubles_with_blank_player.slp should pass crouch uptilt check"
    );
}
