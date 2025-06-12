// Answer 0

#[test]
fn test_split_basic() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"a b \t  c\td    e").collect();
    assert_eq!(result, vec![&b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..]]);
}

#[test]
fn test_split_multiple_spaces() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"   a   b   c   ").collect();
    assert_eq!(result, vec![&b""[..], &b"a"[..], &b""[..], &b"b"[..], &b""[..], &b"c"[..], &b""[..]]);
}

#[test]
fn test_split_leading_trailing_whitespace() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"   \t  a  \t").collect();
    assert_eq!(result, vec![&b""[..], &b"a"[..], &b""[..]]);
}

#[test]
fn test_split_no_matches() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"abcd").collect();
    assert_eq!(result, vec![&b"abcd"[..]]);
}

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"").collect();
    assert_eq!(result, vec![&b""[..]]);
}

#[test]
fn test_split_only_spaces() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b" \t ").collect();
    assert_eq!(result, vec![&b""[..], &b""[..]]);
}

#[test]
fn test_split_consecutive_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let result: Vec<&[u8]> = re.split(b"ab  cd\t ef").collect();
    assert_eq!(result, vec![&b"ab"[..], &b"cd"[..], &b"ef"[..]]);
}

