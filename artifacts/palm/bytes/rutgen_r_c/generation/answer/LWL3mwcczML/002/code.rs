// Answer 0

#[test]
fn test_debug_fmt_empty() {
    let bytes_ref = BytesRef(&[]);
    let mut output = String::new();
    let result = bytes_ref.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"\"");
}

#[test]
fn test_debug_fmt_single_printable() {
    let bytes_ref = BytesRef(&[b'a']);
    let mut output = String::new();
    let result = bytes_ref.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\"");
}

#[test]
fn test_debug_fmt_with_escape_characters() {
    let bytes_ref = BytesRef(&[b'a', b'\n', b'\r', b'\t', b'\\', b'"']);
    let mut output = String::new();
    let result = bytes_ref.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\\n\\r\\t\\\\\\\"\"");
}

#[test]
fn test_debug_fmt_non_printable_characters() {
    let bytes_ref = BytesRef(&[0, 1, 2, 3, 4, 5, 0x10, 0x1F]);
    let mut output = String::new();
    let result = bytes_ref.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"\\0\\x01\\x02\\x03\\x04\\x05\\x10\\x1f\"");
}

#[test]
fn test_debug_fmt_with_multibyte_character() {
    let bytes_ref = BytesRef(&[b'\0', b'\xFF', b'\x7F']);
    let mut output = String::new();
    let result = bytes_ref.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"\\0\\xff\\x7f\"");
}

