// Answer 0

#[test]
fn test_simplify_range_included_equal_len() {
    let range = 0..5;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_len() {
    let range = 5..5;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_unbounded_start() {
    let range = ..;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_start_end_equal() {
    let range = 0..0;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_include_zero() {
    let range = 0..2;
    let len = 2;
    simplify_range(range, len);
}

