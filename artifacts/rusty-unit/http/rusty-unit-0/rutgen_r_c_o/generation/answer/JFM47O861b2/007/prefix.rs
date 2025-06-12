// Answer 0

#[test]
fn test_from_shared_star() {
    let input = Bytes::from_static(b"*");
    let result = Uri::from_shared(input);
}

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(b"");
    let result = Uri::from_shared(input);
}

#[test]
fn test_from_shared_slash() {
    let input = Bytes::from_static(b"/");
    let result = Uri::from_shared(input);
}

#[test]
fn test_from_shared_non_reserved_character() {
    let input = Bytes::from_static(b"abc");
    let result = Uri::from_shared(input);
}

