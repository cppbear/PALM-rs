// Answer 0

#[test]
fn test_prefixes_repetition_bounded_zero_to_zero() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(0, 0)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_bounded_one_to_one() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 1)),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Byte(5))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_bounded_five_to_ten() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(5, 10)),
        greedy: true,
        hir: Box::new(Hir::class(Class::Unicode(ClassUnicode { set: IntervalSet::new() }))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_bounded_ten_to_ten() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(10, 10)),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Unicode('z'))),
    });
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_repetition_bounded_max() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1000, 1000)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Byte(255))),
    });
    prefixes(&expr, &mut lits);
}

