//! Integration tests for input fuzzing check
//!
//! Test data:
//!   legal/digital/techno_p1/ — Player 0: fuzzed (legal), Player 1: unfuzzed (illegal)
//!   legal/digital/carvac_23.1/ — Box player port varies, all fuzzed (legal)
//!   nonlegal/digital/pre-ruleset/ — Player index 3: unfuzzed (illegal)

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::input_fuzzing, parser, types::Coord, utils::is_box_controller};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;

// ---- Existing tests (techno_p1) ----

#[test]
fn test_input_fuzzing_legal_player() {
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

// ---- Manual unit-style integration tests ----

#[test]
fn test_input_fuzzing_manual_no_fuzzing_2d() {
    let target = Coord::new(0.5, 0.5);
    let mut coords = Vec::new();

    for _ in 0..10 {
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.3, 0.3));
        coords.push(target);
        coords.push(target);
    }
    coords.push(Coord::new(0.0, 0.0));
    coords.push(Coord::new(0.0, 0.0));

    let holds = input_fuzzing::identify_holds(&coords);
    let analysis = input_fuzzing::analyze(&coords);

    assert!(holds.len() >= 10, "Should identify at least 10 holds");
    // With only 10 events, may not have enough data for chi-squared,
    // but LLR should be negative (all deltas are 0)
    assert!(
        analysis.llr_score < 0.0,
        "LLR should be negative for unfuzzed data, got {}",
        analysis.llr_score
    );
}

#[test]
fn test_input_fuzzing_manual_with_fuzzing_2d() {
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

    let analysis = input_fuzzing::analyze(&coords);
    // Has some fuzzing, LLR should be positive or at least not strongly negative
    assert!(
        analysis.llr_score > -0.5,
        "LLR should not be strongly negative with some fuzzing, got {}",
        analysis.llr_score
    );
}

#[test]
fn test_input_fuzzing_manual_no_fuzzing_1d() {
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

    let analysis = input_fuzzing::analyze(&coords);
    assert!(
        analysis.llr_score < 0.0,
        "LLR should be negative for unfuzzed 1D data, got {}",
        analysis.llr_score
    );
}

#[test]
fn test_input_fuzzing_cardinals_exempt() {
    let cardinal = Coord::new(1.0, 0.0);
    let mut coords = Vec::new();

    for _ in 0..20 {
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.5, 0.0));
        coords.push(cardinal);
        coords.push(cardinal);
    }

    let analysis = input_fuzzing::analyze(&coords);
    assert_eq!(
        analysis.total_fuzz_events, 0,
        "Cardinal coordinates should produce no fuzz events"
    );
    assert!(analysis.pass, "Cardinals should always pass");
}

// ---- Validation tests: carvac_23.1 (known legal box controller) ----

#[test]
fn test_analyze_carvac_legal_box_player() {
    let files = read_slp_dir("legal/digital/carvac_23.1");
    assert!(!files.is_empty(), "Should find carvac replay files");

    for (filename, data) in &files {
        let game = read_slippi(&mut Cursor::new(data), None).unwrap();

        // Find the box controller player by testing each port
        let mut found_box = false;
        for port in 0..4 {
            if let Some(player_data) = parser::extract_player_data(&game, port) {
                if is_box_controller(&player_data.main_coords) {
                    let analysis = input_fuzzing::analyze(&player_data.main_coords);

                    eprintln!(
                        "[carvac LEGAL] {} port {}: LLR={:.4}, events={}, x=[{},{},{}], y=[{},{},{}], p_x={:?}, p_y={:?}",
                        filename, port,
                        analysis.llr_score, analysis.total_fuzz_events,
                        analysis.observed_x[0], analysis.observed_x[1], analysis.observed_x[2],
                        analysis.observed_y[0], analysis.observed_y[1], analysis.observed_y[2],
                        analysis.p_value_x, analysis.p_value_y,
                    );

                    assert!(
                        analysis.pass,
                        "{} port {} (legal box) should PASS. LLR={:.4}, events={}, violations={:?}",
                        filename, port, analysis.llr_score, analysis.total_fuzz_events, analysis.violations
                    );

                    found_box = true;
                    break;
                }
            }
        }
        assert!(found_box, "{}: no box controller player found", filename);
    }
}

// ---- Validation tests: pre-ruleset (known illegal, player index 3) ----

#[test]
fn test_analyze_preruleset_illegal_player() {
    let files = read_slp_dir("nonlegal/digital/pre-ruleset");
    assert!(!files.is_empty(), "Should find pre-ruleset replay files");

    for (filename, data) in &files {
        let game = read_slippi(&mut Cursor::new(data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 3).unwrap();

        let analysis = input_fuzzing::analyze(&player_data.main_coords);

        eprintln!(
            "[pre-ruleset ILLEGAL] {} port 3: LLR={:.4}, events={}, x=[{},{},{}], y=[{},{},{}], p_x={:?}, p_y={:?}",
            filename,
            analysis.llr_score, analysis.total_fuzz_events,
            analysis.observed_x[0], analysis.observed_x[1], analysis.observed_x[2],
            analysis.observed_y[0], analysis.observed_y[1], analysis.observed_y[2],
            analysis.p_value_x, analysis.p_value_y,
        );

        assert!(
            !analysis.pass,
            "{} port 3 (illegal) should FAIL. LLR={:.4}",
            filename, analysis.llr_score
        );
        assert!(
            analysis.llr_score < 0.0,
            "{} port 3 (illegal) LLR should be negative, got {:.4}",
            filename, analysis.llr_score
        );
    }
}

// ---- Validation tests: techno_p1 with analyze() ----

#[test]
fn test_analyze_techno_legal_player() {
    for i in 1..=5 {
        let data = read_slp_file(&format!(
            "legal/digital/techno_p1/Steech_vs_techno_G{}.slp",
            i
        ));
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 0).unwrap();

        let analysis = input_fuzzing::analyze(&player_data.main_coords);

        eprintln!(
            "[techno LEGAL] G{} port 0: LLR={:.4}, events={}, p_x={:?}, p_y={:?}",
            i, analysis.llr_score, analysis.total_fuzz_events,
            analysis.p_value_x, analysis.p_value_y,
        );

        assert!(
            analysis.pass,
            "G{} player 0 (fuzzed) should PASS. LLR={:.4}",
            i, analysis.llr_score
        );
    }
}

#[test]
fn test_analyze_techno_illegal_player() {
    for i in 1..=5 {
        let data = read_slp_file(&format!(
            "legal/digital/techno_p1/Steech_vs_techno_G{}.slp",
            i
        ));
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 1).unwrap();

        let analysis = input_fuzzing::analyze(&player_data.main_coords);

        eprintln!(
            "[techno ILLEGAL] G{} port 1: LLR={:.4}, events={}, p_x={:?}, p_y={:?}",
            i, analysis.llr_score, analysis.total_fuzz_events,
            analysis.p_value_x, analysis.p_value_y,
        );

        assert!(
            !analysis.pass,
            "G{} player 1 (unfuzzed) should FAIL. LLR={:.4}",
            i, analysis.llr_score
        );
    }
}
