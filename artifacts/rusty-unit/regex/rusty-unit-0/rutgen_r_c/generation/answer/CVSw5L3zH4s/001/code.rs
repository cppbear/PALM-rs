// Answer 0

#[test]
fn test_is_cut_true() {
    let literal = Literal {
        v: vec![b'a', b'b', b'c'],
        cut: true,
    };
    assert_eq!(literal.is_cut(), true);
}

#[test]
fn test_is_cut_false() {
    let literal = Literal {
        v: vec![b'x', b'y', b'z'],
        cut: false,
    };
    assert_eq!(literal.is_cut(), false);
}

#[test]
fn test_is_cut_empty_literal_not_cut() {
    let literal = Literal::empty();
    assert_eq!(literal.is_cut(), false);
}

#[test]
fn test_is_cut_empty_literal_cut() {
    let mut literal = Literal::empty();
    literal.cut();
    assert_eq!(literal.is_cut(), true);
}

