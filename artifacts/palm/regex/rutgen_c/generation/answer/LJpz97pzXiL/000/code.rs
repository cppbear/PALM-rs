// Answer 0

#[test]
fn test_captures_success() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    assert_eq!(&caps[1], &b"Citizen Kane"[..]);
    assert_eq!(&caps[2], &b"1941"[..]);
    assert_eq!(&caps[0], &b"'Citizen Kane' (1941)"[..]);
}

#[test]
fn test_captures_named_groups() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
    let caps = re.captures(text).unwrap();
    assert_eq!(&caps["title"], &b"Citizen Kane"[..]);
    assert_eq!(&caps["year"], &b"1941"[..]);
    assert_eq!(&caps[0], &b"'Citizen Kane' (1941)"[..]);
}

#[test]
fn test_captures_no_match() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = b"No movie title here.";
    let caps = re.captures(text);
    assert!(caps.is_none());
}

