// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    use crate::header::name::StandardHeader;
    use crate::header::name::{HdrName, InvalidHeaderName};

    let result = HdrName::from_bytes(b"accept", |hdr| {
        assert_eq!(hdr.inner, Repr::Standard(StandardHeader::Accept));
    });

    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header() {
    use crate::header::name::HdrName;
    use crate::header::name::InvalidHeaderName;

    let result = HdrName::from_bytes(b"x-custom-header", |hdr| {
        assert!(matches!(hdr.inner, Repr::Custom(_)));
    });

    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_empty() {
    use crate::header::name::{HdrName, InvalidHeaderName};

    let result = HdrName::from_bytes(b"", |_| {});

    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_character() {
    use crate::header::name::{HdrName, InvalidHeaderName};

    let result = HdrName::from_bytes(b"invalid\x00header", |_| {});

    assert!(result.is_err());
}

#[test]
fn test_from_bytes_overflow_length() {
    use crate::header::name::{HdrName, InvalidHeaderName};

    let hdr = b"this_is_a_custom_header_string_length_exceeding_the_limit";
    let result = HdrName::from_bytes(hdr, |_| {});

    assert!(result.is_ok());
}

