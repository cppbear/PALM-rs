// Answer 0

#[test]
fn test_fmt_empty() {
    let data = &[];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\"");
}

#[test]
fn test_fmt_newline() {
    let data = &[b'\n'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\n\"");
}

#[test]
fn test_fmt_carriage_return() {
    let data = &[b'\r'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\r\"");
}

#[test]
fn test_fmt_tab() {
    let data = &[b'\t'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\t\"");
}

#[test]
fn test_fmt_backslash() {
    let data = &[b'\\'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\\\\"");
}

#[test]
fn test_fmt_quote() {
    let data = &[b'"'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\\"\"");
}

#[test]
fn test_fmt_null() {
    let data = &[b'\0'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\0\"");
}

#[test]
fn test_fmt_non_printable() {
    let data = &[b'\x01', b'\x02', b'\x7f'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\x01\\x02\\x7f\"");
}

#[test]
fn test_fmt_printable() {
    let data = &[b'a', b'b', b'c'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"abc\"");
}

