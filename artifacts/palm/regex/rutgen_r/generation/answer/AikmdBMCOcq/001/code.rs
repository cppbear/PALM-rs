// Answer 0

#[test]
fn test_splitn_with_limit_greater_than_words() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 5).collect();
    assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are"[..], &b"you?"[..]]);
}

#[test]
fn test_splitn_with_exact_limit() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 3).collect();
    assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
}

#[test]
fn test_splitn_with_limit_zero() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 0).collect();
    assert_eq!(fields, Vec::<&[u8]>::new());
}

#[test]
fn test_splitn_with_multiple_delimiters() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[\s!]+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 2).collect();
    assert_eq!(fields, vec![&b"Hey"[..], &b"How are you?"[..]]);
}

#[test]
fn test_splitn_with_empty_string() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"", 2).collect();
    assert_eq!(fields, vec![&b""[..]]);
}

#[test]
fn test_splitn_with_single_word() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hello", 1).collect();
    assert_eq!(fields, vec![&b"Hello"[..]]);
}

#[test]
fn test_splitn_with_trailing_delimiter() {
    use regex::bytes::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hello, World! ", 3).collect();
    assert_eq!(fields, vec![&b"Hello"[..], &b"World"[..], &b""[..]]);
}

