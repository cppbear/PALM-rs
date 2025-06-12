// Answer 0

#[test]
fn test_repetition_range_exactly_valid() {
    let range = RepetitionRange::Exactly(5);
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_at_least_valid() {
    let range = RepetitionRange::AtLeast(3);
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_bounded_valid() {
    let range = RepetitionRange::Bounded(2, 5);
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_bounded_equal() {
    let range = RepetitionRange::Bounded(3, 3);
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_bounded_invalid() {
    let range = RepetitionRange::Bounded(5, 3);
    assert!(!range.is_valid());
}

