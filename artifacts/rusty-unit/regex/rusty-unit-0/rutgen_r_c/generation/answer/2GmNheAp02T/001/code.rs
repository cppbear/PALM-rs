// Answer 0

#[test]
fn test_replace_with_empty_text() {
    let re = Regex::new(r"abc").unwrap();
    let result = re.replace("", "replacement");
    assert_eq!(result, Cow::Borrowed(""));
}

#[test]
fn test_replace_no_match() {
    let re = Regex::new(r"abc").unwrap();
    let result = re.replace("xyz", "replacement");
    assert_eq!(result, Cow::Borrowed("xyz"));
}

#[test]
fn test_replace_single_match() {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace("abc123", "$1A");
    assert_eq!(result, Cow::Owned("abc123A".to_string()));
}

#[test]
fn test_replace_multiple_matches() {
    let re = Regex::new(r"(\w+)").unwrap();
    let result = re.replace("hello world", "$1!");
    assert_eq!(result, Cow::Owned("hello!world!".to_string()));
}

#[test]
fn test_replace_with_no_expand() {
    use regex::NoExpand;

    let re = Regex::new(r"(\w+)").unwrap();
    let result = re.replace("hello world", NoExpand("literal"));
    assert_eq!(result, Cow::Owned("literal".to_string()));
}

#[test]
fn test_replace_with_named_capture_groups() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace("deep fried", "$second_$first");
    assert_eq!(result, Cow::Owned("fried_deep".to_string()));
}

#[test]
fn test_replace_with_invalid_capture_group() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace("deep fried", "$third");
    assert_eq!(result, Cow::Owned("deep fried".to_string()));
}

#[test]
fn test_replace_with_curly_braces() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace("deep fried", "${first}_$second");
    assert_eq!(result, Cow::Owned("deep_fried".to_string()));
}

