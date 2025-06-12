// Answer 0

#[test]
fn test_is_match_empty_one_or_more() {
    struct Repetition {
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

    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
    };

    assert_eq!(repetition.is_match_empty(), false);
}

