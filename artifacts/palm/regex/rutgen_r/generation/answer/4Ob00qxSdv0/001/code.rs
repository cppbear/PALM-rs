// Answer 0

#[test]
fn test_is_match_valid_pattern() {
    use regex::bytes::Regex;
    
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"I categorically deny having triskaidekaphobia.";
    
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_no_match() {
    use regex::bytes::Regex;
    
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"Short words only.";
    
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_empty_input() {
    use regex::bytes::Regex;
    
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"";
    
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_exact_word_length() {
    use regex::bytes::Regex;
    
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"Triskaidekaphobia";
    
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_word_with_special_characters() {
    use regex::bytes::Regex;

    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"Thirteen characters! Not a match.";
    
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_multiple_words() {
    use regex::bytes::Regex;

    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"This is a triskaidekaphobia test.";
    
    assert!(regex.is_match(text));
}

