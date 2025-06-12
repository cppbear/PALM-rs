// Answer 0

#[test]
fn test_is_match_valid() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"I categorically deny having triskaidekaphobia.";
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_invalid() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"This text does not have a longword.";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_empty() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_no_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"short";
    assert!(!regex.is_match(text));
} 

#[test]
fn test_is_match_boundary() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"1234567890123"; // exactly 13 characters
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_non_ascii() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"abcde12345xyz"; // contains 13 characters with non-word
    assert!(!regex.is_match(text));
}

