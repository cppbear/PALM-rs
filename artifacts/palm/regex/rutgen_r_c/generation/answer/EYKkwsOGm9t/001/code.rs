// Answer 0

#[test]
fn test_matches_byte_in_range_start_equal() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(inst_bytes.matches(5)); // Test with byte equal to start
}

#[test]
fn test_matches_byte_in_range_end_equal() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(inst_bytes.matches(10)); // Test with byte equal to end
}

#[test]
fn test_matches_byte_in_range_middle() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(inst_bytes.matches(7)); // Test with byte in the middle of the range
}

#[test]
fn test_matches_byte_below_range() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(!inst_bytes.matches(4)); // Test with byte below the range
}

#[test]
fn test_matches_byte_above_range() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(!inst_bytes.matches(11)); // Test with byte above the range
}

