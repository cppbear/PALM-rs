// Answer 0

#[test]
fn test_prefixes_literal_unicode() {
    let literal = Literal::Unicode('a');
    let expr = Hir::literal(literal);
    let mut lits = Literals::empty();
    
    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_literal_byte() {
    let literal = Literal::Byte(97);
    let expr = Hir::literal(literal);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);

    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_unicode_class() {
    let class = Class::Unicode(ClassUnicode {
        set: IntervalSet::new(vec![(97, 122)]) // a-z
    });
    let expr = Hir::class(class);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_bytes_class() {
    let class = Class::Bytes(ClassBytes {
        set: IntervalSet::new(vec![(97, 122)]) // a-z
    });
    let expr = Hir::class(class);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_group() {
    let literal = Literal::Unicode('b');
    let group_expr = Hir::literal(literal);
    let expr = Hir::group(group_expr);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_concatenation_empty() {
    let expr = Hir::concat(vec![]);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_single_concatenation() {
    let literal = Literal::Unicode('c');
    let single_expr = Hir::literal(literal);
    let expr = Hir::concat(vec![single_expr]);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_concatenation_with_anchor() {
    let literal = Literal::Unicode('d');
    let anchor = Hir::anchor(hir::Anchor::StartText);
    let expr = Hir::concat(vec![anchor, Hir::literal(literal)]);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_concatenation_with_multiple_expressions() {
    let literal1 = Literal::Byte(100); // 'd'
    let literal2 = Literal::Byte(101); // 'e'
    let expr1 = Hir::literal(literal1);
    let expr2 = Hir::literal(literal2);
    let expr = Hir::concat(vec![expr1, expr2]);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_alternation() {
    let literal = Literal::Unicode('e');
    let expr1 = Hir::literal(literal.clone());
    let expr2 = Hir::literal(literal);
    let expr = Hir::alternation(vec![expr1, expr2]);
    let mut lits = Literals::empty();

    prefixes(&expr, &mut lits);
    
    assert!(!lits.is_empty());
}

