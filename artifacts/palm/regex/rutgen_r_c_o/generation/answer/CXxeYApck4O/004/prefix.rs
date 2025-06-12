// Answer 0

#[test]
fn test_prefixes_concat_with_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Byte(0)),
        Hir::literal(Literal::Byte(1)),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_concat_with_unicode_class() {
    let mut lits = Literals::empty();
    let unicode_class = ClassUnicode {
        set: IntervalSet::new(),
    };
    let expr = Hir::concat(vec![
        Hir::class(Class::Unicode(unicode_class)),
        Hir::literal(Literal::Byte(2)),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_concat_with_repetition() {
    let mut lits = Literals::empty();
    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Byte(3))),
    };
    let expr = Hir::concat(vec![
        Hir::repetition(repetition),
        Hir::literal(Literal::Byte(4)),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_alternation_with_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Byte(5)),
        Hir::literal(Literal::Byte(6)),
    ]);
    prefixes(&expr, &mut lits);
}

#[test]
fn test_prefixes_concat_non_empty() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Byte(7)),
        Hir::literal(Literal::Byte(8)),
        Hir::literal(Literal::Byte(9)),
    ]);
    prefixes(&expr, &mut lits);
}

