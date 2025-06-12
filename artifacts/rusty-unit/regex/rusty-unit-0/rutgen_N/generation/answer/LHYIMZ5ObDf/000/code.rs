// Answer 0

#[test]
fn test_split_empty_string() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"").collect();
    assert_eq!(fields, vec![&b""[..]]);
}

#[test]
fn test_split_no_delimiter() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"abc").collect();
    assert_eq!(fields, vec![&b"abc"[..]]);
}

#[test]
fn test_split_leading_spaces() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"   abc").collect();
    assert_eq!(fields, vec![&b""[..], &b"abc"[..]]);
}

#[test]
fn test_split_trailing_spaces() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"abc   ").collect();
    assert_eq!(fields, vec![&b"abc"[..], &b""[..]]);
}

#[test]
fn test_split_multiple_delimiters() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b" a  b \t   c  d e ").collect();
    assert_eq!(fields, vec![
        &b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..],
    ]);
}

#[test]
fn test_split_only_delimiters() {
    extern crate regex;
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\s+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"   ").collect();
    assert_eq!(fields, vec![&b""[..]]);
}

