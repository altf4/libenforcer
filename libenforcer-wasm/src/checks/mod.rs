pub mod travel_time;
pub mod disallowed_analog;
pub mod uptilt_rounding;
pub mod crouch_uptilt;
pub mod sdi;
pub mod goomwave;
pub mod input_fuzzing;

use crate::parser::PlayerGameData;
use crate::types::{ControllerType, PlayerAnalysis};
use crate::utils;

/// Analyze a player's inputs: detect controller type, run applicable checks.
/// `is_box_controller` is computed once here and not repeated in each check.
pub fn analyze_player(data: &PlayerGameData) -> PlayerAnalysis {
    let is_box = utils::is_box_controller(&data.main_coords);

    if is_box {
        let travel_time = travel_time::check(&data.main_coords);
        let disallowed_cstick = disallowed_analog::check(&data.c_coords);
        let crouch_uptilt = crouch_uptilt::check(&data.main_coords, &data.action_states);
        let sdi = sdi::check(&data.main_coords);
        let input_fuzzing = input_fuzzing::analyze(&data.main_coords);

        let is_legal = !travel_time.result
            && !disallowed_cstick.result
            && !crouch_uptilt.result
            && !sdi.result
            && input_fuzzing.pass;

        PlayerAnalysis {
            controller_type: ControllerType::Box,
            is_legal,
            travel_time: Some(travel_time),
            disallowed_cstick: Some(disallowed_cstick),
            crouch_uptilt: Some(crouch_uptilt),
            sdi: Some(sdi),
            input_fuzzing: Some(input_fuzzing),
            goomwave: None,
            uptilt_rounding: None,
        }
    } else {
        let goomwave = goomwave::check(&data.main_coords);
        let uptilt_rounding = uptilt_rounding::check(&data.main_coords);

        let is_legal = !goomwave.result && !uptilt_rounding.result;

        PlayerAnalysis {
            controller_type: ControllerType::Analog,
            is_legal,
            travel_time: None,
            disallowed_cstick: None,
            crouch_uptilt: None,
            sdi: None,
            input_fuzzing: None,
            goomwave: Some(goomwave),
            uptilt_rounding: Some(uptilt_rounding),
        }
    }
}
