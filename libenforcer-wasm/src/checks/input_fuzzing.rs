use crate::types::{CheckResult, Coord, DeltaCounts, FuzzAnalysis, Violation};
use crate::utils::{float_equals, is_equal_coord};
use std::collections::HashMap;

/// One raw coordinate unit in normalized space (1/80)
const UNIT: f64 = 1.0 / 80.0;

/// Magnitude threshold above which a NonCardinal coordinate is considered "on the rim".
/// At raw magnitude ≥ 78, diagonal ±1 fuzz offsets can push past 80 and get absorbed
/// by the game's unit-circle clamping, making fuzzing invisible.
const RIM_MAGNITUDE_THRESHOLD: f64 = 0.975; // 78/80

// --- Log-likelihood ratio constants ---
// H_fuzz:   P(δ=0) = 0.50, P(δ=±1) = 0.25
// H_nofuzz: P(δ=0) = 0.95, P(δ=±1) = 0.025
const LLR_DELTA_ZERO: f64 = -0.6418538; // ln(0.50 / 0.95)
const LLR_DELTA_ONE: f64 = 2.3025851;   // ln(0.25 / 0.025)

/// Minimum fuzz events (after filtering uninformative singletons) before we use LLR for pass/fail
const MIN_EVENTS_FOR_LLR: usize = 8;

/// Minimum fuzz events before we consider chi-squared reliable
const MIN_EVENTS_FOR_CHI_SQ: usize = 20;

/// Classification of a coordinate for fuzzing purposes
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CoordClass {
    /// (±1.0, 0.0) or (0.0, ±1.0) — exempt from fuzzing
    Cardinal,
    /// (0.0, 0.0) — exempt from fuzzing
    Origin,
    /// One axis is 0.0, other is non-zero and not ±1.0 — 1D fuzzing required
    Deadzone,
    /// Both axes non-zero — 2D fuzzing required
    NonCardinal,
    /// Both axes non-zero, magnitude ≥ RIM_MAGNITUDE_THRESHOLD.
    /// Fuzz offsets may be absorbed by the game's unit-circle clamping.
    Rim,
}

/// Classify a coordinate for fuzzing requirements
fn classify_coord(coord: &Coord) -> CoordClass {
    let x_zero = float_equals(coord.x, 0.0);
    let y_zero = float_equals(coord.y, 0.0);
    let x_cardinal = float_equals(coord.x.abs(), 1.0);
    let y_cardinal = float_equals(coord.y.abs(), 1.0);

    if x_zero && y_zero {
        CoordClass::Origin
    } else if (x_cardinal && y_zero) || (y_cardinal && x_zero) {
        CoordClass::Cardinal
    } else if x_zero || y_zero {
        CoordClass::Deadzone
    } else {
        let magnitude = (coord.x * coord.x + coord.y * coord.y).sqrt();
        if magnitude >= RIM_MAGNITUDE_THRESHOLD {
            CoordClass::Rim
        } else {
            CoordClass::NonCardinal
        }
    }
}

/// A hold is a sequence of 2+ identical consecutive frames (one targeting event)
pub struct Hold {
    pub coord: Coord,
    pub start_frame: usize,
}

/// Identify all holds in the coordinate sequence
/// A hold = 2+ consecutive frames with the same coordinate
pub fn identify_holds(coords: &[Coord]) -> Vec<Hold> {
    let mut holds = Vec::new();
    if coords.is_empty() {
        return holds;
    }

    let mut i = 0;
    while i < coords.len() {
        let start = i;
        while i + 1 < coords.len() && is_equal_coord(&coords[i], &coords[i + 1]) {
            i += 1;
        }
        if i > start {
            holds.push(Hold {
                coord: coords[start],
                start_frame: start,
            });
        }
        i += 1;
    }

    holds
}

/// Key for hashing coordinates using integer units (multiples of 1/80)
fn coord_key(coord: &Coord) -> (i32, i32) {
    ((coord.x / UNIT).round() as i32, (coord.y / UNIT).round() as i32)
}

