// Answer 0

#[test]
fn test_find_with_no_matches() {
    use regex::Regex;

    let text = "No match here.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_with_exact_match() {
    use regex::Regex;

    let text = "I categorically deny having triskaidekaphobia.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let result = regex.find(text).unwrap();
    assert_eq!(result.start(), 2);
    assert_eq!(result.end(), 15);
}

#[test]
fn test_find_with_multiple_matches() {
    use regex::Regex;

    let text = "The quick brown fox jumps over the lazy dog.";
    let regex = Regex::new(r"\b\w{5}\b").unwrap();
    let result = regex.find(text).unwrap();
    assert_eq!(result.start(), 4);
    assert_eq!(result.end(), 9);
}

#[test]
fn test_find_with_boundary_conditions() {
    use regex::Regex;

    let text = "abcd efghijklmn opqrstu vwxyz.";
    let regex = Regex::new(r"\b\w{12}\b").unwrap();
    let result = regex.find(text);
    assert!(result.is_none());

    let text_no_space = "abcdefghijklm";
    let regex_matching_boundary = Regex::new(r"\b\w{13}\b").unwrap();
    let result_boundary = regex_matching_boundary.find(text_no_space);
    assert!(result_boundary.is_none());
}

