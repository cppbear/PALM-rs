// Answer 0

#[test]
fn test_from_maybe_shared_unchecked_valid_utf8() {
    let valid_utf8: &[u8] = b"valid";
    let header_value: HeaderValue;

    unsafe {
        header_value = HeaderValue::from_maybe_shared_unchecked(valid_utf8);
    }

    assert_eq!(header_value.as_bytes(), valid_utf8);
    assert_eq!(header_value.len(), valid_utf8.len());
    assert!(!header_value.is_empty());
}

#[test]
#[should_panic(expected = "HeaderValue::from_maybe_shared_unchecked() with invalid bytes")]
fn test_from_maybe_shared_unchecked_invalid_utf8() {
    let invalid_utf8: &[u8] = &[0xFF, 0xFE, 0xFD];

    unsafe {
        HeaderValue::from_maybe_shared_unchecked(invalid_utf8);
    }
}

#[test]
fn test_from_maybe_shared_unchecked_with_bytes() {
    use bytes::Bytes;

    let bytes = Bytes::from_static(b"shared bytes");
    let header_value: HeaderValue;

    unsafe {
        header_value = HeaderValue::from_maybe_shared_unchecked(bytes.clone());
    }

    assert_eq!(header_value.as_bytes(), bytes.as_ref());
    assert_eq!(header_value.len(), bytes.len());
    assert!(!header_value.is_empty());
}

