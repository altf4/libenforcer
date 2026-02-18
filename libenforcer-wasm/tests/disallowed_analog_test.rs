//! Integration tests for disallowed analog values check
//! Mirrors TypeScript tests in src/tests/disallowed_analog_values.test.ts
//! Test count: 5

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::disallowed_analog, parser, types::Coord};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_should_pass_check_for_disallowed_c_stick_values() {
    let coords = vec![Coord { x: 0.0, y: 0.0 }, Coord { x: 1.0, y: 1.0 }];

    let result = disallowed_analog::get_cstick_violations(&coords);
    assert_eq!(result.violations.len(), 0, "Should have no violations");
}

#[test]
fn test_should_trigger_check_for_disallowed_c_stick_values_08() {
    let coords = vec![Coord { x: 0.8, y: 0.0 }];

    let result = disallowed_analog::get_cstick_violations(&coords);
    assert_eq!(
        result.violations.len(),
        1,
        "Should have 1 violation for x=0.8"
    );
}

#[test]
fn test_should_trigger_check_for_disallowed_c_stick_values_06625() {
    let coords = vec![Coord { x: 0.6625, y: 0.0 }];

    let result = disallowed_analog::get_cstick_violations(&coords);
    assert_eq!(
        result.violations.len(),
        1,
        "Should have 1 violation for x=0.6625"
    );
}

#[test]
fn test_full_game_with_disallowed_c_stick_value() {
    let data = read_slp_file("banned_c_stick_analog_player_1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = disallowed_analog::check(&player_data.c_coords);
    assert_eq!(
        result.result, true,
        "banned_c_stick_analog_player_1.slp should fail C-stick check"
    );
}
