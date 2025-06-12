// Answer 0

#[test]
fn test_bytes_ref_debug_empty() {
    let data = &[];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""#);
}

#[test]
fn test_bytes_ref_debug_newline() {
    let data = &[b'\n'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\n""#);
}

#[test]
fn test_bytes_ref_debug_carriage_return() {
    let data = &[b'\r'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\r""#);
}

#[test]
fn test_bytes_ref_debug_tab() {
    let data = &[b'\t'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\t""#);
}

#[test]
fn test_bytes_ref_debug_backslash() {
    let data = &[b'\\'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\\"#);
}

#[test]
fn test_bytes_ref_debug_double_quote() {
    let data = &[b'"'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""""#);
}

#[test]
fn test_bytes_ref_debug_null() {
    let data = &[b'\0'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\0""#);
}

#[test]
fn test_bytes_ref_debug_printable() {
    let data = &[b'A', b'B', b'C'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"ABC""#);
}

#[test]
fn test_bytes_ref_debug_non_printable() {
    let data = &[0x1F, 0x20, 0x7F];  // Non-printable and space
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\x1f \x7f""#);
}