/// A fuzz event: one hold assigned to its inferred target, with computed deltas
struct FuzzEvent {
    /// Delta from target in X-axis (integer units: -1, 0, or +1)
    dx: i32,
    /// Delta from target in Y-axis (integer units: -1, 0, or +1)
    dy: i32,
    /// Whether X-axis requires fuzzing (non-zero target X)
    x_fuzzable: bool,
    /// Whether Y-axis requires fuzzing (non-zero target Y)
    y_fuzzable: bool,
    /// Integer key of the inferred target coordinate
    target_key: (i32, i32),
}

/// Get the neighbor offsets for a given coordinate classification.
fn neighbor_offsets_for(coord: &Coord, class: &CoordClass) -> Vec<(i32, i32)> {
    match class {
        CoordClass::Deadzone => {
            if float_equals(coord.y, 0.0) {
                vec![(-1, 0), (1, 0)]
            } else {
                vec![(0, -1), (0, 1)]
            }
        }
        CoordClass::NonCardinal => {
            vec![
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1),
            ]
        }
        _ => vec![],
    }
}

/// Cluster holds into targets and compute per-event fuzz deltas.
///
/// Algorithm:
/// 1. Group holds by integer coordinate key
/// 2. Identify targets: each key that is the most frequent in its fuzz neighborhood
/// 3. Detect contested keys: coordinates that fall in the fuzz zone of multiple targets
/// 4. Produce events only for unambiguous assignments (skip contested keys)
///
/// Returns only events for non-cardinal, non-origin coordinates (fuzzable targets).
fn cluster_and_compute_deltas(holds: &[Hold]) -> Vec<FuzzEvent> {
    // Group holds by integer key → count
    let mut key_counts: HashMap<(i32, i32), usize> = HashMap::new();
    for hold in holds {
        let key = coord_key(&hold.coord);
        *key_counts.entry(key).or_insert(0) += 1;
    }

    // --- Pass 1: Identify all candidate targets ---
    // A key is a target if no neighbor has a strictly higher count.
    let mut target_keys: Vec<(i32, i32)> = Vec::new();

    for &key in key_counts.keys() {
        let coord = Coord::new(key.0 as f64 * UNIT, key.1 as f64 * UNIT);
        let class = classify_coord(&coord);

        match class {
            CoordClass::Cardinal | CoordClass::Origin | CoordClass::Rim => continue,
            _ => {}
        }

        let my_count = key_counts[&key];
        let offsets = neighbor_offsets_for(&coord, &class);

        let is_target = offsets.iter().all(|&(ox, oy)| {
            let nkey = (key.0 + ox, key.1 + oy);
            key_counts.get(&nkey).map_or(true, |&nc| nc <= my_count)
        });

        if is_target {
            target_keys.push(key);
        }
    }

    // --- Pass 2: For each key, find which target(s) claim it ---
    // Build key → Vec<target_key> mapping to detect contested keys.
    let mut key_claimants: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for &tkey in &target_keys {
        let coord = Coord::new(tkey.0 as f64 * UNIT, tkey.1 as f64 * UNIT);
        let class = classify_coord(&coord);
        let offsets = neighbor_offsets_for(&coord, &class);

        // The target itself
        if key_counts.contains_key(&tkey) {
            key_claimants.entry(tkey).or_default().push(tkey);
        }
        // Its neighbors
        for &(ox, oy) in &offsets {
            let nkey = (tkey.0 + ox, tkey.1 + oy);
            if key_counts.contains_key(&nkey) {
                key_claimants.entry(nkey).or_default().push(tkey);
            }
        }
    }

    // --- Pass 2b: Compute cluster size per target ---
    // A target's cluster size = total holds across all unambiguous keys assigned to it.
    // Targets with cluster_size=1 have a single hold that trivially sits at delta=0,
    // providing no information about fuzzing presence. Exclude them.
    let mut target_cluster_size: HashMap<(i32, i32), usize> = HashMap::new();
    for (key, claimants) in &key_claimants {
        if claimants.len() == 1 {
            *target_cluster_size.entry(claimants[0]).or_insert(0) += key_counts[key];
        }
    }

    // --- Pass 3: Produce events only for unambiguous keys ---
    let mut events = Vec::new();
    for hold in holds {
        let key = coord_key(&hold.coord);
        let claimants = match key_claimants.get(&key) {
            Some(c) => c,
            None => continue, // not claimed by any target
        };

        // Skip contested keys (claimed by multiple targets)
        if claimants.len() != 1 {
            continue;
        }

        let target_key = claimants[0];

        // Skip events from targets with only 1 hold in their cluster —
        // a lone hold always has delta=0 and is uninformative.
        if target_cluster_size.get(&target_key).copied().unwrap_or(0) < 2 {
            continue;
        }
        let target_coord = Coord::new(
            target_key.0 as f64 * UNIT,
            target_key.1 as f64 * UNIT,
        );
        let class = classify_coord(&target_coord);

        let dx = key.0 - target_key.0;
        let dy = key.1 - target_key.1;

        // Sanity: only include deltas within expected fuzz range
        if dx.abs() > 1 || dy.abs() > 1 {
            continue;
        }

        let x_fuzzable = match class {
            CoordClass::Deadzone => !float_equals(target_coord.x, 0.0),
            CoordClass::NonCardinal => true,
            _ => false,
        };
        let y_fuzzable = match class {
            CoordClass::Deadzone => !float_equals(target_coord.y, 0.0),
            CoordClass::NonCardinal => true,
            _ => false,
        };

        events.push(FuzzEvent {
            dx,
            dy,
            x_fuzzable,
            y_fuzzable,
            target_key,
        });
    }

    events
}

