// Answer 0

#[test]
fn test_parse_exceed_max_colons() {
    let input = b"localhost:8080:3030:4040:5050:6060:7070:8080";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_uri_with_one_colon() {
    let input = b"localhost:8080";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_uri_with_no_colons() {
    let input = b"localhost";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_uri_with_special_character_at_end() {
    let input = b"localhost:8080@";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_uri_with_percent_character() {
    let input = b"localhost%20:8080";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_uri_with_single_bracket() {
    let input = b"[::1";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_uri_with_closing_bracket() {
    let input = b"localhost:8080]";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_ipv6_uri() {
    let input = b"[2001:db8::1]:80";
    let result = Authority::parse(input);
}

