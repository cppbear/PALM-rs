// Answer 0

#[test]
fn test_debug_bytesref_empty() {
    let bytes_ref = BytesRef(&[]);
    let mut output = vec![];
    let result = bytes_ref.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\"");
}

#[test]
fn test_debug_bytesref_printable() {
    let bytes_ref = BytesRef(b"Hello, World!");
    let mut output = vec![];
    let result = bytes_ref.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"Hello, World!\"");
}

#[test]
fn test_debug_bytesref_escaped_characters() {
    let bytes_ref = BytesRef(b"Line1\nLine2\tTab and a \\ backslash");
    let mut output = vec![];
    let result = bytes_ref.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"Line1\\nLine2\\tTab and a \\\\ backslash\"");
}

#[test]
fn test_debug_bytesref_non_printable() {
    let bytes_ref = BytesRef(b"\x01\x02\x03\x08\x10");
    let mut output = vec![];
    let result = bytes_ref.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\x01\\x02\\x03\\x08\\x10\"");
}

#[test]
fn test_debug_bytesref_mixed() {
    let bytes_ref = BytesRef(b"Hello\n\x00World\x7F");
    let mut output = vec![];
    let result = bytes_ref.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"Hello\\n\\0World\\x7f\"");
}

#[should_panic]
fn test_debug_bytesref_malformed_utf8() {
    let bytes_ref = BytesRef(b"\xFF\xFE\xFD");
    let mut output = vec![];
    let _ = bytes_ref.fmt(&mut Formatter::new(&mut output));
}

