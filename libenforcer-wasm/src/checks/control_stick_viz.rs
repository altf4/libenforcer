use crate::types::{CheckResult, Coord, Violation};

/// Control stick visualization "check" — packages all coordinates as evidence
/// for visualization purposes. Always returns result: false (no violation).
pub fn check(coords: &[Coord]) -> CheckResult {
    let violation = Violation::with_evidence(
        0.0,
        "Control Stick Viz".to_string(),
        coords.to_vec(),
    );
    CheckResult {
        result: false,
        violations: vec![violation],
    }
}
