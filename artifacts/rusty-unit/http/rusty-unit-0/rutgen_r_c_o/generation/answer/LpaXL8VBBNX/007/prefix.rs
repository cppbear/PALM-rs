// Answer 0

#[test]
fn test_fmt_with_visible_ascii() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"visible"),
        is_sensitive: false,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_special_characters() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"visible\x01\x02\x03"),
        is_sensitive: false,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_escaped_quote() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"visible\"text"),
        is_sensitive: false,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_only_special_characters() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"\x01\x02\x03"),
        is_sensitive: false,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_empty_string() {
    let value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_sensitive_value() {
    let value = HeaderValue {
        inner: Bytes::from_static(b"visible"),
        is_sensitive: true,
    };
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

