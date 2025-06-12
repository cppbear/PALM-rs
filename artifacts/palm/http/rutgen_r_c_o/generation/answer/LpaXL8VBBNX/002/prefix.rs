// Answer 0

#[test]
fn test_fmt_sensitive() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Test"),
        is_sensitive: true,
    };
    let mut formatter = String::new();
    let result = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_empty() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    let mut formatter = String::new();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Visible"),
        is_sensitive: false,
    };
    let mut formatter = String::new();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"\x00\x01\x02\x03"),
        is_sensitive: false,
    };
    let mut formatter = String::new();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_double_quotes() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Test with \"quotes\""),
        is_sensitive: false,
    };
    let mut formatter = String::new();
    let _ = header_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_large_size() {
    let large_bytes = vec![0; (u16::MAX as usize + 1)];
    let header_value = HeaderValue {
        inner: Bytes::from(large_bytes),
        is_sensitive: false,
    };
    let mut formatter = String::new();
    let result = header_value.fmt(&mut formatter);
}

