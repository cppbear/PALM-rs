// Answer 0

#[test]
fn test_literal_unicode() {
    let lit = Literal::Unicode('a');
    let hir = Hir::literal(lit);
    assert_eq!(hir.kind(), &HirKind::Literal(Literal::Unicode('a')));
    assert!(hir.is_always_utf8());
    assert!(!hir.is_all_assertions());
    assert!(!hir.is_anchored_start());
    assert!(!hir.is_anchored_end());
    assert!(!hir.is_any_anchored_start());
    assert!(!hir.is_any_anchored_end());
    assert!(!hir.is_match_empty());
}

#[test]
fn test_literal_byte_valid() {
    let lit = Literal::Byte(0x80); // > 0x7F
    let hir = Hir::literal(lit);
    assert_eq!(hir.kind(), &HirKind::Literal(Literal::Byte(0x80)));
    assert!(!hir.is_always_utf8());
    assert!(!hir.is_all_assertions());
    assert!(!hir.is_anchored_start());
    assert!(!hir.is_anchored_end());
    assert!(!hir.is_any_anchored_start());
    assert!(!hir.is_any_anchored_end());
    assert!(!hir.is_match_empty());
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_literal_byte_ascii() {
    let lit = Literal::Byte(0x7F); // ASCII byte, should panic
    Hir::literal(lit);
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_literal_byte_zero() {
    let lit = Literal::Byte(0x00); // Should also panic
    Hir::literal(lit);
}

