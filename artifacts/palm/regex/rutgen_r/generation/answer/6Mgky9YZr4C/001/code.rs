// Answer 0

#[test]
fn test_get_valid_capture_group() {
    use regex::bytes::Regex;
    use regex::bytes::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures(b"abc123").unwrap();

    let text1 = caps.get(1).map_or(&b""[..], |m| m.as_bytes());
    assert_eq!(text1, &b"123"[..]);
}

#[test]
fn test_get_invalid_capture_group() {
    use regex::bytes::Regex;
    use regex::bytes::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures(b"abc123").unwrap();

    let text2 = caps.get(2).map_or(&b""[..], |m| m.as_bytes());
    assert_eq!(text2, &b""[..]);
}

#[test]
fn test_get_capture_group_out_of_bounds() {
    use regex::bytes::Regex;
    use regex::bytes::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures(b"abc123").unwrap();

    let text3 = caps.get(3); // This group does not exist
    assert_eq!(text3, None);
}

#[should_panic]
#[test]
fn test_get_negative_index() {
    use regex::bytes::Regex;
    use regex::bytes::Captures;

    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps: Captures = re.captures(b"abc123").unwrap();

    caps.get(usize::MAX); // Should panic due to out of bounds
}

