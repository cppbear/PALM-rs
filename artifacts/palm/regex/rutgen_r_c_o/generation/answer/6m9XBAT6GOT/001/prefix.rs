// Answer 0

#[test]
fn test_repetition_zero_or_one() {
    let rep = Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    repetition(rep);
}

#[test]
fn test_repetition_zero_or_more() {
    let rep = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    repetition(rep);
}

#[test]
fn test_repetition_exactly_zero() {
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Exactly(0)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    repetition(rep);
}

#[test]
fn test_repetition_at_least_zero() {
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    repetition(rep);
}

#[test]
fn test_repetition_bounded_zero() {
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 0)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    repetition(rep);
}

