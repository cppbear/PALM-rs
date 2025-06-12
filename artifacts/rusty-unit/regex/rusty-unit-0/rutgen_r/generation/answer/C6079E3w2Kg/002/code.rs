// Answer 0

#[test]
fn test_vb_with_max_u8() {
    let result = vb(::std::u8::MAX as usize);
    assert_eq!(result, r"\xFF");
}

#[test]
fn test_vb_with_below_max_u8() {
    let result = vb(::std::u8::MAX as usize - 1);
    assert_eq!(result, r"\xFE");
}

#[test]
fn test_vb_with_zero() {
    let result = vb(0);
    assert_eq!(result, r"\x00");
}

#[test]
fn test_vb_with_non_ascii() {
    let result = vb(128);
    assert_eq!(result, r"\x80");
}

#[test]
fn test_vb_with_eof() {
    let result = vb(::std::u8::MAX as usize + 1);
    assert_eq!(result, "EOF");
}

