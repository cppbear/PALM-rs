// Answer 0

#[test]
fn test_lower_edge_case_min() {
    let range = ClassBytesRange { start: 0, end: 0 };
    range.lower();
}

#[test]
fn test_lower_mid_point() {
    let range = ClassBytesRange { start: 128, end: 128 };
    range.lower();
}

#[test]
fn test_lower_edge_case_max() {
    let range = ClassBytesRange { start: 255, end: 255 };
    range.lower();
}

#[test]
fn test_lower_non_zero_start() {
    let range = ClassBytesRange { start: 1, end: 5 };
    range.lower();
}

#[test]
fn test_lower_inclusive_max() {
    let range = ClassBytesRange { start: 255, end: 255 };
    range.lower();
}

#[test]
fn test_lower_min_non_zero() {
    let range = ClassBytesRange { start: 2, end: 10 };
    range.lower();
}

#[test]
fn test_lower_zero_start() {
    let range = ClassBytesRange { start: 0, end: 10 };
    range.lower();
}

