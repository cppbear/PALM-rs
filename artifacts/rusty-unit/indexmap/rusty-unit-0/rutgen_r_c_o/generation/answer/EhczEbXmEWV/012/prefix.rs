// Answer 0

#[test]
fn test_simplify_range_excluded_boundaries_1() {
    let range = (Bound::Excluded(0), Bound::Excluded(1));
    let len = 1;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_boundaries_2() {
    let range = (Bound::Excluded(1), Bound::Excluded(2));
    let len = 2;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_boundaries_3() {
    let range = (Bound::Excluded(2), Bound::Excluded(3));
    let len = 3;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_boundaries_4() {
    let range = (Bound::Excluded(3), Bound::Excluded(4));
    let len = 4;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_to_unbounded() {
    let range = (Bound::Excluded(4), Bound::Unbounded);
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_unbounded_to_excluded() {
    let range = (Bound::Unbounded, Bound::Excluded(5));
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_equal_len() {
    let range = (Bound::Excluded(5), Bound::Excluded(5));
    let len = 5;
    simplify_range(range, len);
}

