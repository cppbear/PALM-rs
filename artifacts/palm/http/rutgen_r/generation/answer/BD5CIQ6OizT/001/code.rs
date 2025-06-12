// Answer 0

#[test]
fn test_from_bytes_with_empty_input() {
    let hdr: &[u8] = &[];
    let result: Result<&HdrName<'_>, InvalidHeaderName> = from_bytes(hdr, |name| name);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_with_invalid_chars() {
    let hdr: &[u8] = b"Invalid Header Name!";
    let result: Result<&HdrName<'_>, InvalidHeaderName> = from_bytes(hdr, |name| name);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_with_valid_input() {
    let hdr: &[u8] = b"Valid-Header-Name";
    let result: Result<&HdrName<'_>, InvalidHeaderName> = from_bytes(hdr, |name| name);
    assert!(result.is_ok());
}

