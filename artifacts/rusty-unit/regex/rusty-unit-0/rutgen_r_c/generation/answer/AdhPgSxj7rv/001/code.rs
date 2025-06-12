// Answer 0

#[test]
fn test_captures_valid_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    
    assert_eq!(caps.get(1).unwrap().as_str(), "Citizen Kane");
    assert_eq!(caps.get(2).unwrap().as_str(), "1941");
    assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    assert_eq!(&caps[1], "Citizen Kane");
    assert_eq!(&caps[2], "1941");
    assert_eq!(&caps[0], "'Citizen Kane' (1941)");
}

#[test]
fn test_captures_no_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "No matches here.";
    let caps = re.captures(text);
    
    assert!(caps.is_none());
}

#[test]
fn test_captures_with_named_groups() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    
    assert_eq!(&caps["title"], "Citizen Kane");
    assert_eq!(&caps["year"], "1941");
    assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    assert_eq!(&caps["title"], "Citizen Kane");
    assert_eq!(&caps["year"], "1941");
    assert_eq!(&caps[0], "'Citizen Kane' (1941)");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_captures_panic_out_of_bounds() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    
    // This should panic since there is no third capture group
    let _ = &caps[2]; // Valid
    let _ = &caps[3]; // Panic
}

#[test]
fn test_captures_invalid_regex() {
    let re = Regex::new(r"[a-z").unwrap_err(); // Invalid regex should return error
    assert!(re.is_err());
}

