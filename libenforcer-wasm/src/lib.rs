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
use peppi::game::Game;
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

/// Helper to deserialize a Coord array from JsValue
fn coords_from_js(val: JsValue) -> Result<Vec<types::Coord>, JsValue> {
    serde_wasm_bindgen::from_value(val)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize coordinates: {}", e)))
}

/// Helper to deserialize a single Coord from JsValue
fn coord_from_js(val: JsValue) -> Result<types::Coord, JsValue> {
    serde_wasm_bindgen::from_value(val)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize coordinate: {}", e)))
}

/// Check if SLP version is less than 3.15.0
/// Mirrors TypeScript isSlpMinVersion() from index.ts
#[wasm_bindgen]
pub fn is_slp_min_version(slp_bytes: &[u8]) -> Result<bool, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    Ok(game.start().slippi.version.lt(3, 15))
}

/// Detect if player is using a box controller (from SLP bytes)
#[wasm_bindgen]
pub fn is_box_controller(slp_bytes: &[u8], player_index: usize) -> Result<bool, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    Ok(utils::is_box_controller(&player_data.main_coords))
}

/// Detect if coordinates match a box controller (from coord array)
#[wasm_bindgen]
pub fn is_box_controller_from_coords(coords: JsValue) -> Result<bool, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(utils::is_box_controller(&coords))
}

/// Extract joystick coordinates from an SLP file for a given player
/// Mirrors TypeScript getCoordListFromGame() from index.ts
#[wasm_bindgen]
pub fn get_coord_list_from_game(slp_bytes: &[u8], player_index: usize, is_main_stick: bool) -> Result<JsValue, JsValue> {
    let game = read_slippi(&mut Cursor::new(slp_bytes), None)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let player_data = parser::extract_player_data(&game, player_index)
        .ok_or_else(|| JsValue::from_str("Player not found"))?;

    let coords = if is_main_stick {
        player_data.main_coords
    } else {
        player_data.c_coords
    };

    serde_wasm_bindgen::to_value(&coords)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get C-stick violations from a coordinate array
/// Mirrors TypeScript getCStickViolations() from disallowed_analog_values.ts
#[wasm_bindgen]
pub fn get_cstick_violations(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let result = checks::disallowed_analog::get_cstick_violations(&coords);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Calculate the average travel coordinate hit rate
/// Mirrors TypeScript averageTravelCoordHitRate() from travel_time.ts
#[wasm_bindgen]
pub fn average_travel_coord_hit_rate(coords: JsValue) -> Result<f64, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(checks::travel_time::average_travel_coord_hit_rate(&coords))
}

/// Check if coordinates show GoomWave clamping
/// Mirrors TypeScript hasGoomwaveClamping() from goomwave.ts
#[wasm_bindgen]
pub fn has_goomwave_clamping(coords: JsValue) -> Result<bool, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(checks::goomwave::has_goomwave_clamping(&coords))
}

/// Classify joystick position into one of 9 regions
/// Mirrors TypeScript getJoystickRegion() from index.ts
/// Returns: 0=DZ, 1=NE, 2=SE, 3=SW, 4=NW, 5=N, 6=E, 7=S, 8=W
#[wasm_bindgen]
pub fn get_joystick_region(x: f64, y: f64) -> u8 {
    utils::get_joystick_region(x, y) as u8
}

/// Process raw analog stick values into normalized coordinates
/// Mirrors TypeScript processAnalogStick() from index.ts
#[wasm_bindgen]
pub fn process_analog_stick(x: f64, y: f64, deadzone: bool) -> Result<JsValue, JsValue> {
    let coord = parser::process_analog_stick(x as f32, y as f32, deadzone);

    serde_wasm_bindgen::to_value(&coord)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Float equality comparison with epsilon tolerance (0.0001)
/// Mirrors TypeScript FloatEquals() from index.ts
#[wasm_bindgen]
pub fn float_equals(a: f64, b: f64) -> bool {
    utils::float_equals(a, b)
}

/// Check if two coordinates are equal within float tolerance
/// Mirrors TypeScript isEqual() from index.ts
#[wasm_bindgen]
pub fn is_equal(one: JsValue, other: JsValue) -> Result<bool, JsValue> {
    let one = coord_from_js(one)?;
    let other = coord_from_js(other)?;
    Ok(utils::is_equal_coord(&one, &other))
}

/// Get unique coordinates from a list (deduplicated by value)
/// Mirrors TypeScript getUniqueCoords() from index.ts
#[wasm_bindgen]
pub fn get_unique_coords(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let unique = utils::get_unique_coords(&coords);

    serde_wasm_bindgen::to_value(&unique)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get target coordinates (coords held for 2+ consecutive frames)
/// Mirrors TypeScript getTargetCoords() from index.ts
#[wasm_bindgen]
pub fn get_target_coords(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let targets = utils::get_target_coords(&coords);

    serde_wasm_bindgen::to_value(&targets)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
