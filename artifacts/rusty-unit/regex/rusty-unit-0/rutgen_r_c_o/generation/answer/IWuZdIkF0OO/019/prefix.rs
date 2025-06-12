// Answer 0

#[test]
fn test_suffixes_repetition_zero_or_more_empty() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::empty()),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more_single_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more_multiple_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::concat(vec![
            Hir::literal(Literal::Unicode('b')),
            Hir::literal(Literal::Unicode('c')),
        ])),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more_empty_class() {
    let mut lits = Literals::empty();
    let cls = Class::Unicode(ClassUnicode { set: IntervalSet::<ClassUnicodeRange>::new() });
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::class(cls)),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more_byte_class() {
    let mut lits = Literals::empty();
    let cls = Class::Bytes(ClassBytes { set: IntervalSet::<ClassBytesRange>::new() });
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::class(cls)),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_more_with_cuts() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Unicode('d'))),
    });
    suffixes(&expr, &mut lits);
}

