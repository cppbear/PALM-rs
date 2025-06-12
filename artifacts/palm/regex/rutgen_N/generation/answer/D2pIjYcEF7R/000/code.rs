// Answer 0

#[derive(Debug, PartialEq)]
struct Literal {
    v: Vec<u8>,
    cut: bool,
}

impl Literal {
    fn empty() -> Literal {
        Literal { v: vec![], cut: false }
    }
}

#[test]
fn test_literal_empty() {
    let empty_literal = Literal::empty();
    assert_eq!(empty_literal, Literal { v: vec![], cut: false });
}

#[test]
fn test_literal_empty_not_null() {
    let empty_literal = Literal::empty();
    assert!(empty_literal.v.is_empty());
    assert!(!empty_literal.cut);
}

