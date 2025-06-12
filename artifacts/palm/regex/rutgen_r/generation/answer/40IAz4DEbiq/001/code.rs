// Answer 0

#[test]
fn test_find_iter_no_matches() {
    let text = "Short text.";
    let regex = regex::Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_exact_match() {
    let text = "This is an extraordinary event.";
    let regex = regex::Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].start(), 10);
    assert_eq!(matches[0].end(), 23);
}

#[test]
fn test_find_iter_multiple_matches() {
    let text = "Unquestionably adversarially individuals.";
    let regex = regex::Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].start(), 0);
    assert_eq!(matches[0].end(), 16);
    assert_eq!(matches[1].start(), 17);
    assert_eq!(matches[1].end(), 32);
}

#[test]
fn test_find_iter_empty_string() {
    let text = "";
    let regex = regex::Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_only_whitespace() {
    let text = "         ";
    let regex = regex::Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
#[should_panic]
fn test_find_iter_invalid_regex() {
    let text = "Invalid regex testing.";
    let _regex = regex::Regex::new(r"\b\w{#}").unwrap(); // This regex is invalid
}

