// Answer 0

#[test]
fn test_parse_exact_scheme_too_long() {
    let input: &[u8] = b"valid_scheme_with_length_greater_than_sixty_four_bytes_12345";
    let result = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_another_long_scheme() {
    let input: &[u8] = b"this_scheme_is_way_too_long_for_the_scheme_limit_in_http_123";
    let result = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_long_scheme_with_valid_chars() {
    let input: &[u8] = b"anotherValidSchemeThatIsTooLongToBeConsideredValidForHTTPProtocol123";
    let result = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_maximum_length_scheme() {
    let input: &[u8] = b"validschemevalidschemevalidschemevalidschemevalidschemevalidschem";
    let result = Scheme2::<()>::parse_exact(input);
}

