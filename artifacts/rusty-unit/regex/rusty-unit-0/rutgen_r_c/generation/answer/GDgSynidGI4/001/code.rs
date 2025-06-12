// Answer 0

#[test]
fn test_match_end_with_valid_values() {
    let haystack = "Hello, world!";
    let valid_match = Match::new(haystack, 7, 12);
    assert_eq!(valid_match.end(), 12);
}

#[test]
fn test_match_end_with_zero_length_match() {
    let haystack = "Rust programming";
    let zero_length_match = Match::new(haystack, 4, 4);
    assert_eq!(zero_length_match.end(), 4);
}

#[test]
fn test_match_end_with_full_length_match() {
    let haystack = "Full match here!";
    let full_length_match = Match::new(haystack, 0, 16);
    assert_eq!(full_length_match.end(), 16);
}

#[test]
fn test_match_end_with_empty_string() {
    let haystack = "";
    let empty_match = Match::new(haystack, 0, 0);
    assert_eq!(empty_match.end(), 0);
}

