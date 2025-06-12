// Answer 0

#[test]
fn test_empty_literal() {
    let literal = Literal::empty();
    assert_eq!(literal.v, vec![]);
    assert_eq!(literal.cut, false);
}

#[test]
fn test_empty_literal_properties() {
    let literal = Literal::empty();
    assert!(!literal.is_cut());
}

