// Answer 0

#[test]
fn test_find_single_match() {
    let text = b"I categorize triskaidekaphobia.";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let mat = regex.find(text).unwrap();
    assert_eq!((mat.start(), mat.end()), (2, 15));
}

#[test]
fn test_find_no_match() {
    let text = b"short words here.";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_multiple_matches() {
    let text = b"Extraordinarily fine work at the triskaidekaphobia.";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let mat = regex.find(text).unwrap();
    assert_eq!((mat.start(), mat.end()), (0, 13));
}

#[test]
fn test_find_first_word_with_boundary() {
    let text = b"   equivocate    ";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_at_start_of_string() {
    let text = b"triskaidekaphobia is a long word.";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let mat = regex.find(text).unwrap();
    assert_eq!((mat.start(), mat.end()), (0, 13));
}

#[test]
fn test_find_empty_input() {
    let text: &[u8] = b"";
    let regex = regex::bytes::Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

