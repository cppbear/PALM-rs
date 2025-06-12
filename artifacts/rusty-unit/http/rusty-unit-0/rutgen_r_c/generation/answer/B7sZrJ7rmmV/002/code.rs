// Answer 0

#[test]
fn test_from_maybe_shared_unchecked_valid_utf8() {
    let valid_bytes: &[u8] = b"valid utf8";
    unsafe {
        let header_value = HeaderValue::from_maybe_shared_unchecked(valid_bytes);
        assert_eq!(header_value.as_bytes(), valid_bytes);
        assert!(!header_value.is_sensitive());
    }
}

#[test]
#[should_panic]
fn test_from_maybe_shared_unchecked_invalid_utf8() {
    let invalid_bytes: &[u8] = &[0, 159, 146, 150]; // Invalid UTF-8
    unsafe {
        HeaderValue::from_maybe_shared_unchecked(invalid_bytes);
    }
}

#[test]
fn test_from_maybe_shared_unchecked_empty() {
    let empty_bytes: &[u8] = b"";
    unsafe {
        let header_value = HeaderValue::from_maybe_shared_unchecked(empty_bytes);
        assert_eq!(header_value.len(), 0);
        assert!(header_value.is_empty());
    }
}

#[test]
fn test_from_maybe_shared_unchecked_bytes_owned() {
    let owned_bytes = Bytes::from_static(b"owned bytes");
    unsafe {
        let header_value = HeaderValue::from_maybe_shared_unchecked(owned_bytes);
        assert_eq!(header_value.as_bytes(), b"owned bytes");
        assert!(!header_value.is_sensitive());
    }
}

