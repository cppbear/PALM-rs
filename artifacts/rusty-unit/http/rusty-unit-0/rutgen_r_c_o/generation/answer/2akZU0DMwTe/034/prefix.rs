// Answer 0

#[test]
fn test_parse_scheme_none_length_4() {
    let input: &[u8] = b"abcd";
    let _ = Scheme2::parse(input);
}

#[test]
fn test_parse_scheme_none_length_5() {
    let input: &[u8] = b"abcde";
    let _ = Scheme2::parse(input);
}

#[test]
fn test_parse_scheme_none_length_6() {
    let input: &[u8] = b"abcdef";
    let _ = Scheme2::parse(input);
}

