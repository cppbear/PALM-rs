// Answer 0

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[somehost.com:80"; // Unmatched '['
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_extra_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // Multiple colons
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_empty() {
    let input: &[u8] = b""; // Empty string
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_invalid_character() {
    let input: &[u8] = b"localhost:80#"; // Invalid character '#'
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_percent_after_userinfo() {
    let input: &[u8] = b"user:pass@host%20:path"; // Percent after userinfo
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_trailing_at_sign() {
    let input: &[u8] = b"@"; // Nothing after '@'
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_start_double_bracket() {
    let input: &[u8] = b"[host.com::80]"; // Double colons
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_exceeding_max_colons() {
    let input: &[u8] = b"part1:part2:part3:part4:part5:part6:part7:part8:part9"; // Exceeds max colons
    Authority::parse(input);
}

