// Answer 0

#[test]
fn test_parse_invalid_authority_with_invalid_percent() {
    let input: &[u8] = b"invalid%authority@[]";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_with_bracket_and_percent() {
    let input: &[u8] = b"[ipv6%address]:80";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_with_extra_colon() {
    let input: &[u8] = b"localhost:8080:3030";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_with_bracket_mismatch() {
    let input: &[u8] = b"[invalid:authority";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_with_empty_percentage() {
    let input: &[u8] = b"empty%@host";
    let _ = Authority::parse(input);
}

