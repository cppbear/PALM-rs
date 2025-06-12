// Answer 0

#[test]
fn test_suffixes_concat_empty() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_single_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::literal(Literal::Unicode('a'))]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_multiple_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Byte(1)),
        Hir::literal(Literal::Unicode('b')),
    ]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_empty_nested() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::concat(vec![])]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_byte_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::literal(Literal::Byte(255))]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_empty_and_literal() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![Hir::literal(Literal::Byte(2)), Hir::concat(vec![])]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_empty_with_limit() {
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    let expr = Hir::concat(vec![]);
    suffixes(&expr, &mut lits);
}

