// Answer 0

#[test]
fn test_fmt_char_valid() {
    let c = Char(65); // Character 'A'
    let mut buf = String::new();
    let result = fmt(&c, &mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"A\"");
}

#[test]
fn test_fmt_char_empty() {
    let c = Char(u32::MAX); // Invalid Unicode code point
    let mut buf = String::new();
    let result = fmt(&c, &mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "Empty");
}

