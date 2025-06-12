// Answer 0

#[test]
fn test_captures_valid_case() {
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
fn test_captures_named_groups() {
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
fn test_captures_no_match() {
    use regex::Regex;

    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "This text has no movie year.";
    let caps = re.captures(text);
    assert!(caps.is_none());
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_captures_panic_invalid_index() {
    use regex::Regex;

    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    let _panic_val = &caps[3];
}

#[test]
#[should_panic(expected = "group `unknown` not found")]
fn test_captures_panic_invalid_named_group() {
    use regex::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    let _panic_val = &caps["unknown"];
}

