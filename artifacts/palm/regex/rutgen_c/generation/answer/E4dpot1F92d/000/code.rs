// Answer 0

#[test]
fn test_replace_with_byte_string() {
    let re = Regex::new(r"[^01]+").unwrap();
    let result = re.replace(b"1078910", &b""[..]);
    assert_eq!(result, &b"1010"[..]);
}

#[test]
fn test_replace_with_capture_groups() {
    let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace(b"Springsteen, Bruce", |caps: &Captures| {
        let mut replacement = caps[2].to_owned();
        replacement.push(b' ');
        replacement.extend(&caps[1]);
        replacement
    });
    assert_eq!(result, &b"Bruce Springsteen"[..]);
}

#[test]
fn test_replace_with_named_capture_groups() {
    let re = Regex::new(r"(?P<last>[^,\s]+),\s+(?P<first>\S+)").unwrap();
    let result = re.replace(b"Springsteen, Bruce", &b"$first $last"[..]);
    assert_eq!(result, &b"Bruce Springsteen"[..]);
}

#[test]
fn test_replace_with_curly_braces() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace(b"deep fried", &b"${first}_$second"[..]);
    assert_eq!(result, &b"deep_fried"[..]);
}

#[test]
fn test_replace_no_matching() {
    let re = Regex::new(r"notfound").unwrap();
    let result = re.replace(b"this text contains no match", &b"replacement"[..]);
    assert_eq!(result, &b"this text contains no match"[..]);
}

