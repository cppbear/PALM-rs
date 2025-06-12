// Answer 0

#[test]
fn test_is_valid_below_lower_bound() {
    assert_eq!(is_valid(31), false); // b < 32 should return false
}

#[test]
fn test_is_valid_control_character() {
    assert_eq!(is_valid(b'\t'), true); // b is tab character should return true
}

#[test]
fn test_is_valid_upper_bound_exclusive() {
    assert_eq!(is_valid(127), false); // b == 127 should return false
}

#[test]
fn test_is_valid_boundary_case() {
    assert_eq!(is_valid(32), true); // b == 32 should return true
}

