// Answer 0

#[test]
fn test_set_upper_with_valid_bound() {
    let mut range = ClassBytesRange::default();
    range.set_upper(10);
    assert_eq!(range.end, 10);
}

#[test]
fn test_set_upper_with_lower_bound() {
    let mut range = ClassBytesRange { start: 5, end: 15 };
    range.set_upper(7);
    assert_eq!(range.end, 7);
}

#[test]
fn test_set_upper_with_same_value() {
    let mut range = ClassBytesRange { start: 0, end: 10 };
    range.set_upper(10);
    assert_eq!(range.end, 10);
}

#[test]
fn test_set_upper_lower_bound_case() {
    let mut range = ClassBytesRange { start: 10, end: 20 };
    range.set_upper(10);
    assert_eq!(range.end, 10);
}

