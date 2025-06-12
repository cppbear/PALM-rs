// Answer 0

#[test]
fn test_debug_bytes_ref_with_newline() {
    let data: &[u8] = &[b'\n'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b\"\n""#);
}

#[test]
fn test_debug_bytes_ref_with_carriage_return() {
    let data: &[u8] = &[b'\r'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b\"\r""#);
}

#[test]
fn test_debug_bytes_ref_with_tab() {
    let data: &[u8] = &[b'\t'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b\"\t""#);
}

#[test]
fn test_debug_bytes_ref_with_backslash() {
    let data: &[u8] = &[b'\\'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b"\\\""#);
}

#[test]
fn test_debug_bytes_ref_with_quote() {
    let data: &[u8] = &[b'"'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b"\"""#);
}

#[test]
fn test_debug_bytes_ref_with_null() {
    let data: &[u8] = &[b'\0'];
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b"\\0""#);
}

#[test]
fn test_debug_bytes_ref_with_non_printable() {
    let data: &[u8] = &[0x01]; // non-printable character
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), r#""b"\\x01""#);
}

