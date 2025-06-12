// Answer 0

#[test]
fn test_replace_basic() {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace(b"123 and 456", &b"$1"[..]);
}

#[test]
fn test_replace_no_match() {
    let re = Regex::new(r"abc").unwrap();
    let result = re.replace(b"123 and 456", &b"replacement"[..]);
}

#[test]
fn test_replace_multiple_matches() {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace(b"1a2b3c", &b"X$1X"[..]);
}

#[test]
fn test_replace_empty_string() {
    let re = Regex::new(r"pattern").unwrap();
    let result = re.replace(b"", &b"replacement"[..]);
}

#[test]
fn test_replace_with_named_groups() {
    let re = Regex::new(r"(?P<first>\w+)\s(?P<last>\w+)").unwrap();
    let result = re.replace(b"Jane Doe", &b"$last, $first"[..]);
}

#[test]
fn test_replace_with_no_expand() {
    use regex::bytes::NoExpand;
    let re = Regex::new(r"(\w+)").unwrap();
    let result = re.replace(b"hello world", NoExpand(b"$1"));
}

#[test]
fn test_replace_limit() {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replacen(b"1 2 3", 2, &b"$1!"[..]);
}

#[test]
fn test_replace_panic_invalid_capture() {
    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let result = re.replace(b"word", &b"$nonexistent"[..]);
}

#[test]
fn test_replace_with_function() {
    let re = Regex::new(r"(\w+)").unwrap();
    let result = re.replace(b"hello", |caps: &Captures| {
        let mut replacement = caps[0].to_owned();
        replacement.extend_from_slice(b" world");
        replacement
    });
}

#[test]
fn test_replace_large_inputs() {
    let re = Regex::new(r"\d+").unwrap();
    let input: Vec<u8> = (0..10000).map(|i| i.to_string()).collect::<Vec<_>>().join(b" ").to_vec();
    let result = re.replace(&input, &b"number"[..]);
}

