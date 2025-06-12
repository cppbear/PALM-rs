// Answer 0

#[test]
fn test_parse_scheme_too_long() {
    let input: &[u8] = b"scheme_that_is_way_too_long_for_a_valid_uri_scheme_with_a_colon_and_no_following_slash:";
    let result = Scheme2::parse(input);
}

#[test]
fn test_parse_scheme_no_slash() {
    let input: &[u8] = b"invalid_scheme_with_colon:and_no_slash_after";
    let result = Scheme2::parse(input);
}

