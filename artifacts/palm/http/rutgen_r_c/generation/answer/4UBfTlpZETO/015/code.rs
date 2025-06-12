// Answer 0

fn test_parse_invalid_uri_char() {
    let invalid_uri = b"\x00invalid_uri"; // Contains null byte, which is invalid
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

fn test_parse_invalid_uri_char_with_percent() {
    let invalid_uri = b"invalid%uri"; // Contains valid percent but invalid context
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

fn test_parse_invalid_authority_unmatched_brackets() {
    let invalid_uri = b"[invalid_uri"; // Missing closing bracket
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidAuthority);
    }
}

fn test_parse_invalid_authority_extra_colons() {
    let invalid_uri = b"localhost:8080:3030"; // Too many colons
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidAuthority);
    }
}

fn test_parse_invalid_authority_empty_post_at_sign() {
    let invalid_uri = b"username:password@"; // Nothing after '@'
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidAuthority);
    }
}

fn test_parse_invalid_authority_starting_with_percent() {
    let invalid_uri = b"%invalid_uri"; // Starts with percent
    let result = Authority::parse(invalid_uri);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidAuthority);
    }
}

fn test_parse_valid_authority() {
    let valid_uri = b"localhost:8080"; // A valid case for contrast
    let result = Authority::parse(valid_uri);
    assert!(result.is_ok());
}

