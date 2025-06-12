// Answer 0

#[test]
fn test_get_with_valid_capture_group() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures("abc123").unwrap();

    let capture_1 = caps.get(1);
    let capture_2 = caps.get(2);

    assert_eq!(capture_1.map(|m| m.as_str()), Some("123"));
    assert_eq!(capture_2.map(|m| m.as_str()), None);
}

#[test]
fn test_get_with_invalid_capture_group_index() {
    use regex::Regex;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures("abc123").unwrap();

    let capture_invalid = caps.get(3);
    
    assert_eq!(capture_invalid, None);
}

#[test]
fn test_get_with_no_matches() {
    use regex::Regex;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures("no match here").unwrap();

    let capture_1 = caps.get(1);
    let capture_2 = caps.get(2);

    assert_eq!(capture_1, None);
    assert_eq!(capture_2, None);
}

#[test]
#[should_panic]
fn test_get_on_empty_string() {
    use regex::Regex;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let _caps = re.captures("").unwrap(); // This will panic since there are no captures for an empty string
}