/// Accumulate delta counts from fuzz events into per-axis distributions.
/// Returns (x_counts, y_counts) where each is [n_minus, n_zero, n_plus].
fn accumulate_deltas(events: &[FuzzEvent]) -> (DeltaCounts, DeltaCounts) {
    let mut x_counts: DeltaCounts = [0, 0, 0];
    let mut y_counts: DeltaCounts = [0, 0, 0];

    for event in events {
        if event.x_fuzzable {
            match event.dx {
                -1 => x_counts[0] += 1,
                0 => x_counts[1] += 1,
                1 => x_counts[2] += 1,
                _ => {} // out of range, skip
            }
        }
        if event.y_fuzzable {
            match event.dy {
                -1 => y_counts[0] += 1,
                0 => y_counts[1] += 1,
                1 => y_counts[2] += 1,
                _ => {}
            }
        }
    }

    (x_counts, y_counts)
}

/// Compute the normalized log-likelihood ratio score.
/// Positive = evidence of proper fuzzing, negative = evidence of no fuzzing.
pub fn compute_llr(x_counts: &DeltaCounts, y_counts: &DeltaCounts) -> f64 {
    let mut total_score = 0.0;
    let mut total_events = 0usize;

    for counts in [x_counts, y_counts] {
        let n: usize = counts.iter().sum();
        if n == 0 {
            continue;
        }
        // counts = [n_minus, n_zero, n_plus]
        // delta=0 events (index 1)
        total_score += counts[1] as f64 * LLR_DELTA_ZERO;
        // delta=±1 events (indices 0 and 2)
        total_score += (counts[0] + counts[2]) as f64 * LLR_DELTA_ONE;
        total_events += n;
    }

    if total_events == 0 {
        return 0.0;
    }

    total_score / total_events as f64
}

