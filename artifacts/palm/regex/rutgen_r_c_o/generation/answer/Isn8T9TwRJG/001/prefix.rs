// Answer 0

#[test]
fn test_end_zero() {
    let range = ClassBytesRange::new(0, 0);
    range.end();
}

#[test]
fn test_end_minimum_range() {
    let range = ClassBytesRange::new(0, 1);
    range.end();
}

#[test]
fn test_end_middle_range() {
    let range = ClassBytesRange::new(100, 150);
    range.end();
}

#[test]
fn test_end_maximum_range() {
    let range = ClassBytesRange::new(255, 255);
    range.end();
}

#[test]
fn test_end_full_range() {
    let range = ClassBytesRange::new(0, 255);
    range.end();
}

#[test]
fn test_end_scenario_reversed() {
    let range = ClassBytesRange::new(255, 0);
    range.end();
}

