// Answer 0

#[test]
fn test_prefixes_repetition_exactly_1() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(1))),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_exactly_5() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(5))),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_exactly_10() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(10))),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('c'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_exactly_100() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(100))),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('d'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_exactly_1000() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(1000))),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('e'))),
    });
    prefixes(&expr, &mut lits);
}