/// Compute chi-squared statistic and p-value for a single axis.
/// Tests observed counts against expected distribution {0.25, 0.50, 0.25}.
/// Returns None if total observations < MIN_EVENTS_FOR_CHI_SQ.
pub fn chi_squared_test(counts: &DeltaCounts) -> Option<f64> {
    let n: usize = counts.iter().sum();
    if n < MIN_EVENTS_FOR_CHI_SQ {
        return None;
    }

    let n_f = n as f64;
    // Expected: [0.25, 0.50, 0.25] * n
    let expected = [0.25 * n_f, 0.50 * n_f, 0.25 * n_f];
    let observed = [counts[0] as f64, counts[1] as f64, counts[2] as f64];

    let mut chi_sq = 0.0;
    for i in 0..3 {
        if expected[i] > 0.0 {
            let diff = observed[i] - expected[i];
            chi_sq += diff * diff / expected[i];
        }
    }

    // Convert chi-squared statistic (df=2) to p-value using survival function
    Some(chi_sq_survival_df2(chi_sq))
}

/// Survival function (1 - CDF) for chi-squared distribution with df=2.
/// For df=2, the survival function has a simple closed form: P(X > x) = exp(-x/2).
fn chi_sq_survival_df2(x: f64) -> f64 {
    if x <= 0.0 {
        1.0
    } else {
        (-x / 2.0).exp()
    }
}

