use peppi::game::Game;
use crate::parser::process_analog_stick;

/// Deadzone threshold for joystick region classification.
/// Matches the TypeScript getJoystickRegion() DZ check.
const DZ_THRESHOLD: f64 = 0.2875;

fn is_in_deadzone(x: f64, y: f64) -> bool {
    x.abs() < DZ_THRESHOLD && y.abs() < DZ_THRESHOLD
}

/// Detect whether a game is a handwarmer (warmup/practice session).
/// Returns true if any player triggers either:
///   - Game is less than 1 minute (< 3600 frames)
///   - Player stayed in the deadzone for 10+ consecutive seconds (> 600 frames) while alive
///
/// Mirrors TypeScript isHandwarmer() from index.ts.
pub fn is_handwarmer(game: &impl Game) -> bool {
    if game.len() == 0 {
        return true;
    }

    // Get active player ports from game start data.
    // Uses start().players (real players only), matching TS behavior where
    // game.getFrames()[0].players only includes actual characters.
    let active_ports: Vec<usize> = game.start()
        .players
        .iter()
        .map(|p| p.port as usize)
        .collect();

    // TS uses total game frames for the duration check (coords include
    // (0,0) entries for frames where the player is absent).
    let total_frames = game.len();

    for &port_index in &active_ports {
        if total_frames < 3600 {
            return true;
        }

        let mut deadzone_frames = 0usize;

        for i in 0..total_frames {
            let frame = game.frame(i);

            match frame.ports.iter().find(|p| p.port as usize == port_index) {
                Some(port) => {
                    // In slippi-js, eliminated players (0 stocks) are excluded
                    // from frame data, resetting the DZ counter. Peppi still
                    // includes them, so we check stocks explicitly.
                    if port.leader.post.stocks == 0 {
                        deadzone_frames = 0;
                        continue;
                    }

                    let pre = &port.leader.pre;

                    let coord = if let (Some(rx), Some(ry)) = (pre.raw_analog_x, pre.raw_analog_y) {
                        process_analog_stick(rx as f32, ry as f32, false)
                    } else {
                        crate::types::Coord {
                            x: pre.joystick.x as f64,
                            y: pre.joystick.y as f64,
                        }
                    };

                    if is_in_deadzone(coord.x, coord.y) {
                        deadzone_frames += 1;
                    } else {
                        deadzone_frames = 0;
                    }

                    if deadzone_frames > 600 {
                        return true;
                    }
                }
                None => {
                    // Player not present in this frame (dead/respawning)
                    deadzone_frames = 0;
                }
            }
        }
    }

    false
}
