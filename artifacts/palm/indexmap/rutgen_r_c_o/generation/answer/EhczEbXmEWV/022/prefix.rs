// Answer 0

#[test]
fn test_simplify_range_inclusive_start_exclusive_end() {
    let range = 0..3;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_inclusive_start_inclusive_end_equal() {
    let range = 0..=3;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_inclusive_start_exclusive_end_greater_than_len() {
    let range = 1..3;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_exclusive_start_inclusive_end() {
    let range = 0..=2;
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_exclusive_start_exclusive_end() {
    let range = 0..2;
    let len = 3;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_inclusive_start_out_of_bounds() {
    let range = 4..=5;
    let len = 3;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_exclusive_end_out_of_bounds() {
    let range = 0..4;
    let len = 3;
    simplify_range(range, len);
} 

#[test]
#[should_panic]
fn test_simplify_range_start_greater_than_end() {
    let range = 3..=2;
    let len = 3;
    simplify_range(range, len);
}

