// Answer 0

#[test]
fn test_replace_all_no_matches() {
    let regex = Regex::new(r"\d+").unwrap();
    let text = b"abc";
    let result = regex.replace_all(text, b"XYZ");
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replace_all_single_match() {
    let regex = Regex::new(r"abc").unwrap();
    let text = b"abc def";
    let result = regex.replace_all(text, b"XYZ");
    assert_eq!(result, Cow::Owned(b"XYZ def".to_vec()));
}

#[test]
fn test_replace_all_multiple_matches() {
    let regex = Regex::new(r"\d+").unwrap();
    let text = b"123 and 456";
    let result = regex.replace_all(text, b"NUMBER");
    assert_eq!(result, Cow::Owned(b"NUMBER and NUMBER".to_vec()));
}

#[test]
fn test_replace_all_with_empty_replacement() {
    let regex = Regex::new(r"\s+").unwrap();
    let text = b"abc   def";
    let result = regex.replace_all(text, b"");
    assert_eq!(result, Cow::Owned(b"abcdef".to_vec()));
}

#[test]
fn test_replace_all_edge_cases() {
    let regex = Regex::new(r"(\w+)").unwrap();
    let text = b"word";
    let result = regex.replace_all(text, |caps: &Captures| {
        let word = caps.get(0).unwrap().as_bytes();
        Cow::Owned([word, b"XYZ"].concat())
    });
    assert_eq!(result, Cow::Owned(b"wordXYZ".to_vec()));
}