/// Build human-readable violation entries from fuzz events.
/// Produces an overall summary violation plus per-target breakdowns for suspicious targets.
fn build_violations(events: &[FuzzEvent], llr_score: f64) -> Vec<Violation> {
    let mut violations = Vec::new();

    // Group events by target_key
    let mut per_target: HashMap<(i32, i32), Vec<&FuzzEvent>> = HashMap::new();
    for event in events {
        per_target.entry(event.target_key).or_default().push(event);
    }

    let total_events = events.len();

    // Compute odds ratio for the summary
    let total_llr = llr_score * total_events as f64;
    let log10_odds = (-total_llr) * std::f64::consts::LOG10_E;

    // Overall summary violation
    violations.push(Violation::new(
        llr_score,
        format!("1 in 10^{:.0} odds of occurring by chance", log10_odds),
    ));

    // Per-target breakdowns, sorted by per-target LLR (worst first)
    let mut target_stats: Vec<((i32, i32), Vec<&FuzzEvent>)> = per_target.into_iter().collect();
    target_stats.sort_by(|a, b| {
        let llr_a = per_target_llr(&a.1);
        let llr_b = per_target_llr(&b.1);
        llr_a.partial_cmp(&llr_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    for (tkey, tevents) in &target_stats {
        let target_coord = Coord::new(tkey.0 as f64 * UNIT, tkey.1 as f64 * UNIT);
        let n = tevents.len();
        if n == 0 {
            continue;
        }

        // Accumulate per-target delta counts
        let (tx, ty) = accumulate_deltas_from_refs(tevents);
        let tx_total: usize = tx.iter().sum();
        let ty_total: usize = ty.iter().sum();

        let mut reason = format!(
            "Target coordinate ({:.4}, {:.4}) — {} transition{}:",
            target_coord.x, target_coord.y, n, if n == 1 { "" } else { "s" },
        );

        if tx_total > 0 {
            reason.push_str(&format!(
                "\n  X-axis: {:.1}% unchanged, {:.1}% +1, {:.1}% -1 (expected ~50% / ~25% / ~25%)",
                pct(tx[1], tx_total), pct(tx[2], tx_total), pct(tx[0], tx_total),
            ));
        }
        if ty_total > 0 {
            reason.push_str(&format!(
                "\n  Y-axis: {:.1}% unchanged, {:.1}% +1, {:.1}% -1 (expected ~50% / ~25% / ~25%)",
                pct(ty[1], ty_total), pct(ty[2], ty_total), pct(ty[0], ty_total),
            ));
        }

        violations.push(Violation::with_evidence(
            per_target_llr(tevents),
            reason,
            vec![target_coord],
        ));
    }

    violations
}

/// Compute per-target LLR from a slice of event references.
fn per_target_llr(events: &[&FuzzEvent]) -> f64 {
    let mut x_counts: DeltaCounts = [0, 0, 0];
    let mut y_counts: DeltaCounts = [0, 0, 0];
    for event in events {
        if event.x_fuzzable {
            match event.dx {
                -1 => x_counts[0] += 1,
                0 => x_counts[1] += 1,
                1 => x_counts[2] += 1,
                _ => {}
            }
        }
        if event.y_fuzzable {
            match event.dy {
                -1 => y_counts[0] += 1,
                0 => y_counts[1] += 1,
                1 => y_counts[2] += 1,
                _ => {}
            }
        }
    }
    compute_llr(&x_counts, &y_counts)
}

/// Accumulate delta counts from a slice of event references (used for per-target stats).
fn accumulate_deltas_from_refs(events: &[&FuzzEvent]) -> (DeltaCounts, DeltaCounts) {
    let mut x_counts: DeltaCounts = [0, 0, 0];
    let mut y_counts: DeltaCounts = [0, 0, 0];
    for event in events {
        if event.x_fuzzable {
            match event.dx {
                -1 => x_counts[0] += 1,
                0 => x_counts[1] += 1,
                1 => x_counts[2] += 1,
                _ => {}
            }
        }
        if event.y_fuzzable {
            match event.dy {
                -1 => y_counts[0] += 1,
                0 => y_counts[1] += 1,
                1 => y_counts[2] += 1,
                _ => {}
            }
        }
    }
    (x_counts, y_counts)
}

/// Compute percentage (0.0-100.0) from a count and total.
fn pct(count: usize, total: usize) -> f64 {
    if total == 0 { 0.0 } else { count as f64 / total as f64 * 100.0 }
}

/// Perform full statistical analysis of input fuzzing compliance.
/// Returns a FuzzAnalysis with LLR score, chi-squared p-values, and delta distributions.
pub fn analyze(coords: &[Coord]) -> FuzzAnalysis {
    let holds = identify_holds(coords);
    let events = cluster_and_compute_deltas(&holds);
    let (x_counts, y_counts) = accumulate_deltas(&events);
    let total_fuzz_events = events.len();

    let llr_score = compute_llr(&x_counts, &y_counts);
    let p_value_x = chi_squared_test(&x_counts);
    let p_value_y = chi_squared_test(&y_counts);

    // Pass/fail decision: LLR is the sole decision maker.
    // Chi-squared p-values are informational (reported but don't auto-fail).
    // This avoids false positives on controllers with slight distribution biases.
    let (pass, violations) = if total_fuzz_events < MIN_EVENTS_FOR_LLR {
        // Insufficient data — default to pass (avoid false positives)
        (true, vec![])
    } else if llr_score < 0.0 {
        // LLR negative = more consistent with no-fuzzing hypothesis
        (false, build_violations(&events, llr_score))
    } else {
        // LLR positive = evidence of fuzzing present
        (true, vec![])
    };

    FuzzAnalysis {
        pass,
        llr_score,
        p_value_x,
        p_value_y,
        total_fuzz_events,
        observed_x: x_counts,
        observed_y: y_counts,
        violations,
    }
}

/// Check for missing input fuzzing on box controllers.
/// Returns a CheckResult for backward compatibility.
pub fn check(coords: &[Coord]) -> CheckResult {
    let analysis = analyze(coords);
    if analysis.pass {
        CheckResult::pass()
    } else {
        CheckResult::fail(analysis.violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_coordinate() {
        assert_eq!(classify_coord(&Coord::new(1.0, 0.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(-1.0, 0.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(0.0, 1.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(0.0, -1.0)), CoordClass::Cardinal);
        assert_eq!(classify_coord(&Coord::new(0.0, 0.0)), CoordClass::Origin);
        assert_eq!(classify_coord(&Coord::new(0.5, 0.0)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(0.0, 0.5)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(-0.3, 0.0)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(0.5, 0.5)), CoordClass::NonCardinal);
        assert_eq!(classify_coord(&Coord::new(-0.3, 0.4)), CoordClass::NonCardinal);
        // (0.7, 0.7) has magnitude ~0.9899 >= 0.975 → Rim
        assert_eq!(classify_coord(&Coord::new(0.7, 0.7)), CoordClass::Rim);
    }

    #[test]
    fn test_identify_holds() {
        let coords = vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.5, 0.0), // travel (1 frame)
            Coord::new(0.7, 0.0),
            Coord::new(0.7, 0.0), // hold at (0.7, 0.0)
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0), // hold at (0.0, 0.0)
        ];

        let holds = identify_holds(&coords);
        assert_eq!(holds.len(), 2);
        assert!(is_equal_coord(&holds[0].coord, &Coord::new(0.7, 0.0)));
        assert_eq!(holds[0].start_frame, 2);
        assert!(is_equal_coord(&holds[1].coord, &Coord::new(0.0, 0.0)));
        assert_eq!(holds[1].start_frame, 4);
    }

    /// Helper: create a coordinate sequence simulating repeated targeting events.
    /// Each event: neutral hold → travel frame → output hold
    fn make_targeting_sequence(outputs: &[Coord]) -> Vec<Coord> {
        let mut coords = Vec::new();
        for &output in outputs {
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.0, 0.0));
            coords.push(Coord::new(0.3, 0.3)); // travel
            coords.push(output);
            coords.push(output);
        }
        coords.push(Coord::new(0.0, 0.0));
        coords.push(Coord::new(0.0, 0.0));
        coords
    }

    #[test]
    fn test_llr_positive_for_proper_fuzzing() {
        // Perfect 50/25/25 distribution
        let counts: DeltaCounts = [25, 50, 25]; // -1, 0, +1
        let llr = compute_llr(&counts, &[0, 0, 0]);
        assert!(llr > 0.0, "LLR should be positive for proper fuzzing, got {}", llr);
    }

    #[test]
    fn test_llr_negative_for_no_fuzzing() {
        // All deltas are 0 (no fuzzing)
        let counts: DeltaCounts = [0, 100, 0];
        let llr = compute_llr(&counts, &[0, 0, 0]);
        assert!(llr < 0.0, "LLR should be negative for no fuzzing, got {}", llr);
    }

    #[test]
    fn test_llr_zero_with_no_data() {
        let counts: DeltaCounts = [0, 0, 0];
        let llr = compute_llr(&counts, &counts);
        assert!(float_equals(llr, 0.0), "LLR should be 0 with no data");
    }

    #[test]
    fn test_chi_squared_high_p_for_expected_distribution() {
        // Close to expected 25/50/25
        let counts: DeltaCounts = [25, 50, 25];
        let p = chi_squared_test(&counts).unwrap();
        assert!(p > 0.05, "p-value should be high for expected distribution, got {}", p);
    }

    #[test]
    fn test_chi_squared_low_p_for_degenerate_distribution() {
        // All zeros — no fuzzing at all
        let counts: DeltaCounts = [0, 100, 0];
        let p = chi_squared_test(&counts).unwrap();
        assert!(p < 0.001, "p-value should be very low for all-zero deltas, got {}", p);
    }

    #[test]
    fn test_chi_squared_none_for_insufficient_data() {
        let counts: DeltaCounts = [2, 5, 3];
        assert!(chi_squared_test(&counts).is_none(), "Should return None for small samples");
    }

    #[test]
    fn test_analyze_proper_2d_fuzzing() {
        let target = Coord::new(0.5, 0.5);
        let fuzz_px = Coord::new(0.5 + UNIT, 0.5);
        let fuzz_mx = Coord::new(0.5 - UNIT, 0.5);
        let fuzz_py = Coord::new(0.5, 0.5 + UNIT);
        let fuzz_my = Coord::new(0.5, 0.5 - UNIT);

        // ~50% target, ~25% ±1 per axis (roughly)
        let mut outputs = Vec::new();
        for _ in 0..25 { outputs.push(target); }
        for _ in 0..12 { outputs.push(fuzz_px); }
        for _ in 0..13 { outputs.push(fuzz_mx); }
        for _ in 0..12 { outputs.push(fuzz_py); }
        for _ in 0..13 { outputs.push(fuzz_my); }

        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);
        let (x_counts, _y_counts) = accumulate_deltas(&events);

        let llr = compute_llr(&x_counts, &[0, 0, 0]);
        assert!(llr > 0.0, "LLR should be positive for properly fuzzed data, got {}", llr);
    }

    #[test]
    fn test_analyze_no_fuzzing_2d() {
        let target = Coord::new(0.5, 0.5);
        // 30 holds of the exact same coordinate — no fuzzing
        let outputs: Vec<Coord> = (0..30).map(|_| target).collect();
        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);
        let (x_counts, y_counts) = accumulate_deltas(&events);

        let llr = compute_llr(&x_counts, &y_counts);
        assert!(llr < 0.0, "LLR should be negative for unfuzzed data, got {}", llr);
    }

    #[test]
    fn test_analyze_cardinals_exempt() {
        let cardinal = Coord::new(1.0, 0.0);
        let outputs: Vec<Coord> = (0..50).map(|_| cardinal).collect();
        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);

        // Cardinals should produce no fuzz events
        assert_eq!(events.len(), 0, "Cardinals should not produce fuzz events");
    }

    #[test]
    fn test_analyze_1d_deadzone_fuzzing() {
        let target = Coord::new(0.5, 0.0);
        let fuzz_plus = Coord::new(0.5 + UNIT, 0.0);
        let fuzz_minus = Coord::new(0.5 - UNIT, 0.0);

        // ~50% target, ~25% each neighbor
        let mut outputs = Vec::new();
        for _ in 0..25 { outputs.push(target); }
        for _ in 0..12 { outputs.push(fuzz_plus); }
        for _ in 0..13 { outputs.push(fuzz_minus); }

        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);
        let (x_counts, y_counts) = accumulate_deltas(&events);

        // X should have fuzz data, Y should be empty (deadzone)
        let x_total: usize = x_counts.iter().sum();
        let y_total: usize = y_counts.iter().sum();
        assert!(x_total > 0, "X-axis should have fuzz events for horizontal deadzone");
        assert_eq!(y_total, 0, "Y-axis should have no fuzz events (in deadzone)");

        let llr = compute_llr(&x_counts, &y_counts);
        assert!(llr > 0.0, "LLR should be positive for properly fuzzed 1D data");
    }

    #[test]
    fn test_below_threshold_passes() {
        // Only a few holds — insufficient data should pass
        let target = Coord::new(0.5, 0.5);
        let outputs: Vec<Coord> = (0..5).map(|_| target).collect();
        let coords = make_targeting_sequence(&outputs);

        let analysis = analyze(&coords);
        assert!(analysis.pass, "Insufficient data should default to pass");
    }

    #[test]
    fn test_overlapping_targets_excluded() {
        // Two targets 1 unit apart on X — their fuzz zones overlap.
        // Target A at (0.5, 0.5), Target B at (0.5 + UNIT, 0.5).
        // The coordinate (0.5 + UNIT, 0.5) is both B itself AND A's +1 fuzz output.
        // These contested keys should be excluded from the analysis.
        let target_a = Coord::new(0.5, 0.5);
        let target_b = Coord::new(0.5 + UNIT, 0.5);

        let mut outputs = Vec::new();
        // 20 holds on A, 20 holds on B — roughly equal frequency
        for _ in 0..20 { outputs.push(target_a); }
        for _ in 0..20 { outputs.push(target_b); }

        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);

        // Both target_a and target_b are within each other's fuzz zone,
        // so both should be contested and produce no events.
        assert_eq!(
            events.len(), 0,
            "Overlapping targets should produce no events, got {}",
            events.len()
        );
    }

    #[test]
    fn test_well_separated_targets_not_affected() {
        // Two targets far apart — no overlap, both should produce events normally.
        let target_a = Coord::new(0.5, 0.5);
        let target_b = Coord::new(0.3, 0.3);

        let mut outputs = Vec::new();
        for _ in 0..15 { outputs.push(target_a); }
        for _ in 0..15 { outputs.push(target_b); }

        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);

        // Both targets are well separated, no overlap — should get events from both
        assert_eq!(
            events.len(), 30,
            "Well-separated targets should produce events for all holds, got {}",
            events.len()
        );
    }

    #[test]
    fn test_chi_sq_survival_df2() {
        // Known values for chi-squared df=2 survival function
        assert!(float_equals(chi_sq_survival_df2(0.0), 1.0));
        // P(X > 5.991) ≈ 0.05 for df=2
        let p = chi_sq_survival_df2(5.991);
        assert!((p - 0.05).abs() < 0.001, "P(X > 5.991) should be ~0.05, got {}", p);
        // P(X > 13.816) ≈ 0.001 for df=2
        let p = chi_sq_survival_df2(13.816);
        assert!((p - 0.001).abs() < 0.0005, "P(X > 13.816) should be ~0.001, got {}", p);
    }

    #[test]
    fn test_classify_rim_coordinates() {
        // Just above threshold: magnitude = sqrt(0.69^2 + 0.69^2) = 0.9758 >= 0.975
        assert_eq!(classify_coord(&Coord::new(0.69, 0.69)), CoordClass::Rim);
        // Just below threshold: magnitude = sqrt(0.68^2 + 0.68^2) = 0.9617 < 0.975
        assert_eq!(classify_coord(&Coord::new(0.68, 0.68)), CoordClass::NonCardinal);
        // Asymmetric rim: magnitude = sqrt(0.3827^2 + 0.9239^2) ≈ 1.0
        assert_eq!(classify_coord(&Coord::new(0.3827, 0.9239)), CoordClass::Rim);
    }

    #[test]
    fn test_deadzone_near_rim_stays_deadzone() {
        // Deadzone coords have one axis zero — not affected by circular clamping
        assert_eq!(classify_coord(&Coord::new(0.0, 0.975)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(0.0, 0.9875)), CoordClass::Deadzone);
        assert_eq!(classify_coord(&Coord::new(0.9875, 0.0)), CoordClass::Deadzone);
    }

    #[test]
    fn test_rim_coordinates_excluded_from_events() {
        let rim_coord = Coord::new(0.7, 0.7); // magnitude ~0.99 → Rim
        let outputs: Vec<Coord> = (0..30).map(|_| rim_coord).collect();
        let coords = make_targeting_sequence(&outputs);
        let holds = identify_holds(&coords);
        let events = cluster_and_compute_deltas(&holds);
        assert_eq!(events.len(), 0, "Rim coordinates should produce no fuzz events");
    }

    #[test]
    fn test_analyze_rim_only_passes() {
        let rim_coord = Coord::new(0.7, 0.7);
        let outputs: Vec<Coord> = (0..50).map(|_| rim_coord).collect();
        let coords = make_targeting_sequence(&outputs);
        let analysis = analyze(&coords);
        assert!(analysis.pass, "All-rim data should default to pass");
        assert_eq!(analysis.total_fuzz_events, 0);
    }

    #[test]
    fn test_mixed_rim_and_nonrim_unfuzzed_fails() {
        // Non-rim unfuzzed data should still fail even with rim holds present
        let non_rim = Coord::new(0.5, 0.5);
        let rim = Coord::new(0.7, 0.7);
        let mut outputs = Vec::new();
        for _ in 0..25 { outputs.push(non_rim); }
        for _ in 0..25 { outputs.push(rim); }
        let coords = make_targeting_sequence(&outputs);
        let analysis = analyze(&coords);
        assert!(!analysis.pass, "Unfuzzed non-rim data should still fail");
    }
}
