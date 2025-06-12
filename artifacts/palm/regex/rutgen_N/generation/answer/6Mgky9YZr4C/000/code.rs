// Answer 0

#[test]
fn test_get_capture_group_present() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    
    let match1 = caps.get(1);
    let match2 = caps.get(2);
    
    assert_eq!(match1.map(|m| m.as_bytes()), Some(&b"123"[..]));
    assert_eq!(match2.map(|m| m.as_bytes()), Some(&b""[..]));
}

#[test]
fn test_get_non_existent_capture_group() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    
    let match3 = caps.get(3); // Non-existent capture group
    assert_eq!(match3, None);
}

#[test]
fn test_get_capture_group_with_no_match() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc").unwrap();
    
    let match1 = caps.get(1);
    let match2 = caps.get(2);
    
    assert_eq!(match1.map(|m| m.as_bytes()), None);
    assert_eq!(match2.map(|m| m.as_bytes()), None);
}

#[test]
fn test_get_capture_group_empty_input() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"").unwrap();
    
    let match1 = caps.get(1);
    let match2 = caps.get(2);
    
    assert_eq!(match1.map(|m| m.as_bytes()), None);
    assert_eq!(match2.map(|m| m.as_bytes()), None);
}

