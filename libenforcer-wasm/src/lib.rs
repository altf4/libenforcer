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
use peppi::game::immutable::Game as ImmutableGame;
use peppi::io::slippi::de::read as read_slippi;
use std::io::Cursor;

#[derive(serde::Serialize)]
struct PlayerSettings {
    #[serde(rename = "playerIndex")]
    player_index: u8,
    #[serde(rename = "characterId")]
    character_id: u8,
    #[serde(rename = "playerType")]
    player_type: u8,
    #[serde(rename = "characterColor")]
    character_color: u8,
}

#[derive(serde::Serialize)]
struct GameSettings {
    #[serde(rename = "stageId")]
    stage_id: u16,
    players: Vec<PlayerSettings>,
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ---- Standalone utility functions (operate on raw data, not SLP files) ----

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

/// Detect if coordinates match a box controller (from coord array)
#[wasm_bindgen]
pub fn is_box_controller_from_coords(coords: JsValue) -> Result<bool, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(utils::is_box_controller(&coords))
}

/// Get C-stick violations from a coordinate array
#[wasm_bindgen]
pub fn get_cstick_violations(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let result = checks::disallowed_analog::get_cstick_violations(&coords);
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Calculate the average travel coordinate hit rate
#[wasm_bindgen]
pub fn average_travel_coord_hit_rate(coords: JsValue) -> Result<f64, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(checks::travel_time::average_travel_coord_hit_rate(&coords))
}

/// Check if coordinates show GoomWave clamping
#[wasm_bindgen]
pub fn has_goomwave_clamping(coords: JsValue) -> Result<bool, JsValue> {
    let coords = coords_from_js(coords)?;
    Ok(checks::goomwave::has_goomwave_clamping(&coords))
}

/// Classify joystick position into one of 9 regions
/// Returns: 0=DZ, 1=NE, 2=SE, 3=SW, 4=NW, 5=N, 6=E, 7=S, 8=W
#[wasm_bindgen]
pub fn get_joystick_region(x: f64, y: f64) -> u8 {
    utils::get_joystick_region(x, y) as u8
}

/// Process raw analog stick values into normalized coordinates
#[wasm_bindgen]
pub fn process_analog_stick(x: f64, y: f64, deadzone: bool) -> Result<JsValue, JsValue> {
    let coord = parser::process_analog_stick(x as f32, y as f32, deadzone);
    serde_wasm_bindgen::to_value(&coord)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Float equality comparison with epsilon tolerance (0.0001)
#[wasm_bindgen]
pub fn float_equals(a: f64, b: f64) -> bool {
    utils::float_equals(a, b)
}

/// Check if two coordinates are equal within float tolerance
#[wasm_bindgen]
pub fn is_equal(one: JsValue, other: JsValue) -> Result<bool, JsValue> {
    let one = coord_from_js(one)?;
    let other = coord_from_js(other)?;
    Ok(utils::is_equal_coord(&one, &other))
}

/// Get unique coordinates from a list (deduplicated by value)
#[wasm_bindgen]
pub fn get_unique_coords(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let unique = utils::get_unique_coords(&coords);
    serde_wasm_bindgen::to_value(&unique)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get target coordinates (coords held for 2+ consecutive frames)
#[wasm_bindgen]
pub fn get_target_coords(coords: JsValue) -> Result<JsValue, JsValue> {
    let coords = coords_from_js(coords)?;
    let targets = utils::get_target_coords(&coords);
    serde_wasm_bindgen::to_value(&targets)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

// ---- SlpGame: parse once, query many times ----

/// A parsed SLP game that can be queried without re-parsing.
/// Construct with `new SlpGame(slpBytes)`, then call methods on it.
/// Call `.free()` when done to release Wasm memory.
#[wasm_bindgen]
pub struct SlpGame {
    game: ImmutableGame,
}

#[wasm_bindgen]
impl SlpGame {
    /// Parse an SLP file. The parsed game is held in Wasm memory
    /// until `.free()` is called.
    #[wasm_bindgen(constructor)]
    pub fn new(slp_bytes: &[u8]) -> Result<SlpGame, JsValue> {
        let game = read_slippi(&mut Cursor::new(slp_bytes), None)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse SLP file: {}", e)))?;
        Ok(SlpGame { game })
    }

    /// Run all applicable checks on a player and return structured results.
    /// Controller type is detected once; only relevant checks are run.
    #[wasm_bindgen(js_name = "analyzePlayer")]
    pub fn analyze_player(&self, player_index: usize) -> Result<JsValue, JsValue> {
        let player_data = parser::extract_player_data(&self.game, player_index)
            .ok_or_else(|| JsValue::from_str("Player not found in this game"))?;
        let results = checks::analyze_player(&player_data);
        serde_wasm_bindgen::to_value(&results)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Extract main stick coordinates for a player
    #[wasm_bindgen(js_name = "getMainStickCoords")]
    pub fn get_main_stick_coords(&self, player_index: usize) -> Result<JsValue, JsValue> {
        let player_data = parser::extract_player_data(&self.game, player_index)
            .ok_or_else(|| JsValue::from_str("Player not found"))?;
        serde_wasm_bindgen::to_value(&player_data.main_coords)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Extract C-stick coordinates for a player
    #[wasm_bindgen(js_name = "getCStickCoords")]
    pub fn get_cstick_coords(&self, player_index: usize) -> Result<JsValue, JsValue> {
        let player_data = parser::extract_player_data(&self.game, player_index)
            .ok_or_else(|| JsValue::from_str("Player not found"))?;
        serde_wasm_bindgen::to_value(&player_data.c_coords)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Detect if a player is using a box controller
    #[wasm_bindgen(js_name = "isBoxController")]
    pub fn is_box_controller(&self, player_index: usize) -> Result<bool, JsValue> {
        let player_data = parser::extract_player_data(&self.game, player_index)
            .ok_or_else(|| JsValue::from_str("Player not found"))?;
        Ok(utils::is_box_controller(&player_data.main_coords))
    }

    /// Check if the game is a handwarmer
    #[wasm_bindgen(js_name = "isHandwarmer")]
    pub fn is_handwarmer(&self) -> bool {
        handwarmer::is_handwarmer(&self.game)
    }

    /// Extract game settings (stage, players)
    #[wasm_bindgen(js_name = "getGameSettings")]
    pub fn get_game_settings(&self) -> Result<JsValue, JsValue> {
        let start = self.game.start();
        let players = start.players.iter().map(|p| PlayerSettings {
            player_index: p.port as u8,
            character_id: p.character,
            player_type: p.r#type as u8,
            character_color: p.costume,
        }).collect();

        let settings = GameSettings {
            stage_id: start.stage,
            players,
        };

        serde_wasm_bindgen::to_value(&settings)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Check if SLP version is below 3.15.0
    #[wasm_bindgen(js_name = "isSlpMinVersion")]
    pub fn is_slp_min_version(&self) -> bool {
        self.game.start().slippi.version.lt(3, 15)
    }
}
