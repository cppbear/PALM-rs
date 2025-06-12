// Answer 0

#[test]
fn test_split_with_basic_spaces() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"a b \t  c\td    e").collect();
    assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..]]);
}

#[test]
fn test_split_with_leading_and_trailing_spaces() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"    a b c    ").collect();
    assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..]]);
}

#[test]
fn test_split_with_only_spaces() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"     ").collect();
    assert_eq!(fields, Vec::<&[u8]>::new());
}

#[test]
fn test_split_with_no_match() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"abc").collect();
    assert_eq!(fields, vec![&b"abc"[..]]);
}

#[test]
fn test_split_with_consecutive_delimiters() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"a   b\tc").collect();
    assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..]]);
}

#[test]
fn test_split_with_empty_input() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&[u8]> = re.split(b"").collect();
    assert_eq!(fields, Vec::<&[u8]>::new());
}

