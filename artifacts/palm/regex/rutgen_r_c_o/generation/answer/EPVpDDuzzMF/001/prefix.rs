// Answer 0

#[test]
fn test_is_match_at_empty_string() {
    let regex = Regex::new(r"abc").unwrap();
    let result = regex.is_match_at("", 0);
}

#[test]
fn test_is_match_at_full_string() {
    let regex = Regex::new(r"abc").unwrap();
    let result = regex.is_match_at("abc", 0);
}

#[test]
fn test_is_match_at_partial_start() {
    let regex = Regex::new(r"abc").unwrap();
    let result = regex.is_match_at("abc", 1);
}

#[test]
fn test_is_match_at_exact_end() {
    let regex = Regex::new(r"abc").unwrap();
    let result = regex.is_match_at("abc", 3);
}

#[test]
fn test_is_match_at_out_of_bounds() {
    let regex = Regex::new(r"abc").unwrap();
    let result = regex.is_match_at("abc", 4);
}

#[test]
fn test_is_match_at_single_character() {
    let regex = Regex::new(r"a").unwrap();
    let result = regex.is_match_at("a", 0);
}

#[test]
fn test_is_match_at_single_character_out_of_bounds() {
    let regex = Regex::new(r"a").unwrap();
    let result = regex.is_match_at("a", 1);
}

#[test]
fn test_is_match_at_multiline_string() {
    let regex = Regex::new(r"[0-9]+").unwrap();
    let result = regex.is_match_at("abc\n123", 0);
}

#[test]
fn test_is_match_at_multiline_partial_start() {
    let regex = Regex::new(r"[0-9]+").unwrap();
    let result = regex.is_match_at("abc\n123", 5);
}

#[test]
fn test_is_match_at_multiline_out_of_bounds() {
    let regex = Regex::new(r"[0-9]+").unwrap();
    let result = regex.is_match_at("abc\n123", 9);
}

#[test]
fn test_is_match_at_multiline_beyond_end() {
    let regex = Regex::new(r"[0-9]+").unwrap();
    let result = regex.is_match_at("abc\n123", 10);
}

