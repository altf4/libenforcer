//! Integration tests for goomwave check
//! Mirrors TypeScript tests in src/tests/goomwave.test.ts
//! Test count: 4

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::goomwave, parser};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_is_goomwave_negative() {
    for i in 1..=7 {
        let path = format!("legal/digital/potion_p3/potion_{}.slp", i);
        let data = read_slp_file(&path);
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 2).unwrap();

        let result = goomwave::check(&player_data.main_coords);
        assert_eq!(
            result.result, false,
            "Potion {} should not be detected as goomwave",
            i
        );
    }
}

#[test]
fn test_is_goomwave_a_positive() {
    // TypeScript test calls hasGoomwaveClamping() directly (not isGoomwave),
    // because this replay's controller has too few rim coords for isBoxController
    let data = read_slp_file("nonlegal/analog/goomwave_uptilt_p1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        goomwave::has_goomwave_clamping(&player_data.main_coords), true,
        "goomwave_uptilt_p1.slp should have goomwave clamping"
    );
}

#[test]
fn test_is_goomwave_b_positive() {
    let data = read_slp_file("nonlegal/analog/goomwave/Game_20250216T194607.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    let result = goomwave::check(&player_data.main_coords);
    assert_eq!(
        result.result, true,
        "Game_20250216T194607.slp should be detected as goomwave"
    );
}

#[test]
fn test_is_goomwave_c_positive() {
    let data = read_slp_file("nonlegal/analog/goomwave/Game_20250216T194746.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    let result = goomwave::check(&player_data.main_coords);
    assert_eq!(
        result.result, true,
        "Game_20250216T194746.slp should be detected as goomwave"
    );
}
