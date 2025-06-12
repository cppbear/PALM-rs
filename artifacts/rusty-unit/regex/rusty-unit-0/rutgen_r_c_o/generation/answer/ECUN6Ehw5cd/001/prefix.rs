// Answer 0

#[test]
fn test_set_upper_zero() {
    let mut range = ClassBytesRange::default();
    range.set_upper(0);
}

#[test]
fn test_set_upper_mid() {
    let mut range = ClassBytesRange::default();
    range.set_upper(128);
}

#[test]
fn test_set_upper_max() {
    let mut range = ClassBytesRange::default();
    range.set_upper(255);
}

#[test]
fn test_set_upper_lower_than_current() {
    let mut range = ClassBytesRange { start: 0, end: 100 };
    range.set_upper(50);
}

#[test]
fn test_set_upper_equal_current() {
    let mut range = ClassBytesRange { start: 0, end: 100 };
    range.set_upper(100);
}

