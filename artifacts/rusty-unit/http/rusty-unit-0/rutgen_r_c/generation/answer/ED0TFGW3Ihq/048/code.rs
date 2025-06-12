// Answer 0

#[test]
fn test_parse_exact_scheme_too_long() {
    let long_scheme = b"this_scheme_is_way_longer_than_the_maximum_allowed_length_of_sixty_four_bytes";
    let result = Scheme2::<()>::parse_exact(long_scheme);
    assert_eq!(result, Err(InvalidUri(ErrorKind::SchemeTooLong)));
}

