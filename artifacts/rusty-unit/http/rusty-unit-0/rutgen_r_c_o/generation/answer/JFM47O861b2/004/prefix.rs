// Answer 0

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(&[]);
    let _ = Uri::from_shared(input);
}

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
fn test_from_shared_short_authority() {
    let input = Bytes::from_static(b"example.com");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_slash_prefix() {
    let input = Bytes::from_static(b"/path/to/resource");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_max_length() {
    let input = Bytes::from(vec![b'a'; 65534]);
    let _ = Uri::from_shared(input);
}

