// Answer 0

#[test]
#[should_panic]
fn test_literal_with_ascii_byte() {
    let lit = Literal::Byte(0x7F);
    let _hir = Hir::literal(lit);
}

#[test]
fn test_literal_with_valid_unicode() {
    let lit = Literal::Unicode('a');
    let hir = Hir::literal(lit);
    assert!(hir.is_always_utf8());
}

#[test]
fn test_literal_with_invalid_byte() {
    let lit = Literal::Byte(0x80);
    let hir = Hir::literal(lit);
    assert!(!hir.is_always_utf8());
}

