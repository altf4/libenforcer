// Common test utilities shared across integration tests

use std::fs;
use std::path::PathBuf;

/// Read a .slp file from test_data directory
/// Path should be relative to test_data/ (e.g., "legal/digital/potion_p3/potion_1.slp")
pub fn read_slp_file(relative_path: &str) -> Vec<u8> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("test_data");
    let full_path = base.join(relative_path);
    fs::read(&full_path).unwrap_or_else(|_| panic!("Failed to read {}", full_path.display()))
}

/// Helper to check if float is approximately equal
pub fn assert_float_approx(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() < epsilon,
        "Expected {} to be approximately {} (±{}), but difference was {}",
        actual,
        expected,
        epsilon,
        (actual - expected).abs()
    );
}

/// Load all .slp files from a directory
/// Returns vector of (filename, file_data) tuples
pub fn read_slp_dir(relative_path: &str) -> Vec<(String, Vec<u8>)> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("test_data");
    let dir_path = base.join(relative_path);

    let mut files = Vec::new();
    let entries = fs::read_dir(&dir_path)
        .unwrap_or_else(|_| panic!("Failed to read directory {}", dir_path.display()));

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("slp") {
            let filename = entry.file_name().to_string_lossy().to_string();
            let data = fs::read(&path).unwrap();
            files.push((filename, data));
        }
    }

    // Sort by filename for consistent ordering
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}
