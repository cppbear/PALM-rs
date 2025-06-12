// Answer 0

#[test]
fn test_fmt_sensitive() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"Test"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

#[test]
fn test_fmt_not_sensitive_empty_bytes() {
    let value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"\"");
}

#[test]
fn test_fmt_not_sensitive_visible_bytes() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"Visible"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Visible\"");
}

#[test]
fn test_fmt_not_sensitive_non_visible_bytes() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"Non-visible\x00character"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Non-visible\\x0acharacter\"");
}

#[test]
fn test_fmt_not_sensitive_escapes() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"Escape \" character"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Escape \\\" character\"");
}

