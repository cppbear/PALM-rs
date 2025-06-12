// Answer 0

#[test]
fn test_prefixes_empty_concat() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![]);
    prefixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_single_empty_lit_concat() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::literal(Literal::empty())]);
    prefixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_concat_with_empty_and_unicode() {
    let mut lits = Literals::empty();
    let unicode_lit = Hir::literal(Literal::Unicode('a'));
    let expr = Hir::concat(vec![Hir::literal(Literal::empty()), unicode_lit]);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.contains_empty());
}

#[test]
fn test_prefixes_concat_with_literal_and_empty() {
    let mut lits = Literals::empty();
    let unicode_lit = Hir::literal(Literal::Unicode('b'));
    let expr = Hir::concat(vec![unicode_lit, Hir::literal(Literal::empty())]);
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
    assert!(lits.any_complete());
}

#[test]
fn test_prefixes_concat_with_multiple_empty_lits() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::literal(Literal::empty()), Hir::literal(Literal::empty())]);
    prefixes(&expr, &mut lits);
    assert!(lits.all_complete());
}

