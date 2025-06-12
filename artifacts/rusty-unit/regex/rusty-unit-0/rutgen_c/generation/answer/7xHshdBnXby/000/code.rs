// Answer 0

#[test]
fn test_literal_unicode() {
    let lit = Literal::Unicode('a');
    let hir = Hir::literal(lit);
    assert_eq!(hir.kind(), &HirKind::Literal(Literal::Unicode('a')));
    assert!(hir.is_always_utf8());
}

#[test]
fn test_literal_byte() {
    let lit = Literal::Byte(200);
    let hir = Hir::literal(lit);
    assert_eq!(hir.kind(), &HirKind::Literal(Literal::Byte(200)));
    assert!(!hir.is_always_utf8());
}

#[should_panic]
#[test]
fn test_literal_byte_ascii() {
    let lit = Literal::Byte(65); // ASCII 'A'
    Hir::literal(lit);
}

#[test]
fn test_literal_non_ascii() {
    let lit = Literal::Byte(128);
    let hir = Hir::literal(lit);
    assert_eq!(hir.kind(), &HirKind::Literal(Literal::Byte(128)));
    assert!(!hir.is_always_utf8());
}

