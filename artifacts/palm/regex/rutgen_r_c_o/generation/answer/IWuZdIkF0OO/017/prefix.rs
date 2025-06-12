// Answer 0

#[test]
fn test_suffixes_repetition_exactly_zero() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(0)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_exactly_one() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(1)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_exactly_fifty() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(50)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Byte(1))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_exactly_ninety_nine() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(99)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('z'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_exactly_one_hundred() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(100)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Byte(255))),
    });
    suffixes(&expr, &mut lits);
}

