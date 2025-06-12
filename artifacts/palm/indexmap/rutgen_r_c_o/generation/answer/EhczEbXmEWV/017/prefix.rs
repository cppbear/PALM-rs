// Answer 0

#[test]
fn test_simplify_range_excluded_start_upper_bound() {
    let range = 0..0;
    let len = 1;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_equal_len() {
    let range = 1..1;
    let len = 1;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_less_than_len() {
    let range = 2..2;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_greater_than_len() {
    let range = 4..4;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_just_below_len() {
    let range = 3..4;
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_excluded_start_panic_above_len() {
    let range = 5..5;
    let len = 4;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_len() {
    let range = 9..9;
    let len = 10;
    simplify_range(range, len);
}

