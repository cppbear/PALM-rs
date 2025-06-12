// Answer 0

#[test]
fn test_try_from_valid_bytes() {
    let input: &[u8] = b"example.com";
    let result = Authority::try_from(input.to_vec());

    assert!(result.is_ok());
    let authority = result.unwrap();
    assert_eq!(authority.data.bytes.as_ref(), input);
}

#[test]
fn test_try_from_empty_bytes() {
    let input: &[u8] = b"";
    let result = Authority::try_from(input.to_vec());

    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_bytes() {
    let input: &[u8] = b"invalid_uri_with_space ";
    let result = Authority::try_from(input.to_vec());

    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_from_boundary_condition() {
    let input: &[u8] = b"invalid\x00uri";
    let _ = Authority::try_from(input.to_vec()).unwrap();
}

