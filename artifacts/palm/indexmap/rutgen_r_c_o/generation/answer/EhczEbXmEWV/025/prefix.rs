// Answer 0

#[test]
fn test_simplify_range_start_included_end_included_equal_length() {
    simplify_range(0..=5, 5);
}

#[test]
fn test_simplify_range_start_included_end_included_zero_length() {
    simplify_range(0..=0, 0);
}

#[test]
fn test_simplify_range_start_included_end_included_high_length() {
    simplify_range(0..=10, 10);
}

#[test]
fn test_simplify_range_start_excluded_end_included_equal_length() {
    simplify_range(5..=5, 5);
}

#[test]
fn test_simplify_range_start_included_end_excluded_equal_length() {
    simplify_range(5..5, 5);
}

