// Answer 0

#[test]
fn test_repetition_one_or_more_empty() {
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir { kind: HirKind::SomeKind, info: HirInfo::default() }),
    };
    repetition.is_match_empty();
}

#[test]
fn test_repetition_one_or_more_non_greedy() {
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir { kind: HirKind::SomeKind, info: HirInfo::default() }),
    };
    repetition.is_match_empty();
}

#[test]
fn test_repetition_one_or_more_with_hir() {
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir { kind: HirKind::SomeKind, info: HirInfo::default() }),
    };
    repetition.is_match_empty();
}

