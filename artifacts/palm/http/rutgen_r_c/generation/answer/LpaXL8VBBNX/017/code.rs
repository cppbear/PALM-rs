// Answer 0

#[test]
fn test_fmt_non_sensitive_normal_case() {
    let value = HeaderValue {
        inner: Bytes::from("Hello, World!"),
        is_sensitive: false,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"Hello, World!\"");
}

#[test]
fn test_fmt_non_sensitive_with_quotes() {
    let value = HeaderValue {
        inner: Bytes::from("Hello, \"World!\""),
        is_sensitive: false,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"Hello, \\\"World!\\\"\"");
}

#[test]
fn test_fmt_non_sensitive_with_non_visible_ascii() {
    let value = HeaderValue {
        inner: Bytes::from("Hello\x01World"),
        is_sensitive: false,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"Hello\\x1fWorld\"");
}

#[test]
fn test_fmt_non_sensitive_empty() {
    let value = HeaderValue {
        inner: Bytes::from(""),
        is_sensitive: false,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"\"");
}

#[test]
fn test_fmt_sensitive() {
    let value = HeaderValue {
        inner: Bytes::from("Sensitive Data"),
        is_sensitive: true,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "Sensitive");
}

#[test]
fn test_fmt_non_sensitive_special_cases() {
    let value = HeaderValue {
        inner: Bytes::from("Tab\tCharacter"), 
        is_sensitive: false,
    };
    let mut buf = String::new();
    let result = value.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "\"Tab\\tCharacter\"");
}

