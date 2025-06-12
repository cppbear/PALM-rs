// Answer 0

#[test]
fn test_captures_iter_valid_input() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    let mut captures = re.captures_iter(text);
    let first = captures.next().unwrap();
    assert_eq!(&first["title"], "Citizen Kane");
    assert_eq!(&first["year"], "1941");
    let second = captures.next().unwrap();
    assert_eq!(&second["title"], "The Wizard of Oz");
    assert_eq!(&second["year"], "1939");
    let third = captures.next().unwrap();
    assert_eq!(&third["title"], "M");
    assert_eq!(&third["year"], "1931");
    assert!(captures.next().is_none());
}

#[test]
fn test_captures_iter_no_matches() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "No movies here!";
    let captures = re.captures_iter(text);
    assert!(captures.next().is_none());
}

#[test]
#[should_panic]
fn test_captures_iter_incomplete_year() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Incomplete Movie' (20).";
    let _captures = re.captures_iter(text); // This should panic due to incomplete match.
}

