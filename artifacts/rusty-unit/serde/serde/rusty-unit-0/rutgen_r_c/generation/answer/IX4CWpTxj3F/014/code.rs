// Answer 0

#[test]
fn test_fmt_for_char() {
    use std::fmt;

    let char_value = 'a';
    let unexpected_value = Unexpected::Char(char_value);
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| unexpected_value.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "character `a`");
}

#[test]
fn test_fmt_for_unsigned() {
    use std::fmt;

    let unsigned_value = 42u64;
    let unexpected_value = Unexpected::Unsigned(unsigned_value);
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| unexpected_value.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "integer `42`");
}

