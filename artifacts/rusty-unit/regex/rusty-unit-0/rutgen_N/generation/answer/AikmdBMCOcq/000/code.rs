// Answer 0

#[test]
fn test_splitn_limit_non_zero() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hey! How are you?", 3).collect();
    assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
}

#[test]
fn test_splitn_limit_zero() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Hello, world!", 0).collect();
    assert_eq!(fields, Vec::<&[u8]>::new());
}

#[test]
fn test_splitn_limit_greater_than_splits() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"Rust is great!", 5).collect();
    assert_eq!(fields, vec![&b"Rust"[..], &b"is"[..], &b"great!"[..]]);
}

#[test]
fn test_splitn_limit_exceeding() {
    use regex::bytes::Regex;
    
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&[u8]> = re.splitn(b"One! Two! Three! Four!", 10).collect();
    assert_eq!(fields, vec![&b"One"[..], &b"Two"[..], &b"Three"[..], &b"Four!"[..]]);
}

