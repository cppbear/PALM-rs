// Answer 0

#[test]
fn test_parse_non_empty_empty_input() {
    let input: &[u8] = &[]; 
    let result = Authority::parse_non_empty(input);
}

#[test]
#[should_panic]
fn test_parse_non_empty_empty_input_panic() {
    let input: &[u8] = &[];
    let result = Authority::parse_non_empty(input);
}

