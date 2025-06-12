// Answer 0

#[derive(Default)]
struct Literal {
    cut: bool,
}

#[test]
fn test_literal_is_cut_true() {
    let literal = Literal { cut: true };
    assert_eq!(literal.is_cut(), true);
}

#[test]
fn test_literal_is_cut_false() {
    let literal = Literal { cut: false };
    assert_eq!(literal.is_cut(), false);
}

#[test]
fn test_literal_is_cut_default() {
    let literal = Literal::default();
    assert_eq!(literal.is_cut(), false);
}

