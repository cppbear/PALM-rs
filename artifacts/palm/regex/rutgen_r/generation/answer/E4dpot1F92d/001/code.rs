// Answer 0

#[test]
fn test_replace_with_empty_string() {
    let re = regex::bytes::Regex::new(r"[^01]+").unwrap();
    let result = re.replace(b"123456", &b""[..]);
    assert_eq!(result, &b"101010"[..]);
}

#[test]
fn test_replace_no_match() {
    let re = regex::bytes::Regex::new(r"^[AB]+$").unwrap();
    let result = re.replace(b"123456", &b"replaced"[..]);
    assert_eq!(result, &b"123456"[..]);
}

#[test]
fn test_replace_with_captures() {
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"hello world", |caps: &regex::bytes::Captures| {
        let first = &caps["first"];
        let second = &caps["second"];
        [second.to_vec(), b" ".to_vec(), first.to_vec()].concat().into_boxed_slice()
    });
    assert_eq!(result, &b"world hello"[..]);
}

#[test]
fn test_replace_with_named_capture() {
    let re = regex::bytes::Regex::new(r"(?P<last>\w+),\s+(?P<first>\w+)").unwrap();
    let result = re.replace(b"doe, john", &b"$first $last"[..]);
    assert_eq!(result, &b"john doe"[..]);
}

#[test]
fn test_replace_with_curly_braces() {
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"deep fried", &b"${first}_$second"[..]);
    assert_eq!(result, &b"deep_fried"[..]);
}

#[test]
fn test_replace_with_noexpand() {
    use regex::bytes::NoExpand;
    let re = regex::bytes::Regex::new(r"(?P<last>\w+),\s+(\w+)").unwrap();
    let result = re.replace(b"smith, john", NoExpand(b"$2 $last"));
    assert_eq!(result, &b"$2 $last"[..]);
}

#[should_panic]
fn test_replace_with_invalid_capture_group() {
    let re = regex::bytes::Regex::new(r"(?P<valid>\w+)").unwrap();
    let _result = re.replace(b"hello", &b"$invalid"[..]);
}

