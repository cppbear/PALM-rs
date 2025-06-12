// Answer 0

#[test]
fn test_parse_non_empty_valid_input() {
    let input: &[u8] = b"example.com:80";
    let result = parse_non_empty(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_non_empty_valid_input_with_no_port() {
    let input: &[u8] = b"example.com";
    let result = parse_non_empty(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_non_empty_edge_case_single_character() {
    let input: &[u8] = b"a";
    let result = parse_non_empty(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_non_empty_edge_case_with_special_characters() {
    let input: &[u8] = b"example.com/path?query=value#fragment";
    let result = parse_non_empty(input);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_non_empty_empty_input() {
    let input: &[u8] = b"";
    let _ = parse_non_empty(input);
}

