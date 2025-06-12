// Answer 0

#[test]
fn test_prefixes_literal_unicode() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('a');
    let expr = Hir::literal(unicode_literal.clone());
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_literal_byte() {
    let mut lits = Literals::empty();
    let byte_literal = Literal::Byte(97); // ASCII for 'a'
    let expr = Hir::literal(byte_literal.clone());
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_class_unicode() {
    let mut lits = Literals::empty();
    let class_unicode = Class::Unicode(ClassUnicode { set: IntervalSet::new() });
    let expr = Hir::class(class_unicode);
    let result = lits.add_char_class(&class_unicode);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(result);
}

#[test]
fn test_prefixes_class_bytes() {
    let mut lits = Literals::empty();
    let class_bytes = Class::Bytes(ClassBytes { set: IntervalSet::new() });
    let expr = Hir::class(class_bytes);
    let result = lits.add_byte_class(&class_bytes);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(result);
}

#[test]
fn test_prefixes_group() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('b');
    let expr = Hir::group(Group { hir: Box::new(Hir::literal(unicode_literal)) });
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_repetition_zero_or_more() {
    let mut lits = Literals::empty();
    let unicode_literal = Literal::Unicode('c');
    let expr = Hir::repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(unicode_literal)),
    });
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_concatenation() {
    let mut lits = Literals::empty();
    let expr1 = Hir::literal(Literal::Unicode('d'));
    let expr2 = Hir::literal(Literal::Byte(101)); // 'e'
    let expr = Hir::concat(vec![expr1, expr2]);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_alternation() {
    let mut lits = Literals::empty();
    let expr1 = Hir::literal(Literal::Unicode('f'));
    let expr2 = Hir::literal(Literal::Byte(103)); // 'g'
    let expr = Hir::alternation(vec![expr1, expr2]);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.any_complete());
}

