// Answer 0

#[derive(Debug)]
struct RegexWrapper(regex::Regex);

impl RegexWrapper {
    fn new(pattern: &str) -> Self {
        RegexWrapper(regex::Regex::new(pattern).unwrap())
    }

    fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.0.is_match_at(text.as_bytes(), start)
    }
}

#[test]
fn test_is_match_at_matches_at_start() {
    let regex = RegexWrapper::new("hello");
    assert!(regex.is_match_at("hello world", 0));
}

#[test]
fn test_is_match_at_does_not_match_at_start() {
    let regex = RegexWrapper::new("world");
    assert!(!regex.is_match_at("hello world", 0));
}

#[test]
fn test_is_match_at_matches_in_middle() {
    let regex = RegexWrapper::new("world");
    assert!(regex.is_match_at("hello world", 6));
}

#[test]
fn test_is_match_at_does_not_match_out_of_bounds() {
    let regex = RegexWrapper::new("hello");
    assert!(!regex.is_match_at("hello", 5));
}

#[test]
fn test_is_match_at_empty_string() {
    let regex = RegexWrapper::new("");
    assert!(regex.is_match_at("hello", 0));
}

