// Answer 0

#[test]
fn test_find_match_success() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "I categorically deny having triskaidekaphobia.";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 2);
    assert_eq!(mat.end, 15);
}

#[test]
fn test_find_match_failure() {
    let regex = Regex::new(r"\b\w{14}\b").unwrap();
    let text = "No match in this string.";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_empty_string() {
    let regex = Regex::new(r"\b\w{3}\b").unwrap();
    let text = "";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_no_word_characters() {
    let regex = Regex::new(r"\b\w{3}\b").unwrap();
    let text = "!!!###@@@";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_multiple_matches() {
    let regex = Regex::new(r"\b\w{4}\b").unwrap();
    let text = "This test string has many four-letter words.";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 5);
    assert_eq!(mat.end, 9);
}

