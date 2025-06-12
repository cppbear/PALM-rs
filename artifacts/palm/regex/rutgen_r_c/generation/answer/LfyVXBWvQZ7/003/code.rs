// Answer 0

#[test]
fn test_repetition_range_bounded_valid_equal_bounds() {
    let range = RepetitionRange::Bounded(5, 5);
    assert_eq!(range.is_valid(), true);
}

#[test]
fn test_repetition_range_bounded_valid_non_negative_bounds() {
    let range = RepetitionRange::Bounded(0, 10);
    assert_eq!(range.is_valid(), true);
}

#[test]
fn test_repetition_range_bounded_valid_reversed_bounds() {
    let range = RepetitionRange::Bounded(3, 4);
    assert_eq!(range.is_valid(), true);
}

