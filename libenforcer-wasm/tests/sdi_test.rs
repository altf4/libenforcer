//! Integration tests for SDI check
//! Mirrors TypeScript tests in src/tests/sdi.test.ts
//! Test count: 12

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::{
    checks::sdi::{self, get_sdi_region, is_diagonal_adjacent, SDIRegion},
    parser,
    types::Coord,
};
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;


#[test]
fn test_region_sanity_check() {
    // DZ - Dead Zone Test
    assert_eq!(get_sdi_region(0.0, 0.0), SDIRegion::DZ);
    assert_eq!(get_sdi_region(0.2, 0.2), SDIRegion::DZ);
    assert_eq!(get_sdi_region(-0.2, -0.2), SDIRegion::DZ);

    // NE - Northeast
    assert_eq!(get_sdi_region(0.8, 0.8), SDIRegion::NE);
    assert_eq!(get_sdi_region(0.9, 0.7), SDIRegion::NE);

    // SE - Southeast
    assert_eq!(get_sdi_region(0.8, -0.8), SDIRegion::SE);
    assert_eq!(get_sdi_region(0.9, -0.7), SDIRegion::SE);

    // SW - Southwest
    assert_eq!(get_sdi_region(-0.8, -0.8), SDIRegion::SW);
    assert_eq!(get_sdi_region(-0.9, -0.7), SDIRegion::SW);

    // NW - Northwest
    assert_eq!(get_sdi_region(-0.8, 0.8), SDIRegion::NW);
    assert_eq!(get_sdi_region(-0.9, 0.7), SDIRegion::NW);

    // N - North
    assert_eq!(get_sdi_region(0.0, 0.8), SDIRegion::N);
    assert_eq!(get_sdi_region(0.2, 0.9), SDIRegion::N);

    // E - East
    assert_eq!(get_sdi_region(0.8, 0.0), SDIRegion::E);
    assert_eq!(get_sdi_region(0.9, 0.2), SDIRegion::E);

    // S - South
    assert_eq!(get_sdi_region(0.0, -0.8), SDIRegion::S);
    assert_eq!(get_sdi_region(0.2, -0.9), SDIRegion::S);

    // W - West
    assert_eq!(get_sdi_region(-0.8, 0.0), SDIRegion::W);
    assert_eq!(get_sdi_region(-0.9, 0.2), SDIRegion::W);

    // TILT - Middle-ish area
    assert_eq!(get_sdi_region(0.4, 0.4), SDIRegion::TILT);
    assert_eq!(get_sdi_region(-0.4, -0.4), SDIRegion::TILT);
    assert_eq!(get_sdi_region(0.2, -0.3), SDIRegion::TILT);

    // Edge cases
    // Testing near the edges of the DZ boundary (0.2875)
    assert_eq!(get_sdi_region(0.2876, 0.0), SDIRegion::TILT);
    assert_eq!(get_sdi_region(0.0, 0.2876), SDIRegion::TILT);
    assert_eq!(get_sdi_region(-0.2876, 0.0), SDIRegion::TILT);
    assert_eq!(get_sdi_region(0.0, -0.2876), SDIRegion::TILT);

    // Test values on the boundary of magnitude 0.7 (diagonal)
    assert_eq!(get_sdi_region(0.7, 0.7), SDIRegion::NE);
    assert_eq!(get_sdi_region(-0.7, 0.7), SDIRegion::NW);
    assert_eq!(get_sdi_region(0.7, -0.7), SDIRegion::SE);
    assert_eq!(get_sdi_region(-0.7, -0.7), SDIRegion::SW);

    // Boundary checks for cardinal directions (0.7)
    assert_eq!(get_sdi_region(0.7, 0.0), SDIRegion::E);
    assert_eq!(get_sdi_region(0.0, 0.7), SDIRegion::N);
    assert_eq!(get_sdi_region(-0.7, 0.0), SDIRegion::W);
    assert_eq!(get_sdi_region(0.0, -0.7), SDIRegion::S);
}

#[test]
fn test_sdi_from_legal_digital_file() {
    let data = read_slp_file("legal/digital/potion_p3/potion_2.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 2).unwrap();

    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0,
        "potion_2.slp should pass SDI rule 1"
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0,
        "potion_2.slp should pass SDI rule 3"
    );
}

#[test]
fn test_is_diagonal_adjacent() {
    assert_eq!(is_diagonal_adjacent(SDIRegion::SE, SDIRegion::NW), false);
    assert_eq!(is_diagonal_adjacent(SDIRegion::SE, SDIRegion::SE), false);
    assert_eq!(is_diagonal_adjacent(SDIRegion::SE, SDIRegion::DZ), false);
    assert_eq!(is_diagonal_adjacent(SDIRegion::SE, SDIRegion::SW), true);
    assert_eq!(is_diagonal_adjacent(SDIRegion::NW, SDIRegion::NE), true);
    assert_eq!(is_diagonal_adjacent(SDIRegion::SW, SDIRegion::SE), true);
}

