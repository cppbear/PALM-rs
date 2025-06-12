// Answer 0

#[test]
fn test_is_valid_unbounded() {
    // Testing for an unbounded repetition range
    struct RepetitionRange {
        start: usize,
        end: Option<usize>,
    }

    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match self {
                RepetitionRange { start, end: Some(e) } if *start > *e => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 3, end: None };
    assert!(range.is_valid());
}

#[test]
fn test_is_valid_bounded_start_equal_end() {
    // Testing for a bounded repetition range where start equals end
    struct RepetitionRange {
        start: usize,
        end: usize,
    }

    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match self {
                RepetitionRange { start, end } if *start > *end => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 5, end: 5 };
    assert!(range.is_valid());
}

#[test]
fn test_is_valid_bounded_start_less_than_end() {
    // Testing for a bounded repetition range where start is less than end
    struct RepetitionRange {
        start: usize,
        end: usize,
    }

    impl RepetitionRange {
        fn is_valid(&self) -> bool {
            match self {
                RepetitionRange { start, end } if *start > *end => false,
                _ => true,
            }
        }
    }

    let range = RepetitionRange { start: 1, end: 10 };
    assert!(range.is_valid());
}

