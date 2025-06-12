// Answer 0

#[test]
fn test_fmt_valid_standard_header() {
    let header_name = HeaderName::from_static("accept");
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_custom_header_short() {
    let custom_bytes = ByteStr::from_bytes(b"custom-header").unwrap();
    let header_name = HeaderName { inner: Repr::Custom(Custom(custom_bytes)) };
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_custom_header_long() {
    let long_custom_bytes = ByteStr::from_bytes(b"this-is-a-very-long-custom-header-name").unwrap();
    let header_name = HeaderName { inner: Repr::Custom(Custom(long_custom_bytes)) };
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_invalid_static_header() {
    let header_name = HeaderName::from_static("invalid-header-name-which-is-too-long");
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_upper_bound_case() {
    let max_length_bytes = ByteStr::from_bytes(&[b'A'; 256]).unwrap();
    let header_name = HeaderName { inner: Repr::Custom(Custom(max_length_bytes)) };
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_lower_bound_case() {
    let min_length_bytes = ByteStr::from_bytes(b"A").unwrap();
    let header_name = HeaderName { inner: Repr::Custom(Custom(min_length_bytes)) };
    let mut formatter = fmt::Formatter::new();
    header_name.fmt(&mut formatter);
}

