// Answer 0

#[test]
fn test_parse_http_scheme() {
    let input: &[u8] = b"http://example.com"; // s.len() == 15, but parse tests only begin with 7
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Standard(Protocol::Http)));
}

#[test]
fn test_parse_https_scheme() {
    let input: &[u8] = b"https://example.com"; // s.len() == 16
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Standard(Protocol::Https)));
}

#[test]
fn test_parse_other_scheme() {
    let input: &[u8] = b"custom://example.com"; // s.len() == 18, valid scheme with custom prefix
    let result = Scheme2::<usize>::parse(input);
    assert!(matches!(result, Ok(Scheme2::Other(_))));
}

#[test]
fn test_parse_scheme_too_long() {
    let input: &[u8] = b"this_scheme_is_way_too_long_because_it_is_longer_than_sixty_four_characters://example.com"; // exceeds max scheme length
    let result = Scheme2::<usize>::parse(input);
    assert!(matches!(result, Err(InvalidUri(ErrorKind::SchemeTooLong))));
}

#[test]
fn test_parse_empty_scheme() {
    let input: &[u8] = b""; // empty input
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::None));
}

