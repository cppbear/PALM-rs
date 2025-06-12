// Answer 0

#[test]
fn test_captures_with_simple_match() {
    use regex::Regex;
    
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
fn test_captures_with_named_groups() {
    use regex::Regex;
    
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
fn test_captures_with_no_matching_text() {
    use regex::Regex;
    
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "This text does not contain any movies.";
    let caps = re.captures(text);
    
    assert!(caps.is_none());
}

