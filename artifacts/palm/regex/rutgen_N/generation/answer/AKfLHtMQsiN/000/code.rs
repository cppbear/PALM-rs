// Answer 0

#[test]
fn test_read_captures_at_some() {
    struct RegexWrapper(regex::Regex);
    
    let re = RegexWrapper(regex::Regex::new(r"\d+").unwrap());
    let mut locs = regex::Locations::new();
    let text = "There are 42 apples.";

    let result = re.read_captures_at(&mut locs, text, 0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (10, 12)); // Assuming "42" is located at byte indices 10 to 12
}

#[test]
fn test_read_captures_at_none() {
    struct RegexWrapper(regex::Regex);
    
    let re = RegexWrapper(regex::Regex::new(r"\w+").unwrap());
    let mut locs = regex::Locations::new();
    let text = "!!!"; // No words

    let result = re.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_edge_case() {
    struct RegexWrapper(regex::Regex);
    
    let re = RegexWrapper(regex::Regex::new(r"^$").unwrap()); // Match empty string
    let mut locs = regex::Locations::new();
    let text = ""; // Empty text

    let result = re.read_captures_at(&mut locs, text, 0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (0, 0)); // Should match at the start
}

