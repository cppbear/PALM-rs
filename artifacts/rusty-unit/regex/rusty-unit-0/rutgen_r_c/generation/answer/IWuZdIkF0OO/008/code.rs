// Answer 0

#[test]
fn test_suffixes_with_unicode_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::literal(Literal::Unicode('a'));
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_byte_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::literal(Literal::Byte(255));
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_unicode_class() {
    let mut lits = Literals::empty();
    let cls = ClassUnicode { set: IntervalSet::new(vec![ClassUnicodeRange::new('a' as u32, 'z' as u32)]) };
    let expr = Hir::class(Class::Unicode(cls));
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_bytes_class() {
    let mut lits = Literals::empty();
    let cls = ClassBytes { set: IntervalSet::new(vec![ClassBytesRange::new(0, 255)]) };
    let expr = Hir::class(Class::Bytes(cls));
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_group() {
    let mut lits = Literals::empty();
    let group_expr = Hir::literal(Literal::Unicode('b'));
    let expr = Hir::group(Group { hir: Box::new(group_expr) });
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_zero_or_more_repetition() {
    let mut lits = Literals::empty();
    let rep = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('c')))
    };
    let expr = Hir::repetition(rep);
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_one_or_more_repetition() {
    let mut lits = Literals::empty();
    let rep = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('d')))
    };
    let expr = Hir::repetition(rep);
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_bounded_repetition() {
    let mut lits = Literals::empty();
    let rep = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 3)),
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('e')))
    };
    let expr = Hir::repetition(rep);
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_concat() {
    let mut lits = Literals::empty();
    let concatenated_expr = Hir::alternation(vec![
        Hir::literal(Literal::Byte(1)),
        Hir::literal(Literal::Byte(2))
    ]);
    let expr = Hir::concat(vec![concatenated_expr]);
    suffixes(&expr, &mut lits);
    assert!(lits.any_complete());
}

#[test]
fn test_suffixes_with_empty_concat() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![]);
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_with_anchor_end_text() {
    let mut lits = Literals::empty();
    let anchor_end_text = Hir::anchor(Anchor::EndText);
    let expr = Hir::concat(vec![anchor_end_text]);
    suffixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

