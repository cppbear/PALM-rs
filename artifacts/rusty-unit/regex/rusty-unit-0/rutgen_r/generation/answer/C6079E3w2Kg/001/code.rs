// Answer 0

#[test]
fn test_vb_above_max() {
    let result = vb(::std::u8::MAX as usize + 1);
    assert_eq!(result, "EOF".to_owned());
}

#[test]
fn test_vb_max() {
    let result = vb(::std::u8::MAX as usize);
    assert_eq!(result, "\\xff".to_owned());
}

#[test]
fn test_vb_zero() {
    let result = vb(0);
    assert_eq!(result, "\\0".to_owned());
}

#[test]
fn test_vb_special_characters() {
    let result = vb(10);
    assert_eq!(result, "\\n".to_owned()); // Newline character

    let result = vb(13);
    assert_eq!(result, "\\r".to_owned()); // Carriage return character

    let result = vb(34);
    assert_eq!(result, "\"".to_owned()); // Double quote character
}

