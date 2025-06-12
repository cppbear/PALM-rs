// Answer 0

#[test]
fn test_parse_index_with_plus_one() {
    let input = "+1";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_abc() {
    let input = "+abc";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_empty() {
    let input = "+";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_zero() {
    let input = "+0";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_large_number() {
    let input = "+12345";
    let result = parse_index(input);
}

