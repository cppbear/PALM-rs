// Answer 0

#[test]
#[should_panic]
fn test_simplify_range_start_included_out_of_bounds() {
    let range = 10..15; // Range with included start greater than length
    let len = 10;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_end_included_out_of_bounds() {
    let range = 5..20; // Range with included end greater than length
    let len = 10;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_start_excluded_out_of_bounds() {
    let range = 10..=15; // Range with excluded start greater than length
    let len = 10;
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_end_excluded_out_of_bounds() {
    let range = 5..=20; // Range with excluded end greater than length
    let len = 10;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_start_included_at_length() {
    let range = 10..15; // Start is at length, but should not panic
    let len = 15;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_end_included_at_length() {
    let range = 5..15; // End included at length, valid case
    let len = 15;
    simplify_range(range, len);
}

