// Answer 0

#[test]
fn test_bytes_ref_debug_empty() {
    let data = b"";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""#);
}

#[test]
fn test_bytes_ref_debug_newline() {
    let data = b"\n";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\n""#);
}

#[test]
fn test_bytes_ref_debug_carriage_return() {
    let data = b"\r";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\r""#);
}

#[test]
fn test_bytes_ref_debug_tab() {
    let data = b"\t";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\t""#);
}

#[test]
fn test_bytes_ref_debug_backslash() {
    let data = b"\\";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\\""#);
}

#[test]
fn test_bytes_ref_debug_quote() {
    let data = b"\"";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""""#);
}

#[test]
fn test_bytes_ref_debug_ascii_printable() {
    let data = b"Hello World!";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"Hello World!""#);
}

#[test]
fn test_bytes_ref_debug_non_ascii() {
    let data = b"\x00\x01\x02\x03";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\x00\x01\x02\x03""#);
}

