// Answer 0

#[test]
fn test_try_from_valid_bytes() {
    let valid_input: &[u8] = b"valid_authority";
    let result = Authority::try_from(valid_input.to_vec());
    assert!(result.is_ok());
}

#[test]
fn test_try_from_empty_bytes() {
    let empty_input: &[u8] = b"";
    let result = Authority::try_from(empty_input.to_vec());
    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_bytes() {
    let invalid_input: &[u8] = b"invalid_authority_with_invalid_chars!";
    let result = Authority::try_from(invalid_input.to_vec());
    assert!(result.is_err());
}

