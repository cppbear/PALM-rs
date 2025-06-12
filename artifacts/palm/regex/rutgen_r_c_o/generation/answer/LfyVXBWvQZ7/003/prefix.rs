// Answer 0

#[test]
fn test_repetition_range_valid_bounded_equal() {
    let range = RepetitionRange::Bounded(0, 0);
    let result = range.is_valid();
}

#[test]
fn test_repetition_range_valid_bounded_non_negative() {
    let range = RepetitionRange::Bounded(5, 5);
    let result = range.is_valid();
}

#[test]
fn test_repetition_range_valid_bounded_large_equal() {
    let range = RepetitionRange::Bounded(100, 100);
    let result = range.is_valid();
}

#[test]
fn test_repetition_range_valid_bounded_non_equal() {
    let range = RepetitionRange::Bounded(3, 4);
    let result = range.is_valid();
}

