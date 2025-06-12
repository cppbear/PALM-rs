// Answer 0

#[test]
fn test_find_iter_with_no_matches() {
    use regex::Regex;

    let text = "No matches here.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_with_one_match() {
    use regex::Regex;

    let text = "This is an extraordinarily long word.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].start(), 34);
    assert_eq!(matches[0].end(), 47);
}

#[test]
fn test_find_iter_with_multiple_matches() {
    use regex::Regex;

    let text = "Using extraordinarily and uncharacteristically";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].start(), 5);
    assert_eq!(matches[0].end(), 18);
    assert_eq!(matches[1].start(), 22);
    assert_eq!(matches[1].end(), 39);
}

#[test]
fn test_find_iter_with_boundary_conditions() {
    use regex::Regex;

    let text = "abcdefghijklmno pqrstuvwxyz";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_find_iter_empty_string() {
    use regex::Regex;

    let text = "";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<_> = regex.find_iter(text).collect();
    assert!(matches.is_empty());
}

