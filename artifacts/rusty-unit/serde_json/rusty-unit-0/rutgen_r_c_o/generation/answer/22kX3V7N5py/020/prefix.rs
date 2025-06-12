// Answer 0

#[test]
fn test_parse_any_signed_number_valid_negative() {
    let input = b"-1";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_valid_zero() {
    let input = b"0";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_valid_positive() {
    let input = b"1";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_edge_case_min() {
    let input = b"-2147483648";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_edge_case_max() {
    let input = b"2147483647";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_invalid_character() {
    let input = b"a";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_invalid_end() {
    let input = b"-";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_invalid_character_trailing() {
    let input = b"12a";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_empty_input() {
    let input = b"";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_whitespace_only() {
    let input = b"   ";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_invalid_negative() {
    let input = b"-b";
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_exceeding_max() {
    let input = b"9223372036854775808"; // beyond i64::MAX
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

#[test]
fn test_parse_any_signed_number_exceeding_min() {
    let input = b"-9223372036854775809"; // beyond i64::MIN
    let mut deserializer = Deserializer::from_slice(input);
    deserializer.parse_any_signed_number();
}

