// Answer 0

#[test]
fn test_exactly_is_valid() {
    let range = RepetitionRange::Exactly(2);
    assert!(range.is_valid());
}

#[test]
fn test_at_least_is_valid() {
    let range = RepetitionRange::AtLeast(3);
    assert!(range.is_valid());
}

#[test]
fn test_bounded_valid() {
    let range = RepetitionRange::Bounded(1, 5);
    assert!(range.is_valid());
}

#[test]
fn test_bounded_invalid() {
    let range = RepetitionRange::Bounded(5, 2);
    assert!(!range.is_valid());
} 

#[test]
fn test_bounded_equal() {
    let range = RepetitionRange::Bounded(3, 3);
    assert!(range.is_valid());
}

