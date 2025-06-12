// Answer 0

#[test]
#[should_panic]
fn test_literal_with_byte_variant_at_boundary() {
    use regex_syntax::hir::{literal, Hir, Literal};

    let lit = Literal::Byte(0x7F);
    let _ = literal(lit);
}

#[test]
fn test_literal_with_unicode_variant() {
    use regex_syntax::hir::{literal, Hir, Literal};

    let lit = Literal::Unicode('\u{30A0}'); // Example Unicode character
    let result = literal(lit);
    assert_eq!(result.kind, HirKind::Literal(lit));
}

#[test]
fn test_literal_with_byte_variant_above_boundary() {
    use regex_syntax::hir::{literal, Hir, Literal};

    let lit = Literal::Byte(0x80);
    let result = literal(lit);
    assert_eq!(result.kind, HirKind::Literal(lit));
}

