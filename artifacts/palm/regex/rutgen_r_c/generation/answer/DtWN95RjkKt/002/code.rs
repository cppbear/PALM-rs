// Answer 0

#[test]
fn test_is_match_empty_zero_or_one() {
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(), // Assuming there's a constructor for HirInfo
        }),
    };
    assert!(repetition.is_match_empty());
}

#[test]
fn test_is_match_empty_zero_or_more() {
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(),
        }),
    };
    assert!(repetition.is_match_empty());
}

#[test]
fn test_is_match_empty_one_or_more() {
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(),
        }),
    };
    assert!(!repetition.is_match_empty());
}

#[test]
fn test_is_match_empty_range_exactly_zero() {
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Exactly(0)),
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(),
        }),
    };
    assert!(repetition.is_match_empty());
}

#[test]
fn test_is_match_empty_range_at_least_zero() {
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(),
        }),
    };
    assert!(repetition.is_match_empty());
}

#[test]
fn test_is_match_empty_range_bounded_zero() {
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 1)), // assuming 1 is the upper bound
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::SomeKind, // Replace with an actual variant if it exists
            info: HirInfo::new(),
        }),
    };
    assert!(repetition.is_match_empty());
}

