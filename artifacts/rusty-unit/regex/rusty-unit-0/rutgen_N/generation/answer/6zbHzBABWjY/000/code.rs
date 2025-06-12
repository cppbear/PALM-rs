// Answer 0

#[test]
fn test_capture_group_present() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures("abc123").unwrap();

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());
    
    assert_eq!(text1, "123");
    assert_eq!(text2, "");
}

#[test]
fn test_capture_group_not_present() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures("abc").unwrap();

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());

    assert_eq!(text1, ""); // Group 1 should not be present
    assert_eq!(text2, ""); // Group 2 should not be present
}

#[test]
fn test_capture_group_only_digits() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures("xyz456").unwrap();

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());

    assert_eq!(text1, "456"); // Group 1 should be "456"
    assert_eq!(text2, ""); // Group 2 should not be present
}

#[test]
fn test_capture_group_only_uppercase() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures("abcXYZ").unwrap();

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());

    assert_eq!(text1, ""); // Group 1 should not be present
    assert_eq!(text2, "XYZ"); // Group 2 should be "XYZ"
}

#[test]
fn test_invalid_capture_group_index() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures("abc123").unwrap();

    let text = caps.get(3); // Index out of bounds, should be None
    assert_eq!(text, None);
}

