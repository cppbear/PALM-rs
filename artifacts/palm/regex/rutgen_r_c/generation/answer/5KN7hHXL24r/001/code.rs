// Answer 0

#[test]
fn test_captures_iter_with_valid_input() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    let mut captures_iter = re.captures_iter(text);

    assert_eq!(captures_iter.count(), 3);

    let first = captures_iter.next().unwrap();
    assert_eq!(std::str::from_utf8(&first["title"]).unwrap(), "Citizen Kane");
    assert_eq!(std::str::from_utf8(&first["year"]).unwrap(), "1941");

    let second = captures_iter.next().unwrap();
    assert_eq!(std::str::from_utf8(&second["title"]).unwrap(), "The Wizard of Oz");
    assert_eq!(std::str::from_utf8(&second["year"]).unwrap(), "1939");

    let third = captures_iter.next().unwrap();
    assert_eq!(std::str::from_utf8(&third["title"]).unwrap(), "M");
    assert_eq!(std::str::from_utf8(&third["year"]).unwrap(), "1931");
}

#[test]
fn test_captures_iter_with_no_matches() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"No matches here.";
    let captures_iter = re.captures_iter(text);

    assert_eq!(captures_iter.count(), 0);
}

#[test]
#[should_panic]
fn test_captures_iter_with_incomplete_data() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Incomplete Title' (2021), 'Another Incomplete' (";
    let mut captures_iter = re.captures_iter(text);

    // Attempt to access captures that don't exist should panic
    let invalid_capture = captures_iter.next().unwrap();
    let _ = std::str::from_utf8(&invalid_capture["year"]).unwrap();
}

#[test]
fn test_captures_iter_with_empty_input() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"";
    let captures_iter = re.captures_iter(text);

    assert_eq!(captures_iter.count(), 0);
}

#[test]
fn test_captures_iter_with_special_characters() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Special & Unique' (2020), 'Another One!' (2021)";
    let mut captures_iter = re.captures_iter(text);

    assert_eq!(captures_iter.count(), 2);

    let first = captures_iter.next().unwrap();
    assert_eq!(std::str::from_utf8(&first["title"]).unwrap(), "Special & Unique");
    assert_eq!(std::str::from_utf8(&first["year"]).unwrap(), "2020");

    let second = captures_iter.next().unwrap();
    assert_eq!(std::str::from_utf8(&second["title"]).unwrap(), "Another One!");
    assert_eq!(std::str::from_utf8(&second["year"]).unwrap(), "2021");
}

