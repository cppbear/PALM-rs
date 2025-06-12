// Answer 0

#[test]
fn test_splitn_with_non_matching_text() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"HelloWorld", 2).collect();
    assert_eq!(fields, vec![&b"HelloWorld"[..]]);
}

#[test]
fn test_splitn_with_limit_zero() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hello, World!", 0).collect();
    assert_eq!(fields, vec![]);
}

#[test]
fn test_splitn_with_limit_one() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hello, World!", 1).collect();
    assert_eq!(fields, vec![&b"Hello"[..]]);
}

#[test]
fn test_splitn_with_limit_three() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 3).collect();
    assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
}

#[test]
fn test_splitn_with_matching_text() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"What is this? A test!", 3).collect();
    assert_eq!(fields, vec![&b"What"[..], &b"is"[..], &b"this?"[..]]);
}