#[test]
fn test_sdi_manual_coords() {
    // Sanity check. No movement. No violation
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
    ];
    assert_eq!(sdi::fails_sdi_rule_one(&coords).len(), 0);

    // Easy case. Lots of SDI. Violation.
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
    ];
    assert!(
        sdi::fails_sdi_rule_one(&coords).len() >= 1,
        "Rapid SDI should trigger violation"
    );

    // Too many frames in the tilt zone, doesn't count as SDI!
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.3, y: 0.0 },
        Coord { x: 0.32, y: 0.0 },
        Coord { x: 0.35, y: 0.0 },
        Coord { x: 0.4, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
    ];
    assert_eq!(
        sdi::fails_sdi_rule_one(&coords).len(),
        0,
        "Too many tilt frames should not trigger"
    );

    // Slowest possible SDIs. Doesn't count as SDI since it has travel time
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.3, y: 0.0 },
        Coord { x: 0.35, y: 0.0 },
        Coord { x: 0.4, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.35, y: 0.0 },
        Coord { x: 0.4, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
    ];
    assert_eq!(
        sdi::fails_sdi_rule_one(&coords).len(),
        0,
        "Travel time should exempt from violation"
    );

    // Doesn't count as SDI since it has travel time
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.3, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 0.3, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
    ];
    assert_eq!(
        sdi::fails_sdi_rule_one(&coords).len(),
        0,
        "Travel time should exempt from violation"
    );

    // Violation. SDI, then put in a bunch of travel time after.
    // IE: You don't get off the hook just because you have travel time after the SDI
    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.3, y: 0.0 },
        Coord { x: 0.35, y: 0.0 },
        Coord { x: 0.4, y: 0.0 },
    ];
    assert!(
        sdi::fails_sdi_rule_one(&coords).len() >= 1,
        "Travel time after SDI should not exempt"
    );
}

#[test]
fn test_sdi_legal_a() {
    let data = read_slp_file("legal/digital/sdi/sdi_r18_v2.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0,
        "sdi_r18_v2.slp should pass rule 1"
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0,
        "sdi_r18_v2.slp should pass rule 3"
    );
}

#[test]
fn test_sdi_legal_b() {
    let data = read_slp_file("legal/digital/sdi/sdi_t20_cardinal_diagonal.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = sdi::check(&player_data.main_coords);
    assert_eq!(
        result.result, false,
        "sdi_t20_cardinal_diagonal.slp should pass all SDI checks"
    );
    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_two(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0
    );
}

#[test]
fn test_sdi_legal_c() {
    let data = read_slp_file("legal/digital/sdi/sdi_mash.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    let result = sdi::check(&player_data.main_coords);
    assert_eq!(result.result, false, "sdi_mash.slp should pass all SDI checks");
    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_two(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0
    );
}

#[test]
fn test_sdi_legal_d() {
    let data = read_slp_file("legal/digital/sdi/Game_20250201T233229.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_two(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0
    );
}

#[test]
fn test_sdi_legal_e() {
    let data = read_slp_file("legal/digital/sdi/Game_20250201T232732.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_two(&player_data.main_coords).len(),
        0
    );
    assert_eq!(
        sdi::fails_sdi_rule_three(&player_data.main_coords).len(),
        0
    );
}

#[test]
fn test_sdi_nonlegal_a() {
    let data = read_slp_file("nonlegal/digital/sdi/sdi_tas_neutral_cardinal.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 3).unwrap();

    // Fails rule #1
    let result = sdi::check(&player_data.main_coords);
    assert_eq!(
        result.result, true,
        "sdi_tas_neutral_cardinal.slp should fail SDI check"
    );

    let violations = sdi::fails_sdi_rule_one(&player_data.main_coords);
    assert_eq!(violations.len(), 195, "Should have exactly 195 violations");
    assert_eq!(violations[10].reason, "Failed SDI rule #1");
    assert_eq!(violations[10].metric as i32, 139);
    assert_eq!(
        violations[10].evidence,
        vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
            Coord { x: -1.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
            Coord { x: -1.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
            Coord { x: -1.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 },
            Coord { x: -1.0, y: 0.0 },
            Coord { x: 0.0, y: 0.0 }
        ]
    );
}

#[test]
fn test_sdi_nonlegal_b() {
    let data = read_slp_file("nonlegal/digital/sdi/sdi_tas_cardinal_diagonal.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 3).unwrap();

    let violations = sdi::fails_sdi_rule_two(&player_data.main_coords);
    assert_eq!(violations.len(), 36, "Should have exactly 36 violations");
    assert_eq!(violations[10].reason, "Failed SDI rule #2");
    assert_eq!(violations[10].metric as i32, 226);
    assert_eq!(
        violations[10].evidence,
        vec![
            Coord { x: 1.0, y: 0.0 },
            Coord { x: 0.7, y: 0.7 },
            Coord { x: 1.0, y: 0.0 },
            Coord { x: 0.7, y: 0.7 },
            Coord { x: 1.0, y: 0.0 }
        ]
    );
}

#[test]
fn test_sdi_nonlegal_c() {
    let data = read_slp_file("nonlegal/digital/sdi/sdi_unnerfed.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 3).unwrap();

    // Fails rule #3 but not rule #1
    let result = sdi::check(&player_data.main_coords);
    assert_eq!(
        result.result, true,
        "sdi_unnerfed.slp should fail SDI check"
    );
    assert_eq!(
        sdi::fails_sdi_rule_one(&player_data.main_coords).len(),
        0,
        "Should pass rule 1"
    );

    let violations = sdi::fails_sdi_rule_three(&player_data.main_coords);
    assert_eq!(violations.len(), 8, "Should have exactly 8 violations");
    assert_eq!(violations[4].reason, "Failed SDI rule #3");
    assert_eq!(violations[4].metric as i32, 2535);
    assert_eq!(
        violations[4].evidence,
        vec![
            Coord { x: 0.7, y: 0.7 },
            Coord { x: 0.7, y: 0.7 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: -0.7, y: 0.7 },
            Coord { x: 0.7, y: 0.7 }
        ]
    );
}
