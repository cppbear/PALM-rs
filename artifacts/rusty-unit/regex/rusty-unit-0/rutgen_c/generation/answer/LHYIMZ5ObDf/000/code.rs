// Answer 0

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"").collect();
    assert_eq!(fields, vec![b""; 1]);
}

#[test]
fn test_split_no_matches() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"abc").collect();
    assert_eq!(fields, vec![b"abc"]);
}

#[test]
fn test_split_multiple_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"a b \t  c\td    e").collect();
    assert_eq!(fields, vec![b"a", b"b", b"c", b"d", b"e"]);
}

#[test]
fn test_split_leading_trailing_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"  a b \t  c\td    ").collect();
    assert_eq!(fields, vec![b"a", b"b", b"c", b"d"]);
}

#[test]
fn test_split_consecutive_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"a\t\tb c").collect();
    assert_eq!(fields, vec![b"a", b"b", b"c"]);
}

