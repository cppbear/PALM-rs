// Answer 0

#[test]
fn test_captures_with_named_groups() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    assert_eq!(&caps["title"], "Citizen Kane");
    assert_eq!(&caps["year"], "1941");
    assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
}

#[test]
fn test_captures_with_index() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'The Matrix' (1999).";
    let caps = re.captures(text).unwrap();
    assert_eq!(caps.get(1).unwrap().as_str(), "The Matrix");
    assert_eq!(caps.get(2).unwrap().as_str(), "1999");
    assert_eq!(caps.get(0).unwrap().as_str(), "'The Matrix' (1999)");
}

#[test]
fn test_captures_with_no_match() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "No matching movie here.";
    let caps = re.captures(text);
    assert!(caps.is_none());
}

#[test]
fn test_captures_empty_string() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "";
    let caps = re.captures(text);
    assert!(caps.is_none());
}

#[test]
fn test_captures_incomplete_data() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Only Title' (2021)";
    let caps = re.captures(text).unwrap();
    assert_eq!(&caps["title"], "Only Title");
    assert_eq!(&caps["year"], "2021");
    assert_eq!(caps.get(0).unwrap().as_str(), "'Only Title' (2021)");
}

