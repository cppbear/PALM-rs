// Answer 0

#[test]
fn test_literal_new_with_non_empty_bytes() {
    let bytes = vec![104, 101, 108, 108, 111]; // Corresponds to the string "hello"
    let literal = Literal::new(bytes.clone());
    assert_eq!(literal.v, bytes);
    assert!(!literal.cut);
}

#[test]
fn test_literal_new_with_empty_bytes() {
    let bytes: Vec<u8> = vec![];
    let literal = Literal::new(bytes.clone());
    assert_eq!(literal.v, bytes);
    assert!(!literal.cut);
}

#[test]
fn test_literal_new_with_special_characters() {
    let bytes = vec![33, 64, 35, 36, 37]; // Corresponds to the characters "!@#$%"
    let literal = Literal::new(bytes.clone());
    assert_eq!(literal.v, bytes);
    assert!(!literal.cut);
}

