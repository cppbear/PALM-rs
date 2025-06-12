// Answer 0

#[test]
fn test_suffixes_repetition_range_bounded_valid() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(10, 50)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_range_bounded_edge_min() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 100)),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Byte(255))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_range_bounded_edge_max() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(100, 1000)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('z'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_range_bounded_large() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(30, 300)),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Byte(100))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_range_bounded_min_max_equal() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(5, 5)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('!'))),
    });
    suffixes(&expr, &mut lits);
}

