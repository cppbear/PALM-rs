// Answer 0

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(b"/");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_star() {
    let input = Bytes::from_static(b"*");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_invalid_char() {
    let input = Bytes::from_static(b"a");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(b"");
    let _ = Uri::from_shared(input);
}

