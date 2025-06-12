// Answer 0

#[test]
fn test_set_upper_changes_end() {
    let mut range = ClassBytesRange::default();
    range.set_upper(100);
    assert_eq!(range.end, 100);
}

#[test]
fn test_set_upper_with_lower_bound() {
    let mut range = ClassBytesRange { start: 50, end: 75 };
    range.set_upper(100);
    assert_eq!(range.end, 100);
    assert_eq!(range.start, 50);
}

#[test]
fn test_set_upper_does_not_change_lower_bound() {
    let mut range = ClassBytesRange { start: 20, end: 40 };
    range.set_upper(60);
    assert_eq!(range.start, 20);
}

#[test]
fn test_set_upper_to_same_value() {
    let mut range = ClassBytesRange { start: 10, end: 30 };
    range.set_upper(30);
    assert_eq!(range.end, 30);
    assert_eq!(range.start, 10);
}

