// Answer 0

#[test]
fn test_suffixes_repetition_zero_or_one() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('a'))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_one_empty_string() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode(' '))),
    });
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_repetition_zero_or_one_multiple_chars() {
    let mut lits = Literals::empty();
    let expr = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    });
    suffixes(&expr, &mut lits);
}

