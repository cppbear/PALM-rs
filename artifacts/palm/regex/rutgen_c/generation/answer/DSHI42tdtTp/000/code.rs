// Answer 0

#[test]
fn test_new_literal_with_bytes() {
    let bytes = vec![72, 101, 108, 108, 111]; // Represents the string "Hello"
    let literal = Literal::new(bytes.clone());
    assert_eq!(literal.v, bytes);
    assert_eq!(literal.cut, false);
}

#[test]
fn test_new_literal_with_empty_vector() {
    let bytes: Vec<u8> = vec![];
    let literal = Literal::new(bytes.clone());
    assert_eq!(literal.v, bytes);
    assert_eq!(literal.cut, false);
}

