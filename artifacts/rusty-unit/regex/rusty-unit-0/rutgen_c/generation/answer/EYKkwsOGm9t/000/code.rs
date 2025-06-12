// Answer 0

#[test]
fn test_matches_within_range() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(inst_bytes.matches(5));
    assert!(inst_bytes.matches(7));
    assert!(inst_bytes.matches(10));
}

#[test]
fn test_matches_outside_range() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert!(!inst_bytes.matches(4));
    assert!(!inst_bytes.matches(11));
}

#[test]
fn test_matches_equal_to_start_and_end() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 5 };
    assert!(inst_bytes.matches(5));
    assert!(!inst_bytes.matches(4));
    assert!(!inst_bytes.matches(6));
}

#[test]
fn test_matches_empty_range() {
    let inst_bytes = InstBytes { goto: 0, start: 10, end: 5 };
    assert!(!inst_bytes.matches(5));
    assert!(!inst_bytes.matches(10));
    assert!(!inst_bytes.matches(4));
}

