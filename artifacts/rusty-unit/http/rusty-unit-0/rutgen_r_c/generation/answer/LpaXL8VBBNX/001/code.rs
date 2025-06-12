// Answer 0

#[test]
fn test_fmt_sensitive_case() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"example"),
        is_sensitive: true,
    };
    let mut buffer = String::new();
    let result = header_value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Sensitive");
}

#[test]
fn test_fmt_non_sensitive_with_visible_bytes() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello, World!"),
        is_sensitive: false,
    };
    let mut buffer = String::new();
    let result = header_value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"Hello, World!\"");
}

#[test]
fn test_fmt_non_sensitive_with_invisible_bytes() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello\x01World!"),
        is_sensitive: false,
    };
    let mut buffer = String::new();
    let result = header_value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"Hello\\x1World!\"");
}

#[test]
fn test_fmt_non_sensitive_with_double_quotes() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello\"World!"),
        is_sensitive: false,
    };
    let mut buffer = String::new();
    let result = header_value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"Hello\\\"World!\"");
}

#[test]
fn test_fmt_non_sensitive_empty_string() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    let mut buffer = String::new();
    let result = header_value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"\"");
}

