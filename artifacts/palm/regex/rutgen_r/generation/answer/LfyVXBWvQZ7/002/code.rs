// Answer 0

#[derive(Debug)]
enum RepetitionRange {
    Bounded(usize, usize),
}

impl RepetitionRange {
    pub fn is_valid(&self) -> bool {
        match self {
            RepetitionRange::Bounded(s, e) if s > e => false,
            _ => true,
        }
    }
}

#[test]
fn test_bounded_invalid_range() {
    let range = RepetitionRange::Bounded(5, 3);
    assert_eq!(range.is_valid(), false);
}

#[test]
fn test_bounded_invalid_range_equal() {
    let range = RepetitionRange::Bounded(4, 4);
    assert_eq!(range.is_valid(), true);
}

#[test]
fn test_bounded_valid_range() {
    let range = RepetitionRange::Bounded(2, 5);
    assert_eq!(range.is_valid(), true);
}

