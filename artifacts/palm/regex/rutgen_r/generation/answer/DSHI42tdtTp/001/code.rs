// Answer 0

#[test]
fn test_new_literal_with_empty_vector() {
    let bytes = vec![];
    let literal = regex_syntax::hir::literal::new(bytes);
    assert_eq!(literal.v, vec![]);
    assert_eq!(literal.cut, false);
}

#[test]
fn test_new_literal_with_single_byte() {
    let bytes = vec![b'a'];
    let literal = regex_syntax::hir::literal::new(bytes);
    assert_eq!(literal.v, vec![b'a']);
    assert_eq!(literal.cut, false);
}

#[test]
fn test_new_literal_with_multiple_bytes() {
    let bytes = vec![b'h', b'e', b'l', b'l', b'o'];
    let literal = regex_syntax::hir::literal::new(bytes);
    assert_eq!(literal.v, vec![b'h', b'e', b'l', b'l', b'o']);
    assert_eq!(literal.cut, false);
}

#[test]
fn test_new_literal_with_non_ascii_bytes() {
    let bytes = vec![0xC2, 0xA9]; // ©
    let literal = regex_syntax::hir::literal::new(bytes);
    assert_eq!(literal.v, vec![0xC2, 0xA9]);
    assert_eq!(literal.cut, false);
}

