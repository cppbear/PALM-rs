// Answer 0

#[test]
fn test_find_match_exists() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = b"I categorically deny having triskaidekaphobia.";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 2);
    assert_eq!(mat.end, 15);
}

#[test]
fn test_find_no_match() {
    let regex = Regex::new(r"\b\w{20}\b").unwrap();
    let text = b"No matches here.";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_empty_string() {
    let regex = Regex::new(r"\b\w{3}\b").unwrap();
    let text = b"";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_with_special_characters() {
    let regex = Regex::new(r"\b\w+\b").unwrap();
    let text = b"What about $pecial characters?";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 0);
    assert_eq!(mat.end, 4);
}

