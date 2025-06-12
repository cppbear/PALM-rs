// Answer 0

#[test]
fn test_replace_with_no_match() {
    let re = Regex::new(r"notfound").unwrap();
    let result = re.replace(b"test string", &b"replacement"[..]);
    assert_eq!(result, &b"test string"[..]);
}

#[test]
fn test_replace_with_empty_string() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace(b"12345", &b""[..]);
    assert_eq!(result, &b""[..]);
}

#[test]
fn test_replace_with_numeric_capture_group() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"Alice Smith", &b"$second $first"[..]);
    assert_eq!(result, &b"Smith Alice"[..]);
}

#[test]
fn test_replace_with_invalid_capture_group() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"Alice Smith", &b"$third $first"[..]);
    assert_eq!(result, &b" Alice"[..]);
}

#[test]
fn test_replace_with_no_expand() {
    use regex::bytes::NoExpand;

    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"Alice Smith", NoExpand(b"$second $first"));
    assert_eq!(result, &b"$second $first"[..]);
}

#[test]
fn test_replace_check_groups() {
    let re = Regex::new(r"(?P<group1>\w+)(?P<group2>\d+)").unwrap();
    let result = re.replace(b"Test123", |caps: &Captures| {
        format!("{} and {}", &caps["group1"], &caps["group2"]).into_bytes()
    });
    assert_eq!(result, &b"Test and 123"[..]);
}

#[test]
#[should_panic]
fn test_replace_with_invalid_regex() {
    let _ = Regex::new(r"(?P<invalid_capture_group[a-z]+)"); // invalid regex should panic
}

