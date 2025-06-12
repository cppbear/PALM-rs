// Answer 0

#[test]
fn test_fmt_with_newline() {
    let data = BytesRef(b"Hello\nWorld");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\nWorld""#);
}

#[test]
fn test_fmt_with_carriage_return() {
    let data = BytesRef(b"Hello\rWorld");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\rWorld""#);
}

#[test]
fn test_fmt_with_multiple_special_chars() {
    let data = BytesRef(b"Hello\nWorld\r\n\t");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\nWorld\r\n\t""#);
}

#[test]
fn test_fmt_with_tabs_not_triggering() {
    let data = BytesRef(b"Hello\tWorld");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\tWorld""#);
}

#[test]
fn test_fmt_with_escape_sequences() {
    let data = BytesRef(b"Hello\\World\"Test");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\\World\"Test""#);
}

#[test]
fn test_fmt_with_null_byte() {
    let data = BytesRef(b"Hello\0World");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\0World""#);
}

#[test]
fn test_fmt_with_non_printable_characters() {
    let data = BytesRef(b"Hello\x01World");
    let result = format!("{:?}", data);
    assert_eq!(result, r#"b"Hello\x01World""#);
}

