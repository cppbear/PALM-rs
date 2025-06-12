// Answer 0

#[derive(Debug)]
enum RepetitionRange {
    Bounded(usize, usize),
}

impl RepetitionRange {
    pub fn is_valid(&self) -> bool {
        match *self {
            RepetitionRange::Bounded(s, e) if s > e => false,
            _ => true,
        }
    }
}

#[test]
fn test_valid_repetition_range_equal_bounds() {
    let range = RepetitionRange::Bounded(3, 3);
    assert!(range.is_valid());
}

#[test]
fn test_valid_repetition_range_lower_bound() {
    let range = RepetitionRange::Bounded(0, 1);
    assert!(range.is_valid());
}

#[test]
fn test_valid_repetition_range_higher_bound() {
    let range = RepetitionRange::Bounded(1, 2);
    assert!(range.is_valid());
}

#[test]
fn test_invalid_repetition_range() {
    let range = RepetitionRange::Bounded(2, 1);
    assert!(!range.is_valid());
}

