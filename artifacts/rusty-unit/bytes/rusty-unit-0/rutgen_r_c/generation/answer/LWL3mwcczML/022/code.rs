// Answer 0

#[test]
fn test_bytes_ref_debug_with_printable_characters() {
    let data = BytesRef(&[b'a', b'b', b'c']);
    let mut output = String::new();
    let result = data.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"abc\"");
}

#[test]
fn test_bytes_ref_debug_with_special_characters() {
    let data = BytesRef(&[b'\n', b'\r', b'\t', b'\\', b'"', b'\0']);
    let mut output = String::new();
    let result = data.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"\\n\\r\\t\\\\\\\"\\0\"");
}

#[test]
fn test_bytes_ref_debug_with_non_printable_characters() {
    let data = BytesRef(&[0x01, 0x02, 0x03, 0x1F, 0x80, 0xFF]);
    let mut output = String::new();
    let result = data.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"\\x01\\x02\\x03\\x1f\\x80\\xff\"");
}

#[test]
fn test_bytes_ref_debug_with_mixed_characters() {
    let data = BytesRef(&[b'a', b'b', b'\n', b'\t', 0x0F, 0x20, b'\\', 0x7F]);
    let mut output = String::new();
    let result = data.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "b\"ab\\n\\t\\x0f \\x7f\\\\\"");
}

