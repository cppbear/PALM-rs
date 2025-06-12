// Answer 0

#[test]
fn test_suffixes_one_or_more_repetition_unicode() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_one_or_more_repetition_byte() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Byte(b'a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_one_or_more_repetition_unicode_empty_group() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::group(hir::Group { hir: Box::new(Hir::empty()), ..Default::default() })),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_one_or_more_repetition_bounded() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::Bounded(1, 100),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    });
    suffixes(&expr, &mut lits);
}

