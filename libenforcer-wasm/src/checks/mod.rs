pub mod travel_time;
pub mod disallowed_analog;
pub mod uptilt_rounding;
pub mod crouch_uptilt;
pub mod sdi;
pub mod goomwave;

use crate::parser::PlayerGameData;
use crate::types::AllCheckResults;

/// Run all checks on player data and return results
pub fn run_all(data: &PlayerGameData) -> AllCheckResults {
    AllCheckResults {
        travel_time: travel_time::check(&data.main_coords),
        disallowed_cstick: disallowed_analog::check(&data.main_coords, &data.c_coords),
        uptilt_rounding: uptilt_rounding::check(&data.main_coords),
        crouch_uptilt: crouch_uptilt::check(&data.main_coords, &data.action_states),
        sdi: sdi::check(&data.main_coords),
        goomwave: goomwave::check(&data.main_coords),
    }
}
