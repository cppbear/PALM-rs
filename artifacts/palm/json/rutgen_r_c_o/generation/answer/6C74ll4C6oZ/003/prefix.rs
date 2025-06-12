// Answer 0

#[test]
fn test_parse_index_valid_zero() {
    let input = "0";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_valid_positive_integer() {
    let input = "1";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_valid_small_integer() {
    let input = "2";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_invalid_zero_leading() {
    let input = "00";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_invalid_positive_leading() {
    let input = "+1";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_invalid_negative_integer() {
    let input = "-1"; 
    let result = parse_index(input);
}

