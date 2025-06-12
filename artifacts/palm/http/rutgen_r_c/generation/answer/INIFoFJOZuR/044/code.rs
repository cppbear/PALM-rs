// Answer 0

#[test]
fn test_from_bytes_post() {
    let input = b"POST";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::POST)));
}

#[test]
fn test_from_bytes_head() {
    let input = b"HEAD";
    let result = Method::from_bytes(input);
    assert_eq!(result, Ok(Method(Method::HEAD)));
}

