// Answer 0

#[test]
fn test_captures_valid_pattern_with_two_groups() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
}

#[test]
fn test_captures_named_groups() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
}

#[test]
fn test_captures_empty_string() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "";
    let caps = re.captures(text);
}

#[test]
fn test_captures_no_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "No matching text here.";
    let caps = re.captures(text);
}

#[test]
fn test_captures_long_string() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "A very long string that is definitely longer than the usual movie titles...";
    let caps = re.captures(text);
}

#[test]
fn test_captures_with_too_many_groups() {
    let re = Regex::new(r"(?P<first>.+)(?P<second>.+)(?P<third>.+)(?P<fourth>.+)").unwrap();
    let text = "some random text";
    let caps = re.captures(text);
}

#[test]
fn test_captures_accessing_invalid_index() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    
    // Accessing out-of-bounds index
    let _ = caps.get(11);
}

#[test]
#[should_panic]
fn test_captures_accessing_negative_index() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    
    // Accessing invalid negative index
    let _ = &caps[-1];
}

