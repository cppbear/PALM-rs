// Answer 0

#[test]
fn test_set_lower() {
    let mut range = ClassBytesRange::default();
    range.set_lower(100);
    assert_eq!(range.start, 100);
}

#[test]
fn test_set_lower_edge_case() {
    let mut range = ClassBytesRange::default();
    range.set_lower(0);
    assert_eq!(range.start, 0);
}

#[test]
fn test_set_lower_overwrite() {
    let mut range = ClassBytesRange { start: 50, end: 200 };
    range.set_lower(25);
    assert_eq!(range.start, 25);
}

