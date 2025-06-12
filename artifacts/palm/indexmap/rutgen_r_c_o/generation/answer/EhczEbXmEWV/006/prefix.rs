// Answer 0

#[test]
fn test_simplify_range_unbounded_start_included_end() {
    let len = 3;
    let range = std::ops::RangeInclusive::new(0, 2);
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_unbounded_start_excluded_end() {
    let len = 3;
    let range = std::ops::RangeExcluded::new(0, 2);
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_unbounded_start_unbounded_end() {
    let len = 3;
    let range = std::ops::Bound::Unbounded..std::ops::Bound::Included(3);
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_start_greater_than_end() {
    let len = 3;
    let range = std::ops::Range::new(2, 1); // This should panic
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_included_end() {
    let len = 3;
    let range = std::ops::Range::new(1, 3);
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_included_start_above_len() {
    let len = 3;
    let range = std::ops::RangeInclusive::new(4, 2); // This should panic
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_excluded_end_above_len() {
    let len = 3;
    let range = std::ops::Range::new(0, 4); // This should panic
    simplify_range(range, len);
}

