// Answer 0

#[test]
fn test_valid_exactly_zero() {
    let repetition = RepetitionRange::Exactly(0);
    repetition.is_valid();
}

#[test]
fn test_valid_at_least_zero() {
    let repetition = RepetitionRange::AtLeast(0);
    repetition.is_valid();
}

#[test]
fn test_valid_bounded_equal() {
    let repetition = RepetitionRange::Bounded(0, 0);
    repetition.is_valid();
}

#[test]
fn test_valid_bounded_s_and_e() {
    let repetition = RepetitionRange::Bounded(5, 5);
    repetition.is_valid();
}

#[test]
fn test_valid_bounded_zero_and_positive() {
    let repetition = RepetitionRange::Bounded(0, 10);
    repetition.is_valid();
}

#[test]
fn test_valid_bounded_small_values() {
    let repetition = RepetitionRange::Bounded(3, 7);
    repetition.is_valid();
}

