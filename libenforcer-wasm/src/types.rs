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

/// Per-axis fuzz delta distribution counts: [n_minus, n_zero, n_plus]
pub type DeltaCounts = [usize; 3];

/// Detailed statistical analysis of input fuzzing compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzAnalysis {
    /// Overall pass/fail verdict
    pub pass: bool,
    /// Normalized per-event log-likelihood ratio (positive = evidence of proper fuzzing)
    pub llr_score: f64,
    /// Chi-squared p-value for X-axis deltas (None if insufficient data or axis exempt)
    pub p_value_x: Option<f64>,
    /// Chi-squared p-value for Y-axis deltas (None if insufficient data or axis exempt)
    pub p_value_y: Option<f64>,
    /// Total number of fuzz events analyzed
    pub total_fuzz_events: usize,
    /// Observed delta distribution for X-axis: [n_minus, n_zero, n_plus]
    pub observed_x: DeltaCounts,
    /// Observed delta distribution for Y-axis: [n_minus, n_zero, n_plus]
    pub observed_y: DeltaCounts,
    /// Detailed violation descriptions
    pub violations: Vec<Violation>,
}

/// Controller type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControllerType {
    Box,
    Analog,
}

/// Full analysis results for a single player.
/// Checks that don't apply to the detected controller type are None.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnalysis {
    pub controller_type: ControllerType,
    /// Aggregate verdict: true if all applicable checks pass
    pub is_legal: bool,

    // Box controller checks (None if analog)
    pub travel_time: Option<CheckResult>,
    pub disallowed_cstick: Option<CheckResult>,
    pub crouch_uptilt: Option<CheckResult>,
    pub sdi: Option<CheckResult>,
    pub input_fuzzing: Option<FuzzAnalysis>,

    // Analog controller checks (None if box)
    pub goomwave: Option<CheckResult>,
    pub uptilt_rounding: Option<CheckResult>,
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
