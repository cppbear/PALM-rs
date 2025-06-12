// Answer 0

#[test]
fn test_set_upper_updates_end_correctly() {
    let mut range = ClassUnicodeRange { start: 'a', end: 'b' };
    range.set_upper('c');
    assert_eq!(range.end, 'c');
}

#[test]
fn test_set_upper_changes_end_from_initial_value() {
    let mut range = ClassUnicodeRange { start: 'a', end: 'b' };
    range.set_upper('d');
    assert_eq!(range.end, 'd');
}

#[test]
fn test_set_upper_updates_end_to_same_value() {
    let mut range = ClassUnicodeRange { start: 'a', end: 'b' };
    range.set_upper('b');
    assert_eq!(range.end, 'b');
}

#[test]
fn test_set_upper_does_not_change_start() {
    let mut range = ClassUnicodeRange { start: 'a', end: 'b' };
    range.set_upper('c');
    assert_eq!(range.start, 'a');
}

