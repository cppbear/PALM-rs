// Answer 0

#[test]
fn test_empty_literal_creation() {
    let literal = Literal::empty();
    assert_eq!(literal.v.len(), 0);
    assert!(!literal.is_cut());
}

#[test]
fn test_literal_is_not_cut_after_empty_creation() {
    let literal = Literal::empty();
    assert!(!literal.is_cut());
}

#[test]
fn test_literal_cut_state() {
    let mut literal = Literal::empty();
    literal.cut();
    assert!(literal.is_cut());
}

