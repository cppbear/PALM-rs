// Answer 0

#[test]
fn test_find_valid_match() {
    let text = b"I categorically deny having triskaidekaphobia.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text).unwrap();
    assert_eq!((result.start, result.end), (2, 15));
}

#[test]
fn test_find_no_match() {
    let text = b"Short words.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_empty_text() {
    let text: &[u8] = b"";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_match_at_start() {
    let text = b"Unbelievably";
    let regex = Regex::new(r"\b\w{12}\b").unwrap();
    let result = regex.find(text).unwrap();
    assert_eq!((result.start, result.end), (0, 12));
}

#[test]
fn test_find_match_at_end() {
    let text = b"Extraordinariness";
    let regex = Regex::new(r"\b\w{15}\b").unwrap();
    let result = regex.find(text).unwrap();
    assert_eq!((result.start, result.end), (0, 15));
}

