//! Integration tests for control stick visualization check
//! Mirrors TypeScript tests in src/tests/control_stick_viz.test.ts
//! Test count: 1

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{checks::control_stick_viz, parser};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;

#[test]
fn test_control_stick_viz_sanity_check() {
    let data = read_slp_file("nonlegal/analog/goomwave_uptilt_p1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = control_stick_viz::check(&player_data.main_coords);
    assert_eq!(result.result, false);
    assert_eq!(result.details.len(), 1);
    assert_eq!(
        result.details[0].evidence.len(),
        4845,
        "Expected 4845 evidence points for goomwave_uptilt_p1.slp"
    );
}
