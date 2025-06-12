// Answer 0

#[test]
fn test_header_value_fmt_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from("Hello, World!"),
        is_sensitive: false,
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", header_value));

    assert!(result.is_ok());
    assert_eq!(output, "\"Hello, World!\"");
}

#[test]
fn test_header_value_fmt_escape_characters() {
    let header_value = HeaderValue {
        inner: Bytes::from("Hello, \"World!\""),
        is_sensitive: false,
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", header_value));

    assert!(result.is_ok());
    assert_eq!(output, "\"Hello, \\\"World!\\\"\"");
}

#[test]
fn test_header_value_fmt_non_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from("Hello\x00World!"), // Contains null byte, non-visible ascii
        is_sensitive: false,
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", header_value));

    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x0\\x0World!\"");
}

#[test]
fn test_header_value_fmt_empty_string() {
    let header_value = HeaderValue {
        inner: Bytes::from(""),
        is_sensitive: false,
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", header_value));

    assert!(result.is_ok());
    assert_eq!(output, "\"\"");
}

#[test]
fn test_header_value_fmt_sensitive() {
    let header_value = HeaderValue {
        inner: Bytes::from("This should not be shown"),
        is_sensitive: true,
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", header_value));

    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

