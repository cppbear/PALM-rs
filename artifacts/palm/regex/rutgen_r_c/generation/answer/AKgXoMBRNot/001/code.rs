// Answer 0

#[test]
fn test_set_lower() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower('a');
    assert_eq!(range.start, 'a');
}

#[test]
fn test_set_lower_reassign() {
    let mut range = ClassUnicodeRange { start: 'x', end: 'y' };
    range.set_lower('b');
    assert_eq!(range.start, 'b');
}

#[test]
fn test_set_lower_with_multiple_reassignments() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower('m');
    assert_eq!(range.start, 'm');
    range.set_lower('n');
    assert_eq!(range.start, 'n');
}

#[test]
fn test_set_lower_to_same_value() {
    let mut range = ClassUnicodeRange { start: 'c', end: 'd' };
    range.set_lower('c');
    assert_eq!(range.start, 'c');
}

