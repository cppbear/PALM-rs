// Answer 0

#[test]
fn test_prefixes_with_unicode_lowercase() {
    let lit = Hir::literal(Literal::Unicode('a'));
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

#[test]
fn test_prefixes_with_unicode_uppercase() {
    let lit = Hir::literal(Literal::Unicode('A'));
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

#[test]
fn test_prefixes_with_byte_0() {
    let lit = Hir::literal(Literal::Byte(0));
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

#[test]
fn test_prefixes_with_byte_255() {
    let lit = Hir::literal(Literal::Byte(255));
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

#[test]
fn test_prefixes_with_unicode_multiple() {
    let lit = Hir::concat(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Unicode('b')),
        Hir::literal(Literal::Unicode('c')),
    ]);
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

#[test]
fn test_prefixes_with_mixed_literals() {
    let lit = Hir::concat(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Byte(65)),
    ]);
    let mut lits = Literals::empty();
    prefixes(&lit, &mut lits);
}

