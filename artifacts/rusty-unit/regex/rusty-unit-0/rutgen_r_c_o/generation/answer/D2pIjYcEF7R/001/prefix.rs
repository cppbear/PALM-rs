// Answer 0

#[test]
fn test_empty_literal() {
    let literal = Literal::empty();
}

#[test]
fn test_empty_literal_is_cut() {
    let literal = Literal::empty();
    let is_cut = literal.is_cut();
}

#[test]
fn test_empty_literal_vector_length() {
    let literal = Literal::empty();
    let vector_length = literal.v.len();
}

