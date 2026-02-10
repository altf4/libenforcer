// Expose modules publicly for all non-WASM builds (including tests)
// This allows integration tests to access internals while keeping them private for WASM
#[cfg(not(target_arch = "wasm32"))]
pub mod checks;
#[cfg(not(target_arch = "wasm32"))]
pub mod parser;
#[cfg(not(target_arch = "wasm32"))]
pub mod types;
#[cfg(not(target_arch = "wasm32"))]
pub mod utils;
#[cfg(not(target_arch = "wasm32"))]
pub mod game_timer;
#[cfg(not(target_arch = "wasm32"))]
pub mod handwarmer;

// Keep modules private for WASM builds
#[cfg(target_arch = "wasm32")]
mod checks;
#[cfg(target_arch = "wasm32")]
mod parser;
#[cfg(target_arch = "wasm32")]
pub mod types;  // types needs to be pub for WASM bindings
#[cfg(target_arch = "wasm32")]
mod utils;
#[cfg(target_arch = "wasm32")]
mod game_timer;
#[cfg(target_arch = "wasm32")]
mod handwarmer;

use wasm_bindgen::prelude::*;
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

#[wasm_bindgen(start)]
pub fn init() {
    // Set up better error messages in the browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Analyze a Slippi replay file and run all checks on a specific player
/// Returns JSON with all check results
#[wasm_bindgen]
pub fn analyze_replay(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    // Parse the .SLP file with Peppi
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse SLP file: {}", e)))?;

    // Extract player data
    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found in this game"))?;

    // Run all checks
    let results = checks::run_all(&player_data);

    // Convert to JS value
    serde_wasm_bindgen::to_value(&results)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for illegal travel time patterns
#[wasm_bindgen]
pub fn check_travel_time(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::travel_time::check(&player_data.main_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for disallowed C-stick values
#[wasm_bindgen]
pub fn check_disallowed_cstick(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::disallowed_analog::check(&player_data.main_coords, &player_data.c_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for uptilt rounding
#[wasm_bindgen]
pub fn check_uptilt_rounding(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::uptilt_rounding::check(&player_data.main_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for fast crouch-uptilt transitions
#[wasm_bindgen]
pub fn check_crouch_uptilt(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::crouch_uptilt::check(&player_data.main_coords, &player_data.action_states);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for illegal SDI patterns
#[wasm_bindgen]
pub fn check_sdi(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::sdi::check(&player_data.main_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check for GoomWave clamping
#[wasm_bindgen]
pub fn check_goomwave(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::goomwave::check(&player_data.main_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Control stick visualization — packages all coordinates as evidence
#[wasm_bindgen]
pub fn check_control_stick_viz(slp_bytes: &[u8], player_index: usize) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let result = checks::control_stick_viz::check(&player_data.main_coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Check if a game is a handwarmer (warmup/practice session)
#[wasm_bindgen]
pub fn check_handwarmer(slp_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let result = handwarmer::is_handwarmer(&game);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// List all available checks
#[wasm_bindgen]
pub fn list_checks() -> Result<JsValue, JsValue> {
    let checks = types::list_checks();

    serde_wasm_bindgen::to_value(&checks)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
