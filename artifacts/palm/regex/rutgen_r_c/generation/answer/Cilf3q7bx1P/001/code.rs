// Answer 0

#[test]
fn test_is_match_basic() {
    let regex = Regex::new(r"\d+").unwrap();
    assert!(regex.is_match("12345"));
    assert!(!regex.is_match("abc"));
}

#[test]
fn test_is_match_empty_string() {
    let regex = Regex::new(r"").unwrap();
    assert!(regex.is_match(""));  // An empty pattern matches an empty string
}

#[test]
fn test_is_match_no_match() {
    let regex = Regex::new(r"abc").unwrap();
    assert!(!regex.is_match("def"));
}

#[test]
fn test_is_match_with_special_characters() {
    let regex = Regex::new(r"[!@#$%^&*()_+]").unwrap();
    assert!(regex.is_match("Hello!"));
    assert!(!regex.is_match("Hello"));
}

#[test]
fn test_is_match_unicode_characters() {
    let regex = Regex::new(r"\w{2}").unwrap();
    assert!(regex.is_match("汉字"));
    assert!(!regex.is_match(" "));
}

#[test]
fn test_is_match_long_pattern() {
    let regex = Regex::new(r"\w{13}").unwrap();
    assert!(regex.is_match("triskaidekaphobia"));
    assert!(!regex.is_match("short"));
}

#[test]
fn test_is_match_with_boundary_conditions() {
    let regex = Regex::new(r"^[A-Z].*$").unwrap();
    assert!(regex.is_match("Hello World"));
    assert!(!regex.is_match("hello world"));
}

#[test]
fn test_is_match_with_numerics() {
    let regex = Regex::new(r"^\d+$").unwrap();
    assert!(regex.is_match("123456"));
    assert!(!regex.is_match("123abc"));
}

#[test]
fn test_is_match_multiple_matches() {
    let regex = Regex::new(r"\d+").unwrap();
    assert!(regex.is_match("There are 123 and 456."));
    assert!(!regex.is_match("No numbers here."));
}

