// Answer 0

#[test]
fn test_fmt_non_sensitive_with_non_visible_ascii() {
    use std::fmt::Formatter;
    let header_value = HeaderValue {
        inner: Bytes::from(vec![0, 1, 2, 3, 4]),
        is_sensitive: false,
    };
    let mut formatter = Formatter::default();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_sensitive_with_leading_non_visible_ascii() {
    use std::fmt::Formatter;
    let header_value = HeaderValue {
        inner: Bytes::from(vec![0, 1, 3]),
        is_sensitive: false,
    };
    let mut formatter = Formatter::default();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_sensitive_with_trailing_non_visible_ascii() {
    use std::fmt::Formatter;
    let header_value = HeaderValue {
        inner: Bytes::from(vec![2, 3, 4]),
        is_sensitive: false,
    };
    let mut formatter = Formatter::default();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_sensitive_with_only_non_visible_ascii() {
    use std::fmt::Formatter;
    let header_value = HeaderValue {
        inner: Bytes::from(vec![0]),
        is_sensitive: false,
    };
    let mut formatter = Formatter::default();
    let _ = header_value.fmt(&mut formatter);
}

