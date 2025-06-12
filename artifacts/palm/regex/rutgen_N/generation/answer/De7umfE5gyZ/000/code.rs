// Answer 0

#[test]
fn test_find_iter_no_matches() {
    use regex::bytes::Regex;

    let text = b"Hello World!";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_one_match() {
    use regex::bytes::Regex;

    let text = b"Hello there everyone.";
    let regex = Regex::new(r"\b\w{7}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].start(), 6);
    assert_eq!(matches[0].end(), 13);
}

#[test]
fn test_find_iter_multiple_matches() {
    use regex::bytes::Regex;

    let text = b"This is a test sentence with multiple seven words.";
    let regex = Regex::new(r"\b\w{7}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    
    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0].start(), 10);
    assert_eq!(matches[1].start(), 28);
    assert_eq!(matches[2].start(), 36);
}

#[test]
fn test_find_iter_edge_case_empty_input() {
    use regex::bytes::Regex;

    let text: &[u8] = b"";
    let regex = Regex::new(r"\b\w{1}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    
    assert!(matches.is_empty());
}

