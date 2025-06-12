// Answer 0

#[test]
fn test_fmt_with_non_sensitive_ascii() {
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
        inner: Bytes::from("Sensitive data"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

#[test]
fn test_fmt_with_visible_and_invisible_bytes() {
    let value = HeaderValue {
        inner: Bytes::from("Visible\x00Invisible"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Visible\\x00Invisible\"");
}

#[test]
fn test_fmt_with_control_character() {
    let value = HeaderValue {
        inner: Bytes::from("Hello\x1fWorld"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x1fWorld\"");
}

#[test]
fn test_fmt_with_double_quotes() {
    let value = HeaderValue {
        inner: Bytes::from("Quote\"Inside"),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Quote\\\"Inside\"");
}

#[test]
fn test_fmt_with_only_invisible_bytes() {
    let value = HeaderValue {
        inner: Bytes::from(&[0, 1, 2, 3, 4]),
        is_sensitive: false,
    };
    let mut output = String::new();
    let result = value.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"\\x0\\x1\\x2\\x3\\x4\"");
}

