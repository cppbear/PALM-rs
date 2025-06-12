// Answer 0

#[test]
fn test_is_cut_when_cut_is_true() {
    let literal = Literal {
        v: vec![b'a', b'b', b'c'],
        cut: true,
    };
    assert_eq!(literal.is_cut(), true);
}

#[test]
fn test_is_cut_when_cut_is_false() {
    let literal = Literal {
        v: vec![b'a', b'b', b'c'],
        cut: false,
    };
    assert_eq!(literal.is_cut(), false);
}

#[test]
fn test_is_cut_on_empty_literal() {
    let literal = Literal::empty();
    assert_eq!(literal.is_cut(), false);
}

