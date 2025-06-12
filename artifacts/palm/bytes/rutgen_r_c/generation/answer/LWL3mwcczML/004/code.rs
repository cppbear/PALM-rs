// Answer 0

#[test]
fn test_bytes_ref_debug_empty() {
    let bytes_ref = BytesRef(&[]);
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""#);
}

#[test]
fn test_bytes_ref_debug_printable() {
    let bytes_ref = BytesRef(b"Hello, World!");
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"Hello, World!""#);
}

#[test]
fn test_bytes_ref_debug_control_characters() {
    let bytes_ref = BytesRef(b"Hello\nWorld\tTest\x01\x02");
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"Hello\nWorld\tTest\x01\x02""#);
}

#[test]
fn test_bytes_ref_debug_escapes() {
    let bytes_ref = BytesRef(b"Backslash: \\ and Quote: \"");
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"Backslash: \\ and Quote: \"""#);
}

#[test]
fn test_bytes_ref_debug_null_character() {
    let bytes_ref = BytesRef(b"Null char: \0 in text");
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"Null char: \0 in text""#);
}

#[test]
fn test_bytes_ref_debug_non_printable() {
    let bytes_ref = BytesRef(b"This is a byte: \x7f");
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"This is a byte: \x7f""#);
}

