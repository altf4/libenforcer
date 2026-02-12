//! Integration tests for input fuzzing check
//! Tests use real SLP replay data from test_data/legal/digital/techno_p1/
//! Player 1 (index 0): legal controller with fuzzed inputs → should PASS
//! Player 2 (index 1): illegal controller with no fuzzed inputs → should FAIL

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::input_fuzzing, parser, types::Coord};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;

#[test]
fn test_input_fuzzing_legal_player() {
    // Player 1 (index 0) in techno_p1 files — fuzzed inputs
    for i in 1..=5 {
        let data = read_slp_file(&format!(
            "legal/digital/techno_p1/Steech_vs_techno_G{}.slp",
            i
        ));
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 0).unwrap();

        let result = input_fuzzing::check(&player_data.main_coords);
        assert_eq!(
            result.result, false,
            "G{} player 1 (fuzzed) should pass fuzzing check",
            i
        );
    }
}

#[test]
fn test_input_fuzzing_nonlegal_player() {
    // Player 2 (index 1) in techno_p1 files — no fuzzing
    for i in 1..=5 {
        let data = read_slp_file(&format!(
            "legal/digital/techno_p1/Steech_vs_techno_G{}.slp",
            i
        ));
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 1).unwrap();

        let result = input_fuzzing::check(&player_data.main_coords);
        assert_eq!(
            result.result, true,
            "G{} player 2 (unfuzzed) should fail fuzzing check",
            i
        );
    }
}

#[test]
fn test_input_fuzzing_manual_no_fuzzing_2d() {
    // Craft a sequence where a non-cardinal coordinate is held 10 separate times
    // with no neighbors — should detect missing 2D fuzzing
    let target = Coord::new(0.5, 0.5);
    let mut coords = Vec::new();

    for _ in 0..10 {
        // Neutral hold
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        // Travel
        coords.push(Coord::new(0.3, 0.3));
        // Target hold (same coordinate every time — no fuzzing)
        coords.push(target);
        coords.push(target);
    }
    coords.push(Coord::new(0.0, 0.0));
    coords.push(Coord::new(0.0, 0.0));

    let holds = input_fuzzing::identify_holds(&coords);
    let violations = input_fuzzing::detect_missing_fuzzing(&holds);
    assert!(
        !violations.is_empty(),
        "Should detect missing 2D fuzzing with 10 identical holds"
    );
}

#[test]
fn test_input_fuzzing_manual_with_fuzzing_2d() {
    // Craft a sequence where a non-cardinal coordinate shows proper variance
    let target = Coord::new(0.5, 0.5);
    let neighbor = Coord::new(0.5 + 1.0 / 80.0, 0.5);

    let mut coords = Vec::new();
    let outputs = [
        target, target, neighbor, target, target,
        neighbor, target, target, neighbor, target,
    ];

    for &output in &outputs {
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.3, 0.3));
        coords.push(output);
        coords.push(output);
    }

    let holds = input_fuzzing::identify_holds(&coords);
    let violations = input_fuzzing::detect_missing_fuzzing(&holds);
    assert_eq!(
        violations.len(),
        0,
        "Should pass with proper 2D fuzzing variance"
    );
}

#[test]
fn test_input_fuzzing_manual_no_fuzzing_1d() {
    // Deadzone coordinate (y=0) held 15 separate times — should detect missing 1D fuzzing
    let target = Coord::new(0.5, 0.0);
    let mut coords = Vec::new();

    for _ in 0..15 {
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.3, 0.0));
        coords.push(target);
        coords.push(target);
    }
    coords.push(Coord::new(0.0, 0.0));
    coords.push(Coord::new(0.0, 0.0));

    let holds = input_fuzzing::identify_holds(&coords);
    let violations = input_fuzzing::detect_missing_fuzzing(&holds);
    assert!(
        !violations.is_empty(),
        "Should detect missing 1D fuzzing with 15 identical holds"
    );
}

#[test]
fn test_input_fuzzing_cardinals_exempt() {
    // Cardinal coordinates repeated many times — should never trigger
    let cardinal = Coord::new(1.0, 0.0);
    let mut coords = Vec::new();

    for _ in 0..20 {
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.5, 0.0));
        coords.push(cardinal);
        coords.push(cardinal);
    }

    let holds = input_fuzzing::identify_holds(&coords);
    let violations = input_fuzzing::detect_missing_fuzzing(&holds);
    assert_eq!(
        violations.len(),
        0,
        "Cardinal coordinates should be exempt from fuzzing"
    );
}
