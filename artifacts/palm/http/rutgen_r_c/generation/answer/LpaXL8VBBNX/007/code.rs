// Answer 0

#[test]
fn test_fmt_normal_case() {
    let value = HeaderValue {
        inner: Bytes::from("Hello, World!"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello, World!\"");
}

#[test]
fn test_fmt_with_sensitive() {
    let value = HeaderValue {
        inner: Bytes::from("Sensitive Data"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

#[test]
fn test_fmt_with_non_visible_ascii() {
    let value = HeaderValue {
        inner: Bytes::from("Hello\x00World!"), // Contains non-visible ASCII
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x:0World!\"");
}

#[test]
fn test_fmt_with_quotes() {
    let value = HeaderValue {
        inner: Bytes::from("Hello\"World!"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\\"World!\"");
}

