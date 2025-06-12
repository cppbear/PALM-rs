// Answer 0

#[test]
fn test_create_authority_empty_bytes() {
    let result = create_authority(b"", |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_invalid_utf8_bytes() {
    let result = create_authority(b"\xFF\xFE\xFD", |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_partial_valid_utf8() {
    let result = create_authority(b"valid:authority\xFF", |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_valid_input() {
    let result = create_authority(b"valid:authority", |b| Bytes::from(b));
    assert!(result.is_ok());
}

#[test]
fn test_create_authority_non_empty_trailing_invalid() {
    let result = create_authority(b"valid:authority\x00", |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_large_valid_input() {
    let long_input = b"valid:authority_with_a_really_long_string_of_chars_1234567890";
    let result = create_authority(long_input, |b| Bytes::from(b));
    assert!(result.is_ok());
}

