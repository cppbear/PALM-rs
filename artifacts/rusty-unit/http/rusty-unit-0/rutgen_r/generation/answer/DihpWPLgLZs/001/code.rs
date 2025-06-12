// Answer 0

#[test]
fn test_new_valid_utf8() {
    let src: &[u8] = b"valid utf8";
    let result = new(src);
    assert!(result.is_ok());
    if let Ok(extension) = result {
        assert_eq!(extension.0.as_ref(), &src[..]);
    }
}

#[test]
fn test_new_empty_input() {
    let src: &[u8] = b"";
    let result = new(src);
    assert!(result.is_ok());
    if let Ok(extension) = result {
        assert!(extension.0.is_empty());
    }
}

#[should_panic]
fn test_new_invalid_utf8() {
    let src: &[u8] = &[0xFF];
    let _ = new(src);
}

#[should_panic]
fn test_new_non_utf8_boundary() {
    let src: &[u8] = &[0x80]; // Invalid starting byte for UTF-8
    let _ = new(src);
}

