// Answer 0

#[test]
fn test_parse_short_length() {
    let input = b"foo:";
    let result = Scheme2::parse(input);
}

#[test]
fn test_parse_invalid_character() {
    let input = b"invalid#scheme:";
    let result = Scheme2::parse(input);
}

#[test]
fn test_parse_valid_scheme_length_5() {
    let input = b"abcde:";
    let result = Scheme2::parse(input);
}

#[test]
fn test_parse_valid_scheme_length_6() {
    let input = b"123456:";
    let result = Scheme2::parse(input);
}

