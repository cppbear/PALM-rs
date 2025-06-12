// Answer 0

#[test]
fn test_is_match_empty_zero_or_one() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::ZeroOrOne };
    assert_eq!(repetition.is_match_empty(), true);
}

#[test]
fn test_is_match_empty_zero_or_more() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::ZeroOrMore };
    assert_eq!(repetition.is_match_empty(), true);
}

#[test]
fn test_is_match_empty_one_or_more() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::OneOrMore };
    assert_eq!(repetition.is_match_empty(), false);
}

#[test]
fn test_is_match_empty_range_exactly_zero() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::Range(RepetitionRange::Exactly(0)) };
    assert_eq!(repetition.is_match_empty(), true);
}

#[test]
fn test_is_match_empty_range_at_least_zero() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)) };
    assert_eq!(repetition.is_match_empty(), true);
}

#[test]
fn test_is_match_empty_range_bounded_zero() {
    struct TestRepetition {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    impl TestRepetition {
        fn is_match_empty(&self) -> bool {
            match self.kind {
                RepetitionKind::ZeroOrOne => true,
                RepetitionKind::ZeroOrMore => true,
                RepetitionKind::OneOrMore => false,
                RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
                RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
            }
        }
    }

    let repetition = TestRepetition { kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 1)) };
    assert_eq!(repetition.is_match_empty(), true);
}

