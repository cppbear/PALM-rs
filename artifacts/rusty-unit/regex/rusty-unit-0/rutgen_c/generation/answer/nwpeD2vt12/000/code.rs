// Answer 0

#[test]
fn test_find_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "I categorically deny having triskaidekaphobia.";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 2);
    assert_eq!(mat.end, 15);
}

#[test]
fn test_find_no_match() {
    let regex = Regex::new(r"\b\w{20}\b").unwrap();
    let text = "Short words only.";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_empty_string() {
    let regex = Regex::new(r"\b\w{1}\b").unwrap();
    let text = "";
    let mat = regex.find(text);
    assert!(mat.is_none());
}

#[test]
fn test_find_with_leading_trailing_spaces() {
    let regex = Regex::new(r"\b\w{6}\b").unwrap();
    let text = "    travel    ";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 4);
    assert_eq!(mat.end, 10);
}

#[test]
fn test_find_multiple_matches() {
    let regex = Regex::new(r"\b\w{5}\b").unwrap();
    let text = "There are ample apples in the basket.";
    let mat = regex.find(text).unwrap();
    assert_eq!(mat.start, 6);
    assert_eq!(mat.end, 11);
}

