use crate::types::Coord;
use peppi::game::Game;

/// Extracted game data for a single player
pub struct PlayerGameData {
    pub main_coords: Vec<Coord>,       // Main joystick coordinates
    pub c_coords: Vec<Coord>,          // C-stick coordinates
    pub action_states: Vec<u16>,       // Action state IDs per frame
    pub raw_joystick_coords: Vec<Coord>, // Raw joystick values for uptilt check
}

/// Extract all relevant data for analysis from a Peppi Game
pub fn extract_player_data(game: &impl Game, player_index: usize) -> Option<PlayerGameData> {
    let mut main_coords = Vec::new();
    let mut c_coords = Vec::new();
    let mut action_states = Vec::new();
    let mut raw_joystick_coords = Vec::new();

    // Iterate through all frames using the Game trait
    for i in 0..game.len() {
        let frame = game.frame(i);

        // Find the port data for this player
        let port_data = match frame.ports.iter().find(|p| p.port as usize == player_index) {
            Some(port) => port,
            None => continue, // Player doesn't exist in this frame
        };

        // Extract pre-frame data (inputs before processing)
        let pre = &port_data.leader.pre;
        let post = &port_data.leader.post;

        // Main stick - use raw analog values (int8, like slippi-js rawJoystickX/Y)
        // when available, otherwise fall back to peppi's already-normalized joystick
        // (raw_analog_x added in Slippi v1.2, raw_analog_y in v3.15)
        let (processed_main, raw_x_f64, raw_y_f64) =
            if let (Some(rx), Some(ry)) = (pre.raw_analog_x, pre.raw_analog_y) {
                let rx_f32 = rx as f32;
                let ry_f32 = ry as f32;
                (process_analog_stick(rx_f32, ry_f32, false), rx_f32 as f64, ry_f32 as f64)
            } else {
                // Fallback: peppi's joystick values are already engine-normalized (-1..1)
                let coord = Coord { x: pre.joystick.x as f64, y: pre.joystick.y as f64 };
                // Raw values unavailable, approximate from normalized
                (coord, pre.joystick.x as f64 * 80.0, pre.joystick.y as f64 * 80.0)
            };
        main_coords.push(processed_main);

        // Raw joystick for uptilt check
        raw_joystick_coords.push(Coord {
            x: raw_x_f64,
            y: raw_y_f64,
        });

        // C-stick - already normalized in Peppi
        c_coords.push(Coord {
            x: pre.cstick.x as f64,
            y: pre.cstick.y as f64,
        });

        // Extract post-frame data (game state after processing)
        action_states.push(post.state);
    }

    if main_coords.is_empty() {
        return None;
    }

    Some(PlayerGameData {
        main_coords,
        c_coords,
        action_states,
        raw_joystick_coords,
    })
}

/// Process analog stick values to match the TypeScript implementation
/// This mirrors the processAnalogStick() function from index.ts
pub fn process_analog_stick(x: f32, y: f32, deadzone: bool) -> Coord {
    let magnitude_squared = (x * x) + (y * y);

    if magnitude_squared < 1e-3 {
        return Coord { x: 0.0, y: 0.0 };
    }

    let magnitude = magnitude_squared.sqrt();
    let threshold = 80.0;

    let mut fx = x;
    let mut fy = y;

    // Cap magnitude at threshold
    if magnitude > threshold {
        let shrink_factor = threshold / magnitude;
        if fx > 0.0 {
            fx = (fx * shrink_factor).floor();
            fy = (fy * shrink_factor).floor();
        } else {
            fx = (fx * shrink_factor).ceil();
            fy = (fy * shrink_factor).ceil();
        }
    }

    // Apply deadzone if requested
    if deadzone {
        if fx.abs() < 23.0 {
            fx = 0.0;
        }
        if fy.abs() < 23.0 {
            fy = 0.0;
        }
    }

    // Round to nearest integer
    fx = fx.round();
    fy = fy.round();

    // Normalize to -1.0..1.0 range
    Coord {
        x: (fx / 80.0) as f64,
        y: (fy / 80.0) as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_analog_stick_zero() {
        let coord = process_analog_stick(0.0, 0.0, false);
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
    }

    #[test]
    fn test_process_analog_stick_normalization() {
        let coord = process_analog_stick(80.0, 0.0, false);
        assert!((coord.x - 1.0).abs() < 0.01);
        assert_eq!(coord.y, 0.0);
    }

    #[test]
    fn test_process_analog_stick_magnitude_cap() {
        // Input exceeds threshold, should be capped
        let coord = process_analog_stick(100.0, 0.0, false);
        assert!((coord.x - 1.0).abs() < 0.01);
    }
}
