// Answer 0

#[test]
#[should_panic]
fn test_parse_invalid_authority_too_many_colons() {
    let invalid_authority = b"localhost:8080:3030"; // Too many colons
    let result = parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_unmatched_brackets() {
    let invalid_authority = b"[Invalid:Authority"; // Unmatched start bracket
    let result = parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_unmatched_end_bracket() {
    let invalid_authority = b"Invalid:Authority]"; // Unmatched end bracket
    let result = parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_percent_in_hostname() {
    let invalid_authority = b"Invalid%Host"; // Percent character in hostname
    let result = parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_at_sign_without_username() {
    let invalid_authority = b"@something"; // At sign without a preceding username
    let result = parse(invalid_authority);
    assert!(result.is_err());
}

