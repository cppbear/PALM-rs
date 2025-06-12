// Answer 0

#[test]
fn test_is_match_with_exact_word_length() {
    use regex::bytes::Regex;
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"I categorically deny having triskaidekaphobia.";
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_with_no_matching_word() {
    use regex::bytes::Regex;
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"This is a test without long words.";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_with_empty_string() {
    use regex::bytes::Regex;
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_with_boundary_word_length() {
    use regex::bytes::Regex;
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"1234567890123"; // Exactly 13 characters
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_with_one_char_word() {
    use regex::bytes::Regex;
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"a"; // Less than 13 characters
    assert!(!regex.is_match(text));
}

