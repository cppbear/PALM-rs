// Answer 0

#[test]
fn test_is_match_simple_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"I categorically deny having triskaidekaphobia.";
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_no_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"This is a short text.";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_empty_text() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_partial_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"His triskaidekaphob ia is well known.";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_exact_edge_case() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"abcdefghijklm \n noprint";
    assert!(regex.is_match(text));
}

