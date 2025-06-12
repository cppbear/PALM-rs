// Answer 0

#[test]
fn test_fmt_non_sensitive() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello, World!"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello, World!\"");
}

#[test]
fn test_fmt_sensitive() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Sensitive Data"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

#[test]
fn test_fmt_with_special_characters() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello, \"World\"!"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello, \\\"World\\\"!\"");
}

#[test]
fn test_fmt_with_non_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Hello\x00World"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x0World\"");
}

#[test]
fn test_fmt_empty_string() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"\"");
}

