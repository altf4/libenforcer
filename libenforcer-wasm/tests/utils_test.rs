//! Integration tests for utility functions
//! Mirrors TypeScript tests in src/tests/utils.test.ts
//! Test count: 16

#[path = "common/mod.rs"]
mod common;

use common::*;
use libenforcer_wasm::{parser, types, types::Coord, utils};
use peppi::game::Game;
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_float_equals_allow_wiggle_room() {
    assert_eq!(utils::float_equals(0.8, 0.8), true);
    assert_eq!(utils::float_equals(0.8, 0.7999), true);
    assert_eq!(utils::float_equals(-0.7, -0.7000000000001), true);
    assert_eq!(utils::float_equals(0.8, -0.8), false);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_get_target_coords() {
    let coords: Vec<Coord> = vec![];
    assert_eq!(utils::get_target_coords(&coords).len(), 0);

    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: -1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: -1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.5, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
    ];

    let targets = utils::get_target_coords(&coords);
    assert_eq!(targets.len(), 2);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_get_unique_coords() {
    let coords: Vec<Coord> = vec![];
    assert_eq!(utils::get_unique_coords(&coords).len(), 0);

    let coords = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: -1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: -1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 1.0, y: 0.0 },
        Coord { x: 0.5, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
        Coord { x: 1.0, y: 0.5 },
    ];

    let unique = utils::get_unique_coords(&coords);
    assert_eq!(unique.len(), 5);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_pikachu_game() {
    let data = read_slp_file("legal/digital/should_count_as_box/Game_20250201T124602.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        true,
        "Pikachu game should be detected as box controller"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs() {
    // Games 1-5: Confirmed box players (techno_p1 dataset)
    for i in 1..=5 {
        let path = format!(
            "legal/digital/techno_p1/Steech_vs_techno_G{}.slp",
            i
        );
        let data = read_slp_file(&path);
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 0).unwrap();

        assert_eq!(
            utils::is_box_controller(&player_data.main_coords),
            true,
            "Game {} should be detected as box controller",
            i
        );
    }

    // Confirmed GCC player
    let data = read_slp_file("legal/analog/traveltime/Game_8C56C529AEAA_20231022T181554.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 3).unwrap();

    assert!(
        player_data.main_coords.len() > 0,
        "Should have coordinate data"
    );
    let unique = utils::get_unique_coords(&player_data.main_coords);
    assert!(unique.len() > 13, "GCC should have many unique coordinates");
    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "GCC player should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_potion_dataset() {
    for i in 1..=7 {
        let path = format!("legal/digital/potion_p3/potion_{}.slp", i);
        let data = read_slp_file(&path);
        let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
        let player_data = parser::extract_player_data(&game, 2).unwrap();

        assert_eq!(
            utils::is_box_controller(&player_data.main_coords),
            true,
            "Potion {} should be detected as box controller",
            i
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_orca_dataset_a() {
    let data = read_slp_file("legal/analog/orca/Game_20241228T175942.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Orca dataset A should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_orca_dataset_b() {
    let data = read_slp_file("legal/analog/orca/Game_20241228T180350.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Orca dataset B should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_orca_dataset_c() {
    let data = read_slp_file("legal/analog/orca/Game_20241228T180707.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Orca dataset C should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_orca_dataset_d() {
    let data = read_slp_file("legal/analog/orca/Game_20241228T180831.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Orca dataset D should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_orca_dataset_e() {
    let data = read_slp_file("legal/analog/orca/Game_20241228T181133.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 0).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Orca dataset E should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_xbox_controller_a() {
    let data = read_slp_file("legal/analog/xbox_p2/Game_20250209T181347.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Xbox controller A should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_is_box_inputs_xbox_controller_b() {
    let data = read_slp_file("legal/analog/xbox_p2/Game_20250209T183921.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    let player_data = parser::extract_player_data(&game, 1).unwrap();

    assert_eq!(
        utils::is_box_controller(&player_data.main_coords),
        false,
        "Xbox controller B should not be detected as box"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_parse_replay_file_correctly() {
    let data = read_slp_file("banned_c_stick_analog_player_1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();

    assert!(game.len() > 0, "Game should have frames");

    // Verify frame-by-frame that frame IDs are sequential starting at -123
    // Mirrors TS: for (let frame = -123; frame < 1111; frame++)
    //     expect(frames[frame].players[0]?.pre.frame).toEqual(frame)
    for i in 0..game.len() {
        let frame = game.frame(i);
        let expected_id = peppi::frame::FIRST_INDEX + i as i32;
        assert_eq!(
            frame.id, expected_id,
            "Frame at index {} should have id {}, got {}",
            i, expected_id, frame.id
        );
    }

    // Verify character ID at frame 500 (0-based index = 500 - (-123) = 623)
    // Mirrors TS: expect(frames[500].players[0]?.post.internalCharacterId).toEqual(0x0A)
    let frame_500_idx = (500 - peppi::frame::FIRST_INDEX) as usize;
    let frame_500 = game.frame(frame_500_idx);
    let port_data = frame_500
        .ports
        .iter()
        .find(|p| p.port as usize == 0)
        .expect("Player 0 should exist at frame 500");
    assert_eq!(
        port_data.leader.post.character, 0x0A,
        "Player 0 at frame 500 should have internalCharacterId 0x0A"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_process_main_stick_inputs() {
    // Test basic normalization
    let coord = parser::process_analog_stick(0.0, 0.0, false);
    assert!(utils::is_equal_coord(&coord, &Coord { x: 0.0, y: 0.0 }));

    let coord = parser::process_analog_stick(0.0, -80.0, false);
    assert!(utils::is_equal_coord(&coord, &Coord { x: 0.0, y: -1.0 }));

    // Verify process_analog_stick matches the engine's processed joystick values
    // frame-by-frame on a real SLP file.
    // Mirrors TS: for (let frame = -123; frame < 1794; frame++) {
    //     let rawCoord = processAnalogStick({x: rawJoystickX, y: rawJoystickY}, true)
    //     expect(isEqual(processedCoord, rawCoord)).toEqual(true)
    // }
    let data = read_slp_file("legal/digital/techno_p1/Steech_vs_techno_G1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();

    for i in 0..game.len() {
        let frame = game.frame(i);
        // Match TS range: for (let frame = -123; frame < 1794; frame++)
        if frame.id >= 1794 {
            break;
        }

        let port_data = match frame.ports.iter().find(|p| p.port as usize == 0) {
            Some(port) => port,
            None => continue,
        };

        let pre = &port_data.leader.pre;
        if let (Some(raw_x), Some(raw_y)) = (pre.raw_analog_x, pre.raw_analog_y) {
            let raw_coord = parser::process_analog_stick(raw_x as f32, raw_y as f32, true);
            let processed_coord = Coord {
                x: pre.joystick.x as f64,
                y: pre.joystick.y as f64,
            };
            assert!(
                utils::is_equal_coord(&raw_coord, &processed_coord),
                "Frame {}: process_analog_stick({}, {}, true) = ({}, {}), expected ({}, {})",
                frame.id,
                raw_x,
                raw_y,
                raw_coord.x,
                raw_coord.y,
                processed_coord.x,
                processed_coord.y
            );
        }
    }
}
