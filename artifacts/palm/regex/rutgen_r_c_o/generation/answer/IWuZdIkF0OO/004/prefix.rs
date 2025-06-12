// Answer 0

#[test]
fn test_suffixes_concat_with_unicode_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Unicode('b')),
    ]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_byte_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Byte(1)),
        Hir::literal(Literal::Byte(2)),
    ]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_mixed_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Unicode('x')),
        Hir::literal(Literal::Byte(100)),
        Hir::literal(Literal::Unicode('z')),
    ]);
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_multiple_unicode_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat((b'a'..=b'z')
        .map(|b| Hir::literal(Literal::Unicode(b as char)))
        .collect());
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_multiple_byte_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat((1..=10)
        .map(|b| Hir::literal(Literal::Byte(b)))
        .collect());
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_with_empty_and_non_empty_literals() {
    let mut lits = Literals::empty();
    let expr = Hir::concat(vec![
        Hir::literal(Literal::Unicode('c')),
        Hir::literal(Literal::Byte(250)),
        Hir::literal(Literal::Byte(255)),
        Hir::literal(Literal::Unicode('d')),
    ]);
    suffixes(&expr, &mut lits);
}

