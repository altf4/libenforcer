use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A 2D coordinate representing joystick position
/// Values are normalized from -1.0 to 1.0
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Coord {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Coord {
        Coord { x, y }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        crate::utils::is_equal_coord(self, other)
    }
}

/// Represents a single violation of a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub metric: f64,           // Usually a frame number
    pub reason: String,
    pub evidence: Vec<Coord>,  // Optional coordinate evidence
}

impl Violation {
    pub fn new(metric: f64, reason: String) -> Self {
        Violation {
            metric,
            reason,
            evidence: vec![],
        }
    }

    pub fn with_evidence(metric: f64, reason: String, evidence: Vec<Coord>) -> Self {
        Violation {
            metric,
            reason,
            evidence,
        }
    }
}

/// Result of running a check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub result: bool,           // true = violation detected
    pub violations: Vec<Violation>,
}

impl CheckResult {
    pub fn pass() -> Self {
        CheckResult {
            result: false,
            violations: vec![],
        }
    }

    pub fn fail(violations: Vec<Violation>) -> Self {
        CheckResult {
            result: true,
            violations,
        }
    }

    pub fn fail_single(metric: f64, reason: String) -> Self {
        CheckResult {
            result: true,
            violations: vec![Violation::new(metric, reason)],
        }
    }
}

/// All check results for a single player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllCheckResults {
    pub travel_time: CheckResult,
    pub disallowed_cstick: CheckResult,
    pub uptilt_rounding: CheckResult,
    pub crouch_uptilt: CheckResult,
    pub sdi: CheckResult,
    pub goomwave: CheckResult,
    pub control_stick_viz: CheckResult,
    pub input_fuzzing: CheckResult,
}

/// Joystick region classification (9 regions based on 0.2875 threshold)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoystickRegion {
    DZ = 0,
    NE = 1,
    SE = 2,
    SW = 3,
    NW = 4,
    N = 5,
    E = 6,
    S = 7,
    W = 8,
}

/// Metadata about an available check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInfo {
    pub name: String,
}

/// Returns the list of all available checks.
/// Mirrors TypeScript ListChecks() from index.ts.
pub fn list_checks() -> Vec<CheckInfo> {
    vec![
        CheckInfo { name: "Box Travel Time".to_string() },
        CheckInfo { name: "Disallowed Analog C-Stick Values".to_string() },
        CheckInfo { name: "Uptilt Rounding".to_string() },
        CheckInfo { name: "Fast Crouch Uptilt".to_string() },
        CheckInfo { name: "Illegal SDI".to_string() },
        CheckInfo { name: "GoomWave Clamping".to_string() },
        CheckInfo { name: "Control Stick Visualization".to_string() },
        CheckInfo { name: "Input Fuzzing".to_string() },
    ]
}
