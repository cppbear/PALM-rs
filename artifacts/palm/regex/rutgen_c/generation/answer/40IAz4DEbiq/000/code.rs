// Answer 0

#[test]
fn test_find_iter_empty_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter("").collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_no_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "short word";
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_single_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "Retroactively relinquishing";
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].start(), 0);
    assert_eq!(matches[0].end(), 13);
}

#[test]
fn test_find_iter_multiple_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "Retroactively relinquishing remunerations is reprehensible.";
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0].start(), 0);
    assert_eq!(matches[0].end(), 13);
    assert_eq!(matches[1].start(), 14);
    assert_eq!(matches[1].end(), 30);
    assert_eq!(matches[2].start(), 31);
    assert_eq!(matches[2].end(), 45);
}

