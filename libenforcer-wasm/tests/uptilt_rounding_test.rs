//! Integration tests for uptilt rounding check
//! Mirrors TypeScript tests in src/tests/uptilt_rounding.test.ts
//! Test count: 5

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::uptilt_rounding, parser};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_uptilt_rounding_legal_analog_a() {
    let files = read_slp_dir("legal/analog/traveltime/");

    for (filename, data) in files {
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();

        // Test port 3 (player index 2)
        if let Some(player_data) = parser::extract_player_data(&game, 2) {
            let result = uptilt_rounding::check(&player_data.main_coords);
            assert_eq!(
                result.result, false,
                "{} port 3 should pass uptilt rounding check",
                filename
            );
        }

        // Test port 4 (player index 3)
        if let Some(player_data) = parser::extract_player_data(&game, 3) {
            let result = uptilt_rounding::check(&player_data.main_coords);
            assert_eq!(
                result.result, false,
                "{} port 4 should pass uptilt rounding check",
                filename
            );
        }
    }
}

#[test]
fn test_uptilt_rounding_nonlegal_analog() {
    // TypeScript test calls getUptiltCheck() directly (not hasIllegalUptiltRounding),
    // because this replay's controller has too few rim coords for isBoxController
    let data = read_slp_file("nonlegal/analog/goomwave_uptilt_p1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = uptilt_rounding::get_uptilt_check(&player_data.main_coords);
    assert_eq!(
        result.result, true,
        "goomwave_uptilt_p1.slp should fail uptilt rounding check"
    );
    assert!(
        result.details.len() >= 1,
        "Should have at least 1 violation, got {}",
        result.details.len()
    );
}

#[test]
fn test_uptilt_rounding_legal_analog_b() {
    let data = read_slp_file("legal/analog/Game_20250107T140347.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = uptilt_rounding::check(&player_data.main_coords);
    assert_eq!(
        result.result, false,
        "Game_20250107T140347.slp should pass uptilt rounding check"
    );
    assert_eq!(result.details.len(), 0, "Should have 0 violations");
}

#[test]
fn test_uptilt_rounding_legal_analog_c() {
    let data = read_slp_file("legal/analog/Game_20250107T142211.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = uptilt_rounding::check(&player_data.main_coords);
    assert_eq!(
        result.result, false,
        "Game_20250107T142211.slp should pass uptilt rounding check"
    );
    assert_eq!(result.details.len(), 0, "Should have 0 violations");
}

#[test]
fn test_uptilt_rounding_legal_analog_d() {
    let data = read_slp_file("legal/analog/Game_20250123T212056.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = uptilt_rounding::check(&player_data.main_coords);
    assert_eq!(
        result.result, false,
        "Game_20250123T212056.slp should pass uptilt rounding check"
    );
    assert_eq!(result.details.len(), 0, "Should have 0 violations");
}
