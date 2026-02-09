//! Integration tests for travel time check
//! Mirrors TypeScript tests in src/tests/travel_time.test.ts
//! Test count: 4

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::travel_time, parser, types::Coord};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_average_travel_time() {
    // AAA BB C DD
    // Three targets, one travel
    // Two conversions between travel points. Thus 50%
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 1.0 },
        Coord { x: 1.0, y: 1.0 },
        Coord { x: 0.5, y: 0.5 },
        Coord { x: -1.0, y: -1.0 },
        Coord { x: -1.0, y: -1.0 },
    ];

    let rate = travel_time::average_travel_coord_hit_rate(&coords);
    assert_float_approx(rate, 0.5, 0.01);
}

#[test]
fn test_average_travel_time_from_legal_digital_file() {
    for i in 1..=7 {
        let path = format!("legal/digital/potion_p3/potion_{}.slp", i);
        let data = read_slp_file(&path);
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 2).unwrap();

        let result = travel_time::check(&player_data.main_coords);
        assert_eq!(
            result.result, false,
            "Potion {} should pass travel time check",
            i
        );

        let hit_rate = travel_time::average_travel_coord_hit_rate(&player_data.main_coords);
        assert!(
            hit_rate > 0.30,
            "Potion {} should have hit rate > 0.30, got {}",
            i,
            hit_rate
        );
    }
}

#[test]
fn test_average_travel_time_from_legal_analog_file() {
    let files = read_slp_dir("legal/analog/traveltime/");

    for (filename, data) in files {
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 2).unwrap();

        let result = travel_time::check(&player_data.main_coords);
        assert_eq!(
            result.result, false,
            "{} should pass travel time check",
            filename
        );

        let hit_rate = travel_time::average_travel_coord_hit_rate(&player_data.main_coords);
        assert!(
            hit_rate > 0.85,
            "{} should have hit rate > 0.85, got {}",
            filename,
            hit_rate
        );
    }
}

#[test]
fn test_average_travel_time_from_nonlegal_digital_file() {
    let files = read_slp_dir("nonlegal/digital/pre-ruleset/");

    for (filename, data) in files {
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 3).unwrap();

        let result = travel_time::check(&player_data.main_coords);
        assert_eq!(
            result.result, true,
            "{} should fail travel time check",
            filename
        );

        let hit_rate = travel_time::average_travel_coord_hit_rate(&player_data.main_coords);
        assert!(
            hit_rate < 0.20,
            "{} should have hit rate < 0.20, got {}",
            filename,
            hit_rate
        );
    }
}
