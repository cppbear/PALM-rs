// Answer 0

#[test]
fn test_as_str_custom_valid() {
    let valid_bytes = Bytes::from_static(b"valid-header");
    let custom_inner = Custom(ByteStr { bytes: valid_bytes });
    let header_name = HeaderName { inner: Repr::Custom(custom_inner) };

    let _ = header_name.as_str();
}

#[test]
fn test_as_str_custom_min_length() {
    let min_length_bytes = Bytes::from_static(b"a");
    let custom_inner = Custom(ByteStr { bytes: min_length_bytes });
    let header_name = HeaderName { inner: Repr::Custom(custom_inner) };

    let _ = header_name.as_str();
}

#[test]
fn test_as_str_custom_max_length() {
    let max_length_bytes = Bytes::from_static(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let custom_inner = Custom(ByteStr { bytes: max_length_bytes });
    let header_name = HeaderName { inner: Repr::Custom(custom_inner) };

    let _ = header_name.as_str();
}

#[test]
fn test_as_str_custom_empty() {
    let empty_bytes = Bytes::from_static(b"");
    let custom_inner = Custom(ByteStr { bytes: empty_bytes });
    let header_name = HeaderName { inner: Repr::Custom(custom_inner) };

    let _ = header_name.as_str();
}

