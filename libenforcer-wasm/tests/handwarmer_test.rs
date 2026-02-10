//! Integration tests for handwarmer detection
//! Mirrors TypeScript tests in src/tests/utils.test.ts (Is handwarmer? A/B/C)
//! Test count: 3

#[path = "common/mod.rs"]
mod common;

#[cfg(not(target_arch = "wasm32"))]
use libenforcer_wasm::handwarmer;
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

use common::*;

#[test]
fn test_is_handwarmer_a_techno_not_handwarmers() {
    let files = read_slp_dir("legal/digital/techno_p1");
    assert!(!files.is_empty(), "Should find techno test files");

    for (filename, data) in &files {
        let game = read_slippi(&mut Cursor::new(data), None).unwrap();
        assert_eq!(
            handwarmer::is_handwarmer(&game),
            false,
            "{} should NOT be a handwarmer",
            filename
        );
    }
}

#[test]
fn test_is_handwarmer_b_handwarmer_files() {
    let files = read_slp_dir("handwarmers");
    assert!(!files.is_empty(), "Should find handwarmer test files");

    for (filename, data) in &files {
        let game = read_slippi(&mut Cursor::new(data), None).unwrap();
        assert_eq!(
            handwarmer::is_handwarmer(&game),
            true,
            "{} should be a handwarmer",
            filename
        );
    }
}

#[test]
fn test_is_handwarmer_c_doubles_not_handwarmer() {
    let data = read_slp_file("doubles_match_1.slp");
    let game = read_slippi(&mut Cursor::new(&data), None).unwrap();
    assert_eq!(
        handwarmer::is_handwarmer(&game),
        false,
        "doubles_match_1.slp should NOT be a handwarmer"
    );
}
