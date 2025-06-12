// Answer 0

#[test]
fn test_repetition_range_bounded_valid() {
    struct RepetitionRange {
        start: usize,
        end: usize,
    }
    
    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match *self {
                RepetitionRange { start, end } if start > end => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 1, end: 5 };
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_bounded_invalid() {
    struct RepetitionRange {
        start: usize,
        end: usize,
    }
    
    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match *self {
                RepetitionRange { start, end } if start > end => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 5, end: 1 };
    assert!(!range.is_valid());
}

#[test]
fn test_repetition_range_bounded_equal() {
    struct RepetitionRange {
        start: usize,
        end: usize,
    }
    
    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match *self {
                RepetitionRange { start, end } if start > end => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 3, end: 3 };
    assert!(range.is_valid());
}

#[test]
fn test_repetition_range_unbounded_valid() {
    struct RepetitionRange;

    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            true
        }
    }

    let range = RepetitionRange;
    assert!(range.is_valid());
}

