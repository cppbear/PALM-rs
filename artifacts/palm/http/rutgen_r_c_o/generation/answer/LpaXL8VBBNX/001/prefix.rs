// Answer 0

#[test]
fn test_fmt_sensitive() {
    let sensitive_value = HeaderValue {
        inner: Bytes::from_static("Sensitive Content"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", sensitive_value);
}

#[test]
fn test_fmt_sensitive_empty_string() {
    let sensitive_value = HeaderValue {
        inner: Bytes::from_static(""),
        is_sensitive: true,
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", sensitive_value);
}

#[test]
fn test_fmt_sensitive_with_special_chars() {
    let sensitive_value = HeaderValue {
        inner: Bytes::from_static("Sensitive Content with special char: \" and \x00"),
        is_sensitive: true,
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", sensitive_value);
}

#[test]
fn test_fmt_sensitive_long_content() {
    let sensitive_value = HeaderValue {
        inner: Bytes::from_static(
            "This is a long sensitive content that should not panic when formatted.",
        ),
        is_sensitive: true,
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", sensitive_value);
}

